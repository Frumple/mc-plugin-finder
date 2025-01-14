use crate::HttpServer;
use crate::modrinth::ModrinthClient;
use mc_plugin_finder::database::modrinth::project::{ModrinthProject, upsert_modrinth_project};
use mc_plugin_finder::database::source_repository::{SourceRepository, extract_source_repository_from_url};

use anyhow::Result;
use deadpool_postgres::Pool;
use futures::future;
use futures::stream::TryStreamExt;
use page_turner::prelude::*;
use regex::Regex;
use reqwest::StatusCode;
use serde::{Serialize, Deserialize};
use time::OffsetDateTime;
use std::fmt::Debug;
use std::sync::{Arc, LazyLock};
use std::sync::atomic::{AtomicU32, Ordering};
use thiserror::Error;
use time::format_description::well_known::Rfc3339;
use tracing::{info, warn, instrument};

const MODRINTH_PROJECTS_REQUESTS_AHEAD: usize = 2;
const MODRINTH_PROJECTS_CONCURRENT_FUTURES: usize = 10;

#[derive(Clone, Debug, Serialize)]
struct SearchModrinthProjectsRequest {
    facets: String,
    limit: u32,
    offset: u32,
    index: String
}

impl SearchModrinthProjectsRequest {
    fn create_request() -> Self {
        Self {
            facets: "[[\"project_type:plugin\"]]".to_string(),
            limit: 100,
            offset: 0,
            index: "updated".to_string()
        }
    }
}

