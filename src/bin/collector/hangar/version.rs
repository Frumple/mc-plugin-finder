use crate::HttpServer;
use crate::hangar::HangarClient;
use crate::hangar::project::HangarResponsePagination;
use mc_plugin_finder::database::hangar::project::{get_hangar_projects, upsert_hangar_project, HangarProject};

use anyhow::Result;
use deadpool_postgres::Pool;
use futures::stream::{self, StreamExt, TryStreamExt};
use reqwest::StatusCode;
use serde::{Serialize, Deserialize};
use std::cell::Cell;
use std::fmt::Debug;
use thiserror::Error;
use tracing::{info, warn, instrument};

#[derive(Clone, Debug, Serialize)]
pub struct GetHangarVersionsRequest {
    limit: u32,
    offset: u32
}

impl GetHangarVersionsRequest {
    fn create_request() -> Self {
        Self {
            limit: 1,
            offset: 0
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct GetHangarVersionsResponse {
    pagination: HangarResponsePagination,
    result: Vec<IncomingHangarVersion>
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct IncomingHangarVersion {
    name: String
    // TODO: Get visibility and channel?
}

#[derive(Debug, Error)]
enum GetHangarVersionError {
    #[error("Project '{slug}': Latest version not found.")]
    LatestVersionNotFound {
        slug: String
    },
    #[error("Project '{slug}': Received unexpected status code {status_code}")]
    UnexpectedStatusCode {
        slug: String,
        status_code: u16
    }
}

impl<T> HangarClient<T> where T: HttpServer + Send + Sync {
    #[instrument(
        skip(self, db_pool)
    )]
    pub async fn populate_hangar_versions(&self, db_pool: &Pool) -> Result<()> {
        let count_cell: Cell<u32> = Cell::new(0);

        let projects = get_hangar_projects(db_pool).await?;
        let project_stream = stream::iter(projects);

        let result = project_stream
            .map(Ok)
            .try_for_each_concurrent(None, |project| self.process_hangar_project(project, db_pool, &count_cell))
            .await;

        let count = count_cell.get();
        info!("Hangar project versions populated: {}", count);

        result
    }

    async fn process_hangar_project(&self, project: HangarProject, db_pool: &Pool, count_cell: &Cell<u32>) -> Result<()> {
        let version_result = self.get_latest_hangar_project_version_from_api(&project.slug).await;

        match version_result {
            Ok(version_name) => {
                let mut new_project = project.clone();
                new_project.version_name = Some(version_name);
                let db_result = upsert_hangar_project(db_pool, &new_project).await;

                match db_result {
                    Ok(_) => count_cell.set(count_cell.get() + 1),
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
    pub async fn get_latest_hangar_project_version_from_api(&self, slug: &str) -> Result<String> {
        self.rate_limiter.until_ready().await;

        // Hangar's "projects/{slug}/versions" seems to order versions for newest to oldest.
        // This allows us to assume that the first version returned is the latest.
        // However, there is no guarantee this behaviour will remain in the future.
        let request = GetHangarVersionsRequest::create_request();

        let path = &["projects/", slug, "/versions"].concat();
        let url = self.http_server.base_url().join(path)?;
        let raw_response = self.api_client.get(url)
            .query(&request)
            .send()
            .await?;

        let status = raw_response.status();
        match status {
            StatusCode::OK => {
                let response: GetHangarVersionsResponse = raw_response.json().await?;

                if response.result.is_empty() {
                    return Err(
                        GetHangarVersionError::LatestVersionNotFound {
                            slug: slug.to_string()
                        }.into()
                    )
                }

                Ok(response.result[0].name.clone())
            }
            _ => {
                Err (
                    GetHangarVersionError::UnexpectedStatusCode {
                        slug: slug.to_string(),
                        status_code: status.into()
                    }.into()
                )
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::hangar::test::HangarTestServer;

    use speculoos::prelude::*;
    use wiremock::{Mock, ResponseTemplate};
    use wiremock::matchers::{method, path, query_param};

    #[tokio::test]
    async fn should_get_latest_project_version_name_from_api() -> Result<()> {
        // Arrange
        let hangar_server = HangarTestServer::new().await;

        let request = GetHangarVersionsRequest::create_request();

        let expected_version = "v1.2.3";
        let expected_response = GetHangarVersionsResponse {
            pagination: HangarResponsePagination {
                limit: 0,
                offset: 1,
                count: 10
            },
            result: vec![
                IncomingHangarVersion {
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
        let version = hangar_client.get_latest_hangar_project_version_from_api(slug).await;

        // Assert
        assert_that(&version).is_ok().is_equal_to(expected_version.to_string());

        Ok(())
    }

    #[tokio::test]
    async fn should_return_error_if_latest_project_version_is_not_found() -> Result<()> {
        // Arrange
        let hangar_server = HangarTestServer::new().await;

        let request = GetHangarVersionsRequest::create_request();

        let response = GetHangarVersionsResponse {
            pagination: HangarResponsePagination {
                limit: 0,
                offset: 1,
                count: 10
            },
            result: vec![]
        };
        let response_template = ResponseTemplate::new(200)
            .set_body_json(response);

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
        let result = hangar_client.get_latest_hangar_project_version_from_api(slug).await;

        // Assert
        assert_that(&result).is_err();

        let error = result.unwrap_err();
        let downcast_error = error.downcast_ref::<GetHangarVersionError>().unwrap();

        if let GetHangarVersionError::LatestVersionNotFound{slug} = downcast_error {
            assert_that(&slug).is_equal_to(slug);
        } else {
            panic!("expected error to be LatestVersionNotFound, but was {}", downcast_error);
        }

        Ok(())
    }
}