use crate::collector::HttpServer;
use crate::collector::hangar::HangarClient;

use anyhow::Result;
use deadpool_postgres::Client;
use futures::future;
use futures::stream::TryStreamExt;
use page_turner::prelude::*;
use serde::{Serialize, Deserialize};
use time::OffsetDateTime;
use std::cell::Cell;
use std::fmt::Debug;
use std::rc::Rc;
use thiserror::Error;
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
    pagination: GetHangarProjectsResponsePagination,
    result: Vec<IncomingHangarProject>
}

impl GetHangarProjectsResponse {
    fn more_projects_available(&self) -> bool {
        self.pagination.offset + self.pagination.limit < self.pagination.count
    }
}

#[derive(Clone, Debug,  Deserialize, PartialEq, Serialize)]
struct GetHangarProjectsResponsePagination {
    limit: u32,
    offset: u32,
    count: u32
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

// TODO: IncomingHangarProjectError?

impl<T> HangarClient<T> where T: HttpServer + Send + Sync {
    #[instrument(
        skip(self, db_client)
    )]
    pub async fn populate_hangar_projects(&self, db_client: &Client) -> Result<()> {
        let request = GetHangarProjectsRequest::create_request();

        let count_rc: Rc<Cell<u32>> = Rc::new(Cell::new(0));

        let result= self
            .pages_ahead(HANGAR_POPULATE_PROJECTS_REQUESTS_AHEAD, Limit::None, request)
            .items()
            .try_for_each_concurrent(None, |incoming_project| {
                let count_rc_clone = count_rc.clone();
                async move {
                    // TODO: Process and upsert
                    // println!("{:?}", incoming_project);

                    count_rc_clone.set(count_rc_clone.get() + 1);

                    Ok(())
                }
            })
            .await;

        let count = count_rc.get();
        info!("Hangar projects populated: {}", count);

        result
    }

    // TODO: update_hangar_projects

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

// TODO: process_incoming_project

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
            pagination: GetHangarProjectsResponsePagination {
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

    // TODO: should_process_incoming_project

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