impl RequestAhead for SearchModrinthProjectsRequest {
    fn next_request(&self) -> Self {
        Self {
            facets: "[[\"project_type:plugin\"]]".to_string(),
            limit: self.limit,
            offset: self.offset + self.limit,
            index: self.index.clone()
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
struct SearchModrinthProjectsResponse {
    hits: Vec<IncomingModrinthProject>,
    offset: u32,
    limit: u32,
    total_hits: u32
}

impl SearchModrinthProjectsResponse {
    fn more_projects_available(&self) -> bool {
        self.offset + self.limit < self.total_hits
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct IncomingModrinthProject {
    project_id: String,
    slug: String,
    title: String,
    description: String,
    author: String,
    date_created: String,
    date_modified: String,
    versions: Vec<String>,
    downloads: i32,
    follows: i32,
    icon_url: Option<String>,
    monetization_status: Option<String>
}

impl IncomingModrinthProject {
    // Assume that the last entry in the list of MC versions from the API is the latest version.
    fn latest_minecraft_version(&self) -> Option<String> {
        filter_release_minecraft_versions(self.versions.last().cloned())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct GetModrinthProjectResponse {
    source_url: Option<String>,
    status: String,
    versions: Vec<String>
}

impl GetModrinthProjectResponse {
    // Assume that the last entry in the list of plugin versions from the API is the latest version.
    fn latest_version(&self) -> Option<String> {
        self.versions.last().cloned()
    }
}

#[derive(Debug, Error)]
enum SearchModrinthProjectsError {
    #[error("Could not search Modrinth projects {request:?}: Received unexpected status code {status_code}")]
    UnexpectedStatusCode {
        request: SearchModrinthProjectsRequest,
        status_code: u16
    }
}

#[derive(Debug, Error)]
enum GetModrinthProjectError {
    #[error("Project {id}: Latest version not found.")]
    LatestVersionNotFound {
        id: String
    },
    #[error("Could not get Modrinth project '{id}': Received unexpected status code {status_code}")]
    UnexpectedStatusCode {
        id: String,
        status_code: u16
    }
}

impl<T> ModrinthClient<T> where T: HttpServer + Send + Sync {
    #[instrument(
        skip(self, db_pool)
    )]
    pub async fn populate_modrinth_projects(&self, db_pool: &Pool) -> Result<()> {
        let request = SearchModrinthProjectsRequest::create_request();

        let count = Arc::new(AtomicU32::new(0));

        let result = self
            .pages_ahead(MODRINTH_PROJECTS_REQUESTS_AHEAD, Limit::None, request)
            .items()
            .try_for_each_concurrent(MODRINTH_PROJECTS_CONCURRENT_FUTURES, |incoming_project| self.process_incoming_project(incoming_project, db_pool, &count, false))
            .await;

        info!("Modrinth projects populated: {}", count.load(Ordering::Relaxed));

        result
    }

    #[instrument(
        skip(self, db_pool)
    )]
    pub async fn update_modrinth_projects(&self, db_pool: &Pool, update_date_later_than: OffsetDateTime) -> Result<()> {
        let request = SearchModrinthProjectsRequest::create_request();

        let count = Arc::new(AtomicU32::new(0));

        let result = self
            .pages_ahead(MODRINTH_PROJECTS_REQUESTS_AHEAD, Limit::None, request)
            .items()
            .try_take_while(|x| future::ready(Ok(OffsetDateTime::parse(x.date_modified.as_str(), &Rfc3339).unwrap() > update_date_later_than)))
            .try_for_each_concurrent(MODRINTH_PROJECTS_CONCURRENT_FUTURES, |incoming_project| self.process_incoming_project(incoming_project, db_pool, &count, true))
            .await;

        info!("Modrinth projects updated: {}", count.load(Ordering::Relaxed));

        result
    }

    async fn process_incoming_project(&self, incoming_project: IncomingModrinthProject, db_pool: &Pool, count: &Arc<AtomicU32>, get_version: bool) -> Result<()> {
        let project_id = incoming_project.project_id.clone();
        let project_result = self.get_project_from_api(&project_id).await;

        match project_result {
            Ok(project_response) => {
                let mut version_id: Option<String> = None;
                let mut version_name: Option<String> = None;

                if let Some(latest_version_id) = project_response.latest_version() {
                    version_id = Some(latest_version_id.to_string());

                    if get_version {
                        let version_result = self.get_latest_modrinth_project_version_from_api(&project_id, &latest_version_id).await;

                        match version_result {
                            Ok(retrieved_version_name) => version_name = Some(retrieved_version_name),
                            Err(err) => warn!("{}", err)
                        }
                    }
                }

                let convert_result = convert_incoming_project(incoming_project, &project_response, &version_id, &version_name).await;

                match convert_result {
                    Ok(project) => {
                        let db_result = upsert_modrinth_project(db_pool, &project).await;

                        match db_result {
                            Ok(_) => {
                                count.fetch_add(1, Ordering::Relaxed);
                            },
                            Err(err) => warn!("{}", err)
                        }
                    }
                    Err(err) => warn!("{}", err)
                }
            }
            Err(err) => warn!("{}", err)
        }

        Ok(())
    }

    #[instrument(
        skip(self)
    )]
    async fn get_projects_from_api(&self, request: SearchModrinthProjectsRequest) -> Result<SearchModrinthProjectsResponse> {
        self.rate_limiter.until_ready().await;

        let url = self.http_server.base_url().join("search")?;
        let raw_response = self.api_client.get(url)
            .query(&request)
            .send()
            .await?;

        let status = raw_response.status();
        if status == StatusCode::OK {
            let response: SearchModrinthProjectsResponse = raw_response.json().await?;
            Ok(response)
        } else {
            Err(
                SearchModrinthProjectsError::UnexpectedStatusCode {
                    request,
                    status_code: status.into()
                }.into()
            )
        }
    }

    #[instrument(
        skip(self)
    )]
    async fn get_project_from_api(&self, id: &str) -> Result<GetModrinthProjectResponse> {
        self.rate_limiter.until_ready().await;

        let path = &["project/", id].concat();
        let url = self.http_server.base_url().join(path)?;
        let raw_response = self.api_client.get(url)
            .send()
            .await?;

        let status = raw_response.status();
        match status {
            StatusCode::OK => {
                let response: GetModrinthProjectResponse = raw_response.json().await?;
                Ok(response)
            }
            StatusCode::NOT_FOUND => {
                Err(
                    GetModrinthProjectError::LatestVersionNotFound {
                        id: id.to_string()
                    }.into()
                )
            }
            _ => {
                Err(
                    GetModrinthProjectError::UnexpectedStatusCode {
                        id: id.to_string(),
                        status_code: status.into()
                    }.into()
                )
            }
        }
    }
}

impl<T> PageTurner<SearchModrinthProjectsRequest> for ModrinthClient<T> where T: HttpServer + Send + Sync {
    type PageItems = Vec<IncomingModrinthProject>;
    type PageError = anyhow::Error;

