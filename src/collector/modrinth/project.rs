use crate::collector::HttpServer;
use crate::collector::modrinth::ModrinthClient;
use crate::collector::util::extract_source_repository_from_url;
use crate::database::modrinth::project::{ModrinthProject, upsert_modrinth_project};

use anyhow::Result;
use deadpool_postgres::Pool;
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

const MODRINTH_POPULATE_PROJECTS_REQUESTS_AHEAD: usize = 2;

#[derive(Clone, Debug, Serialize)]
struct GetModrinthSearchProjectsRequest {
    facets: String,
    limit: u32,
    offset: u32,
    index: String
}

impl GetModrinthSearchProjectsRequest {
    fn create_request() -> Self {
        Self {
            facets: "[[\"project_type:plugin\"]]".to_string(),
            limit: 100,
            offset: 0,
            index: "updated".to_string()
        }
    }
}

impl RequestAhead for GetModrinthSearchProjectsRequest {
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
struct GetModrinthSearchProjectsResponse {
    hits: Vec<IncomingModrinthProject>,
    offset: u32,
    limit: u32,
    total_hits: u32
}

impl GetModrinthSearchProjectsResponse {
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
    downloads: i32,
    latest_version: Option<String>,
    icon_url: Option<String>,
    monetization_status: Option<String>
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct GetModrinthProjectResponse {
    source_url: Option<String>
}

#[derive(Debug, Error)]
enum IncomingModrinthProjectError {
    #[error("Skipping project {id}: Latest version not found.")]
    LatestVersionNotFound {
        id: String
    }
}

impl<T> ModrinthClient<T> where T: HttpServer + Send + Sync {
    #[instrument(
        skip(self, db_pool)
    )]
    pub async fn populate_modrinth_projects(&self, db_pool: &Pool) -> Result<()> {
        let request = GetModrinthSearchProjectsRequest::create_request();

        let count_cell: Cell<u32> = Cell::new(0);

        let result = self
            .pages_ahead(MODRINTH_POPULATE_PROJECTS_REQUESTS_AHEAD, Limit::None, request)
            .items()
            .try_for_each_concurrent(None, |incoming_project| self.process_incoming_project(incoming_project, db_pool, &count_cell, false))
            .await;

        let count = count_cell.get();
        info!("Modrinth projects populated: {}", count);

        result
    }

