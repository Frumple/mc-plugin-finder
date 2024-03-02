use crate::collector::HttpServer;
use crate::collector::hangar::HangarClient;
use crate::collector::util::extract_source_repository_from_url;
use crate::database::hangar::project::{HangarProject, upsert_hangar_project};

use anyhow::Result;
use deadpool_postgres::Client;
use futures::future;
use futures::stream::TryStreamExt;
use page_turner::prelude::*;
use serde::{Serialize, Deserialize};
use time::OffsetDateTime;
use std::cell::Cell;
use std::fmt::Debug;
use thiserror::Error;
use time::format_description::well_known::Rfc3339;
use tracing::{info, warn, instrument};

const HANGAR_POPULATE_PROJECTS_REQUESTS_AHEAD: usize = 2;

#[derive(Clone, Debug, Serialize)]
struct GetHangarProjectsRequest {
    limit: u32,
    offset: u32,
    sort: String
}

#[derive(Clone, Debug, Serialize)]
struct GetHangarProjectsRequestPagination {
    limit: u32,
    offset: u32
}

impl GetHangarProjectsRequest {
    fn create_request() -> Self {
        Self {
            limit: 25,
            offset: 0,
            sort: "updated".to_string()
        }
    }
}

impl RequestAhead for GetHangarProjectsRequest {
    fn next_request(&self) -> Self {
        Self {
            limit: self.limit,
            offset: self.offset + self.limit,
            sort: self.sort.clone()
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
struct GetHangarProjectsResponse {
    pagination: HangarResponsePagination,
    result: Vec<IncomingHangarProject>
}

impl GetHangarProjectsResponse {
    fn more_projects_available(&self) -> bool {
        self.pagination.offset + self.pagination.limit < self.pagination.count
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IncomingHangarProject {
    name: String,
    description: String,
    namespace: IncomingHangarProjectNamespace,
    created_at: String,
    last_updated: String,
    visibility: String,
    avatar_url: String,
    settings: IncomingHangarProjectSettings
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct IncomingHangarProjectNamespace {
    owner: String,
    slug: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct IncomingHangarProjectSettings {
    links: Vec<IncomingHangarProjectLinkGroup>,
    tags: Vec<String>,
    keywords: Vec<String>
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct IncomingHangarProjectLinkGroup {
    id: u32,
    #[serde(rename = "type")]
    r_type: String,
    title: Option<String>,
    links: Vec<IncomingHangarProjectLink>
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct IncomingHangarProjectLink {
    id: u32,
    name: String,
    url: Option<String>
}

#[derive(Clone, Debug, Serialize)]
pub struct GetHangarProjectVersionsRequest {
    limit: u32,
    offset: u32
}

impl GetHangarProjectVersionsRequest {
    fn create_request() -> Self {
        Self {
            limit: 1,
            offset: 0
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct GetHangarProjectVersionsResponse {
    pagination: HangarResponsePagination,
    result: Vec<IncomingHangarProjectVersion>
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct IncomingHangarProjectVersion {
    name: String
    // TODO: Get visibility and channel?
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
struct HangarResponsePagination {
    limit: u32,
    offset: u32,
    count: u32
}

#[derive(Debug, Error)]
enum IncomingHangarProjectError {
    #[error("Skipping project {slug}: Latest version not found.")]
    LatestVersionNotFound {
        slug: String
    }
}

impl<T> HangarClient<T> where T: HttpServer + Send + Sync {
    #[instrument(
        skip(self, db_client)
    )]
    pub async fn populate_hangar_projects(&self, db_client: &Client) -> Result<()> {
        let request = GetHangarProjectsRequest::create_request();

        let count_cell: Cell<u32> = Cell::new(0);

        let result= self
            .pages_ahead(HANGAR_POPULATE_PROJECTS_REQUESTS_AHEAD, Limit::None, request)
            .items()
            .try_for_each_concurrent(None, |incoming_project| self.process_incoming_project(incoming_project, db_client, &count_cell))
            .await;

        let count = count_cell.get();
        info!("Hangar projects populated: {}", count);

        result
    }

    #[instrument(
        skip(self, db_client)
    )]
    pub async fn update_hangar_projects(&self, db_client: &Client, update_date_later_than: OffsetDateTime) -> Result<()> {
        let request = GetHangarProjectsRequest::create_request();

        let count_cell: Cell<u32> = Cell::new(0);

        let result = self
            .pages(request)
            .items()
            .try_take_while(|x| future::ready(Ok(OffsetDateTime::parse(x.last_updated.as_str(), &Rfc3339).unwrap() > update_date_later_than)))
            .try_for_each(|incoming_project| self.process_incoming_project(incoming_project, db_client, &count_cell))
            .await;

        let count = count_cell.get();
        info!("Hangar projects updated: {}", count);

        result
    }

    async fn process_incoming_project(&self, incoming_project: IncomingHangarProject, db_client: &Client, count_cell: &Cell<u32>) -> Result<()> {
        let version_result = self.get_project_latest_version_from_api(&incoming_project.namespace.slug).await;

        match version_result {
            Ok(version) => {
                let process_result = convert_incoming_project(incoming_project, &version).await;

                match process_result {
                    Ok(project) => {
                        let db_result = upsert_hangar_project(db_client, project).await;

                        match db_result {
                            Ok(_) => count_cell.set(count_cell.get() + 1),
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
    async fn get_projects_from_api(&self, request: GetHangarProjectsRequest) -> Result<GetHangarProjectsResponse> {
        self.rate_limiter.until_ready().await;

        let url = self.http_server.base_url().join("projects")?;
        let raw_response = self.api_client.get(url)
            .query(&request)
            .send()
            .await?;

        let response: GetHangarProjectsResponse = raw_response.json().await?;

        Ok(response)
    }

    #[instrument(
        level = "trace",
        skip(self)
    )]
    async fn get_project_latest_version_from_api(&self, slug: &str) -> Result<String> {
        self.rate_limiter.until_ready().await;

        // Hangar's "projects/{slug}/versions" seems to order versions for newest to oldest.
        // This allows us to assume that the first version returned is the latest.
        // However, there is no guarantee this behaviour will remain in the future.
        let request = GetHangarProjectVersionsRequest::create_request();

        let path = &["projects/", slug, "/versions"].concat();
        let url = self.http_server.base_url().join(path)?;
        let raw_response = self.api_client.get(url)
            .query(&request)
            .send()
            .await?;

        let response: GetHangarProjectVersionsResponse = raw_response.json().await?;

        if response.result.is_empty() {
            return Err(
                IncomingHangarProjectError::LatestVersionNotFound {
                    slug: slug.to_string()
                }.into()
            )
        }

        Ok(response.result[0].name.clone())
    }
}

impl<T> PageTurner<GetHangarProjectsRequest> for HangarClient<T> where T: HttpServer + Send + Sync {
    type PageItems = Vec<IncomingHangarProject>;
    type PageError = anyhow::Error;

    async fn turn_page(&self, mut request: GetHangarProjectsRequest) -> TurnedPageResult<Self, GetHangarProjectsRequest> {
        let response = self.get_projects_from_api(request.clone()).await?;

        if response.more_projects_available() {
            request.offset += request.limit;
            Ok(TurnedPage::next(response.result, request))
        } else {
            Ok(TurnedPage::last(response.result))
        }
    }
}

async fn convert_incoming_project(incoming_project: IncomingHangarProject, version: &str) -> Result<HangarProject> {
    let source_code_link = find_source_code_link(incoming_project.settings);

    let mut project = HangarProject {
        slug: incoming_project.namespace.slug,
        owner: incoming_project.namespace.owner,
        name: incoming_project.name,
        description: incoming_project.description,
        created_at: OffsetDateTime::parse(&incoming_project.created_at, &Rfc3339)?,
        last_updated: OffsetDateTime::parse(&incoming_project.last_updated, &Rfc3339)?,
        visibility: incoming_project.visibility,
        avatar_url: incoming_project.avatar_url,
        version: version.to_string(),
        source_code_link: source_code_link.clone(),
        source_repository_host: None,
        source_repository_owner: None,
        source_repository_name: None
    };

    let option_repo = if let Some(url) = source_code_link {
        extract_source_repository_from_url(url.as_str())
    } else {
        None
    };

    if let Some(repo) = option_repo {
        project.source_repository_host = Some(repo.host);
        project.source_repository_owner = Some(repo.owner);
        project.source_repository_name = Some(repo.name);
    }

    Ok(project)
}

fn find_source_code_link(settings: IncomingHangarProjectSettings) -> Option<String> {
    for link_group in settings.links {
        for link in link_group.links {
            if link.name.contains("Source") {
                return link.url
            }
        }
    }

    None
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::collector::hangar::test::HangarTestServer;

    use speculoos::prelude::*;
    use time::macros::datetime;
    use wiremock::{Mock, ResponseTemplate};
    use wiremock::matchers::{method, path, query_param};

    #[tokio::test]
    async fn should_get_projects_from_api() -> Result<()> {
        // Arrange
        let hangar_server = HangarTestServer::new().await;

        let request = GetHangarProjectsRequest::create_request();

        let expected_response = GetHangarProjectsResponse {
            pagination: HangarResponsePagination {
                limit: 25,
                offset: 50,
                count: 100
            },
            result: create_test_projects()
        };

        let response_template = ResponseTemplate::new(200)
            .set_body_json(expected_response.clone());

        Mock::given(method("GET"))
            .and(path("/projects"))
            .and(query_param("limit", request.limit.to_string().as_str()))
            .and(query_param("offset", request.offset.to_string().as_str()))
            .and(query_param("sort", request.sort.as_str()))
            .respond_with(response_template)
            .mount(hangar_server.mock())
            .await;

        // Act
        let hangar_client = HangarClient::new(hangar_server)?;
        let response = hangar_client.get_projects_from_api(request).await?;

        // Assert
        assert_that(&response).is_equal_to(expected_response);

        Ok(())
    }

    #[tokio::test]
    async fn should_get_project_latest_version_from_api() -> Result<()> {
        // Arrange
        let hangar_server = HangarTestServer::new().await;

        let request = GetHangarProjectVersionsRequest::create_request();

        let expected_version = "v1.2.3";
        let expected_response = GetHangarProjectVersionsResponse {
            pagination: HangarResponsePagination {
                limit: 0,
                offset: 1,
                count: 10
            },
            result: vec![
                IncomingHangarProjectVersion {
                    name: expected_version.to_string()
                }
            ]
        };

        let response_template = ResponseTemplate::new(200)
            .set_body_json(expected_response);

        let slug = "foo";
        let request_path = &["projects/", slug, "/versions"].concat();
        Mock::given(method("GET"))
            .and(path(request_path))
            .and(query_param("limit", request.limit.to_string().as_str()))
            .and(query_param("offset", request.offset.to_string().as_str()))
            .respond_with(response_template)
            .mount(hangar_server.mock())
            .await;

        // Act
        let hangar_client = HangarClient::new(hangar_server)?;
        let version = hangar_client.get_project_latest_version_from_api(slug).await?;

        // Assert
        assert_that(&version).is_equal_to(expected_version.to_string());

        Ok(())
    }

    #[tokio::test]
    async fn should_process_incoming_project() -> Result<()> {
        // Arrange
        let incoming_project = create_test_projects()[0].clone();
        let version = "v1.2.3";

        // Act
        let project = convert_incoming_project(incoming_project, version).await?;

        // Assert
        assert_that(&project.slug).is_equal_to("foo".to_string());
        assert_that(&project.owner).is_equal_to("Frumple".to_string());
        assert_that(&project.name).is_equal_to("project-1".to_string());
        assert_that(&project.description).is_equal_to("project-1-description".to_string());
        assert_that(&project.created_at).is_equal_to(datetime!(2020-01-01 0:00 UTC));
        assert_that(&project.last_updated).is_equal_to(datetime!(2021-01-01 0:00 UTC));
        assert_that(&project.visibility).is_equal_to("public".to_string());
        assert_that(&project.avatar_url).is_equal_to("https://hangarcdn.papermc.io/avatars/project/1.webp?v=1".to_string());
        assert_that(&project.version).is_equal_to(version.to_string());
        assert_that(&project.source_code_link).is_some().is_equal_to("https://github.com/Frumple/foo".to_string());
        assert_that(&project.source_repository_host).is_some().is_equal_to("github.com".to_string());
        assert_that(&project.source_repository_owner).is_some().is_equal_to("Frumple".to_string());
        assert_that(&project.source_repository_name).is_some().is_equal_to("foo".to_string());

        Ok(())
    }

    fn create_test_projects() -> Vec<IncomingHangarProject> {
        vec![
            IncomingHangarProject {
                name: "project-1".to_string(),
                description: "project-1-description".to_string(),
                namespace: IncomingHangarProjectNamespace {
                    owner: "Frumple".to_string(),
                    slug: "foo".to_string()
                },
                created_at: "2020-01-01T00:00:00Z".to_string(),
                last_updated: "2021-01-01T00:00:00Z".to_string(),
                visibility: "public".to_string(),
                avatar_url: "https://hangarcdn.papermc.io/avatars/project/1.webp?v=1".to_string(),
                settings: IncomingHangarProjectSettings {
                    links: create_test_project_links(),
                    tags: vec!["ADDON".to_string(), "SUPPORTS_FOLIA".to_string()],
                    keywords: vec!["foo".to_string(), "fi".to_string()]
                }
            },
            IncomingHangarProject {
                name: "project-2".to_string(),
                description: "project-2-description".to_string(),
                namespace: IncomingHangarProjectNamespace {
                    owner: "Frumple".to_string(),
                    slug: "bar".to_string()
                },
                created_at: "2020-01-01T00:00:00Z".to_string(),
                last_updated: "2022-01-01T00:00:00Z".to_string(),
                visibility: "public".to_string(),
                avatar_url: "https://hangarcdn.papermc.io/avatars/project/1.webp?v=1".to_string(),
                settings: IncomingHangarProjectSettings {
                    links: create_test_project_links(),
                    tags: vec!["ADDON".to_string(), "SUPPORTS_FOLIA".to_string()],
                    keywords: vec!["foo".to_string(), "fi".to_string()]
                }
            },
        ]
    }

    fn create_test_project_links() -> Vec<IncomingHangarProjectLinkGroup> {
        vec![
            IncomingHangarProjectLinkGroup {
                id: 0,
                r_type: "top".to_string(),
                title: Some("top".to_string()),
                links: vec![
                    IncomingHangarProjectLink {
                        id: 1,
                        name: "Issues".to_string(),
                        url: Some("https://github.com/Frumple/foo/issues".to_string())
                    },
                    IncomingHangarProjectLink {
                        id: 2,
                        name: "Source".to_string(),
                        url: Some("https://github.com/Frumple/foo".to_string())
                    },
                    IncomingHangarProjectLink {
                        id: 3,
                        name: "Support".to_string(),
                        url: Some("https://github.com/Frumple/foo/discussions".to_string())
                    },
                    IncomingHangarProjectLink {
                        id: 4,
                        name: "Wiki".to_string(),
                        url: Some("https://github.com/Frumple/foo/wiki".to_string())
                    }
                ]
            }
        ]
    }
}