    async fn turn_page(&self, mut request: SearchModrinthProjectsRequest) -> TurnedPageResult<Self, SearchModrinthProjectsRequest> {
        let response = self.get_projects_from_api(request.clone()).await?;

        if response.more_projects_available() {
            request.offset += request.limit;
            Ok(TurnedPage::next(response.hits, request))
        } else {
            Ok(TurnedPage::last(response.hits))
        }
    }
}

async fn convert_incoming_project(incoming_project: IncomingModrinthProject, project_response: &GetModrinthProjectResponse, version_id: &Option<String>, version_name: &Option<String>) -> Result<ModrinthProject> {
    let latest_minecraft_version = incoming_project.latest_minecraft_version();

    let mut project = ModrinthProject {
        id: incoming_project.project_id,
        slug: incoming_project.slug,
        name: incoming_project.title,
        description: incoming_project.description,
        author: incoming_project.author,
        date_created: OffsetDateTime::parse(&incoming_project.date_created, &Rfc3339)?,
        date_updated: OffsetDateTime::parse(&incoming_project.date_modified, &Rfc3339)?,
        latest_minecraft_version,
        downloads: incoming_project.downloads,
        follows: incoming_project.follows,
        version_id: version_id.clone(),
        version_name: version_name.clone(),
        status: project_response.status.clone(),
        icon_url: incoming_project.icon_url,
        source_url: project_response.source_url.clone(),
        source_repository: None
    };

    if let Some(url) = &project_response.source_url {
        let option_repo = extract_source_repository_from_url(url.as_str());

        if let Some(repo) = option_repo {
            project.source_repository = Some(SourceRepository {
                host: repo.host,
                owner: repo.owner,
                name: repo.name
            });
        }
    }

    Ok(project)
}

static MINECRAFT_VERSION_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\d+\.\d+(\.\d)?$").unwrap());