    #[instrument(
        skip(self, db_pool)
    )]
    pub async fn update_modrinth_projects(&self, db_pool: &Pool, update_date_later_than: OffsetDateTime) -> Result<()> {
        let request = GetModrinthSearchProjectsRequest::create_request();

        let count_cell: Cell<u32> = Cell::new(0);

        let result = self
            .pages(request)
            .items()
            .try_take_while(|x| future::ready(Ok(OffsetDateTime::parse(x.date_modified.as_str(), &Rfc3339).unwrap() > update_date_later_than)))
            .try_for_each(|incoming_project| self.process_incoming_project(incoming_project, db_pool, &count_cell, true))
            .await;

        let count = count_cell.get();
        info!("Modrinth projects updated: {}", count);

        result
    }

    async fn process_incoming_project(&self, incoming_project: IncomingModrinthProject, db_pool: &Pool, count_cell: &Cell<u32>, get_version: bool) -> Result<()> {
        let mut version_name = None;

        let project_id = incoming_project.project_id.clone();

        if get_version {
            if let Some(version_id) = incoming_project.latest_version.clone() {
                let version_result = self.get_latest_modrinth_project_version_from_api(&project_id, &version_id).await;

                match version_result {
                    Ok(retrieved_version_name) => version_name = Some(retrieved_version_name),
                    Err(err) => warn!("{}", err)
                }
            }
        }

        let source_result = self.get_project_source_url_from_api(&project_id).await;

        match source_result {
            Ok(source_url) => {
                let convert_result = convert_incoming_project(incoming_project, &source_url, &version_name).await;

                match convert_result {
                    Ok(project) => {
                        let db_result = upsert_modrinth_project(db_pool, &project).await;

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
    async fn get_projects_from_api(&self, request: GetModrinthSearchProjectsRequest) -> Result<GetModrinthSearchProjectsResponse> {
        self.rate_limiter.until_ready().await;

        let url = self.http_server.base_url().join("search")?;
        let raw_response = self.api_client.get(url)
            .query(&request)
            .send()
            .await?;

        let response: GetModrinthSearchProjectsResponse = raw_response.json().await?;

        Ok(response)
    }

    #[instrument(
        level = "debug",
        skip(self)
    )]
    async fn get_project_source_url_from_api(&self, id: &str) -> Result<Option<String>> {
        self.rate_limiter.until_ready().await;

        let path = &["project/", id].concat();
        let url = self.http_server.base_url().join(path)?;
        let raw_response = self.api_client.get(url)
            .send()
            .await?;

        let response: GetModrinthProjectResponse = raw_response.json().await?;

        Ok(response.source_url)
    }
}

impl<T> PageTurner<GetModrinthSearchProjectsRequest> for ModrinthClient<T> where T: HttpServer + Send + Sync {
    type PageItems = Vec<IncomingModrinthProject>;
    type PageError = anyhow::Error;

    async fn turn_page(&self, mut request: GetModrinthSearchProjectsRequest) -> TurnedPageResult<Self, GetModrinthSearchProjectsRequest> {
        let response = self.get_projects_from_api(request.clone()).await?;

        if response.more_projects_available() {
            request.offset += request.limit;
            Ok(TurnedPage::next(response.hits, request))
        } else {
            Ok(TurnedPage::last(response.hits))
        }
    }
}

async fn convert_incoming_project(incoming_project: IncomingModrinthProject, source_url: &Option<String>, version_name: &Option<String>) -> Result<ModrinthProject> {
    let project_id = incoming_project.project_id;

    if let Some(version_id) = incoming_project.latest_version {
        let mut project = ModrinthProject {
            id: project_id,
            slug: incoming_project.slug,
            title: incoming_project.title,
            description: incoming_project.description,
            author: incoming_project.author,
            date_created: OffsetDateTime::parse(&incoming_project.date_created, &Rfc3339)?,
            date_modified: OffsetDateTime::parse(&incoming_project.date_modified, &Rfc3339)?,
            downloads: incoming_project.downloads,
            version_id,
            version_name: version_name.clone(),
            icon_url: incoming_project.icon_url,
            monetization_status: incoming_project.monetization_status,
            source_url: source_url.clone(),
            source_repository_host: None,
            source_repository_owner: None,
            source_repository_name: None
        };

        if let Some(url) = source_url {
            let option_repo = extract_source_repository_from_url(url.as_str());

            if let Some(repo) = option_repo {
                project.source_repository_host = Some(repo.host);
                project.source_repository_owner = Some(repo.owner);
                project.source_repository_name = Some(repo.name);
            }
        }

        Ok(project)
    } else {
        Err(
            IncomingModrinthProjectError::LatestVersionNotFound {
                id: project_id
            }.into()
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::collector::modrinth::test::ModrinthTestServer;

    use speculoos::prelude::*;
    use time::macros::datetime;
    use wiremock::{Mock, ResponseTemplate};
    use wiremock::matchers::{method, path, query_param};

    #[tokio::test]
    async fn should_get_projects_from_api() -> Result<()> {
        // Arrange
        let modrinth_server = ModrinthTestServer::new().await;

        let request = GetModrinthSearchProjectsRequest::create_request();

        let expected_response = GetModrinthSearchProjectsResponse {
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
        let response = modrinth_client.get_projects_from_api(request).await?;

        // Assert
        assert_that(&response).is_equal_to(expected_response);

        Ok(())
    }

    #[tokio::test]
    async fn should_get_project_source_url_from_api() -> Result<()> {
        // Arrange
        let modrinth_server = ModrinthTestServer::new().await;

        let expected_source_url = "https://github.com/Frumple/foo";
        let expected_response = GetModrinthProjectResponse {
            source_url: Some(expected_source_url.to_string())
        };

        let response_template = ResponseTemplate::new(200)
            .set_body_json(expected_response);

        let project_id = "aaaaaaaa";
        let request_path = &["project/", project_id].concat();
        Mock::given(method("GET"))
            .and(path(request_path))
            .respond_with(response_template)
            .mount(modrinth_server.mock())
            .await;

        // Act
        let modrinth_client = ModrinthClient::new(modrinth_server)?;
        let source_url = modrinth_client.get_project_source_url_from_api(project_id).await?;

        // Assert
        assert_that(&source_url).is_some().is_equal_to(expected_source_url.to_string());

        Ok(())
    }

    #[tokio::test]
    async fn should_process_incoming_project() -> Result<()> {
        // Arrange
        let incoming_project = create_test_modrinth_projects()[0].clone();
        let source_url = "https://github.com/Frumple/foo";
        let version_name = "v1.2.3";

        // Act
        let project = convert_incoming_project(incoming_project, &Some(source_url.to_string()), &Some(version_name.to_string())).await?;

        // Assert
        assert_that(&project.id).is_equal_to("aaaaaaaa".to_string());
        assert_that(&project.slug).is_equal_to("foo".to_string());
        assert_that(&project.title).is_equal_to("foo".to_string());
        assert_that(&project.description).is_equal_to("foo-description".to_string());
        assert_that(&project.author).is_equal_to("Frumple".to_string());
        assert_that(&project.date_created).is_equal_to(datetime!(2020-01-01 0:00 UTC));
        assert_that(&project.date_modified).is_equal_to(datetime!(2021-01-01 0:00 UTC));
        assert_that(&project.downloads).is_equal_to(100);
        assert_that(&project.version_id).is_equal_to("aaaa1111".to_string());
        assert_that(&project.version_name).is_none();
        assert_that(&project.icon_url).is_some().is_equal_to("https://cdn.modrinth.com/data/aaaaaaaa/icon.png".to_string());
        assert_that(&project.monetization_status).is_none();
        assert_that(&project.source_url).is_some().is_equal_to(source_url.to_string());
        assert_that(&project.source_repository_host).is_some().is_equal_to("github.com".to_string());
        assert_that(&project.source_repository_owner).is_some().is_equal_to("Frumple".to_string());
        assert_that(&project.source_repository_name).is_some().is_equal_to("foo".to_string());

        Ok(())
    }

    fn create_test_modrinth_projects() -> Vec<IncomingModrinthProject> {
        vec![
            IncomingModrinthProject {
                project_id: "aaaaaaaa".to_string(),
                slug: "foo".to_string(),
                title: "foo".to_string(),
                description: "foo-description".to_string(),
                author: "Frumple".to_string(),
                date_created: "2020-01-01T00:00:00Z".to_string(),
                date_modified: "2021-01-01T00:00:00Z".to_string(),
                downloads: 100,
                latest_version: Some("aaaa1111".to_string()),
                icon_url: Some("https://cdn.modrinth.com/data/aaaaaaaa/icon.png".to_string()),
                monetization_status: None
            },
            IncomingModrinthProject {
                project_id: "bbbbbbbb".to_string(),
                slug: "bar".to_string(),
                title: "bar".to_string(),
                description: "bar-description".to_string(),
                author: "Frumple".to_string(),
                date_created: "2020-01-01T00:00:00Z".to_string(),
                date_modified: "2022-01-01T00:00:00Z".to_string(),
                downloads: 100,
                latest_version: Some("bbbb1111".to_string()),
                icon_url: Some("https://cdn.modrinth.com/data/bbbbbbbb/icon.png".to_string()),
                monetization_status: None
            },
        ]
    }
}