// Ignore pre-releases, release candidates, snapshots, and other versions like "b1.7.3" that can mess up the ordering of versions.
fn filter_release_minecraft_versions(latest_minecraft_version: Option<String>) -> Option<String> {
    if let Some(ref version) = latest_minecraft_version {
        if MINECRAFT_VERSION_REGEX.is_match(version) {
            return latest_minecraft_version;
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::modrinth::test::ModrinthTestServer;

    use rstest::*;
    use speculoos::prelude::*;
    use time::macros::datetime;
    use wiremock::{Mock, ResponseTemplate};
    use wiremock::matchers::{method, path, query_param};

    #[tokio::test]
    async fn should_get_projects_from_api() -> Result<()> {
        // Arrange
        let modrinth_server = ModrinthTestServer::new().await;

        let request = SearchModrinthProjectsRequest::create_request();

        let expected_response = SearchModrinthProjectsResponse {
            hits: create_test_modrinth_projects(),
            limit: 100,
            offset: 200,
            total_hits: 1000
        };

        let response_template = ResponseTemplate::new(200)
            .set_body_json(expected_response.clone());

        Mock::given(method("GET"))
            .and(path("/search"))
            .and(query_param("limit", request.limit.to_string().as_str()))
            .and(query_param("offset", request.offset.to_string().as_str()))
            .and(query_param("index", request.index.as_str()))
            .respond_with(response_template)
            .mount(modrinth_server.mock())
            .await;

        // Act
        let modrinth_client = ModrinthClient::new(modrinth_server)?;
        let response = modrinth_client.get_projects_from_api(request).await;

        // Assert
        assert_that(&response).is_ok().is_equal_to(expected_response);

        Ok(())
    }

    #[tokio::test]
    async fn should_get_project_from_api() -> Result<()> {
        // Arrange
        let modrinth_server = ModrinthTestServer::new().await;

        let expected_response = GetModrinthProjectResponse {
            source_url: Some("https://github.com/alice/foo".to_string()),
            status: "approved".to_string(),
            versions: vec!["aaaa1111".to_string(), "bbbb2222".to_string(), "cccc3333".to_string()]
        };

        let response_template = ResponseTemplate::new(200)
            .set_body_json(expected_response.clone());

        let project_id = "aaaaaaaa";
        let request_path = &["project/", project_id].concat();
        Mock::given(method("GET"))
            .and(path(request_path))
            .respond_with(response_template)
            .mount(modrinth_server.mock())
            .await;

        // Act
        let modrinth_client = ModrinthClient::new(modrinth_server)?;
        let project_result = modrinth_client.get_project_from_api(project_id).await;

        // Assert
        assert_that(&project_result).is_ok().is_equal_to(expected_response);

        Ok(())
    }

    #[tokio::test]
    async fn should_process_incoming_project() -> Result<()> {
        // Arrange
        let incoming_project = create_test_modrinth_projects()[0].clone();
        let project_response = GetModrinthProjectResponse {
            source_url: Some("https://github.com/alice/foo".to_string()),
            status: "approved".to_string(),
            versions: vec!["aaaa1111".to_string(), "bbbb2222".to_string(), "cccc3333".to_string()]
        };

        let version_id = "cccc3333";
        let version_name = "v1.2.3";

        // Act
        let project = convert_incoming_project(incoming_project, &project_response, &Some(version_id.to_string()), &Some(version_name.to_string())).await?;

        // Assert
        let expected_project = ModrinthProject {
            id: "aaaaaaaa".to_string(),
            slug: "foo".to_string(),
            name: "foo-modrinth".to_string(),
            description: "foo-modrinth-description".to_string(),
            author: "alice".to_string(),
            date_created: datetime!(2021-01-01 0:00 UTC),
            date_updated: datetime!(2021-02-03 0:00 UTC),
            latest_minecraft_version: Some("1.21".to_string()),
            downloads: 100,
            follows: 200,
            version_id: project_response.latest_version(),
            version_name: Some(version_name.to_string()),
            status: project_response.status,
            icon_url: Some("https://cdn.modrinth.com/data/aaaaaaaa/icon.png".to_string()),
            source_url: project_response.source_url,
            source_repository: Some(SourceRepository {
                host: "github.com".to_string(),
                owner: "alice".to_string(),
                name: "foo".to_string()
            })
        };

        assert_that(&project).is_equal_to(expected_project);

        Ok(())
    }

    #[tokio::test]
    #[rstest]
    #[case::beta("b1.7.3")]
    #[case::snapshot("24w46a")]
    #[case::prerelease("1.21.4-pre3")]
    #[case::release_candidate("1.21.4-rc3")]
    async fn should_process_incoming_project_where_latest_minecraft_version_is_not_release_version(#[case] version: &str) -> Result<()> {
        // Arrange
        let mut incoming_project = create_test_modrinth_projects()[0].clone();
        incoming_project.versions = vec![version.to_string()];
        let project_response = GetModrinthProjectResponse {
            source_url: Some("https://github.com/alice/foo".to_string()),
            status: "approved".to_string(),
            versions: vec!["aaaa1111".to_string(), "bbbb2222".to_string(), "cccc3333".to_string()]
        };

        let version_id = "cccc3333";
        let version_name = "v1.2.3";

        // Act
        let project = convert_incoming_project(incoming_project, &project_response, &Some(version_id.to_string()), &Some(version_name.to_string())).await?;

        // Assert
        assert_that(&project.latest_minecraft_version).is_none();

        Ok(())
    }

    fn create_test_modrinth_projects() -> Vec<IncomingModrinthProject> {
        vec![
            IncomingModrinthProject {
                project_id: "aaaaaaaa".to_string(),
                slug: "foo".to_string(),
                title: "foo-modrinth".to_string(),
                description: "foo-modrinth-description".to_string(),
                author: "alice".to_string(),
                date_created: "2021-01-01T00:00:00Z".to_string(),
                date_modified: "2021-02-03T00:00:00Z".to_string(),
                versions: vec!["1.20".to_string(), "1.20.6".to_string(), "1.21".to_string()],
                downloads: 100,
                follows: 200,
                icon_url: Some("https://cdn.modrinth.com/data/aaaaaaaa/icon.png".to_string()),
                monetization_status: None
            },
            IncomingModrinthProject {
                project_id: "bbbbbbbb".to_string(),
                slug: "bar".to_string(),
                title: "bar-modrinth".to_string(),
                description: "bar-modrinth-description".to_string(),
                author: "bob".to_string(),
                date_created: "2021-01-02T00:00:00Z".to_string(),
                date_modified: "2021-02-02T00:00:00Z".to_string(),
                versions: vec!["1.6".to_string(), "1.7".to_string(), "1.8".to_string()],
                downloads: 300,
                follows: 300,
                icon_url: Some("https://cdn.modrinth.com/data/bbbbbbbb/icon.png".to_string()),
                monetization_status: None
            },
        ]
    }
}