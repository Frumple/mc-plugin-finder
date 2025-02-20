use crate::HttpServer;
use crate::modrinth::ModrinthClient;
use mc_plugin_finder::database::ingest_log::{IngestLog, IngestLogAction, IngestLogRepository, IngestLogItem, insert_ingest_log};
use mc_plugin_finder::database::modrinth::project::{get_modrinth_projects, upsert_modrinth_project, ModrinthProject};

use anyhow::Result;
use deadpool_postgres::Pool;
use futures::stream::{self, StreamExt, TryStreamExt};
use reqwest::StatusCode;
use serde::{Serialize, Deserialize};
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use thiserror::Error;
use time::OffsetDateTime;
use tracing::{info, warn, instrument};

const MODRINTH_VERSIONS_CONCURRENT_FUTURES: usize = 10;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct GetModrinthVersionResponse {
    version_number: String
}

#[derive(Debug, Error)]
enum GetModrinthVersionError {
    #[error("Project '{project_id}' and version '{version_id}': Latest version not found.")]
    LatestVersionNotFound {
        project_id: String,
        version_id: String
    },
    #[error("Project '{project_id}' and version '{version_id}': Received unexpected status code {status_code}")]
    UnexpectedStatusCode {
        project_id: String,
        version_id: String,
        status_code: u16
    }
}

impl<T> ModrinthClient<T> where T: HttpServer + Send + Sync {
    #[instrument(
        skip(self, db_pool)
    )]
    pub async fn populate_modrinth_versions(&self, db_pool: &Pool) -> Result<()> {
        info!("Populating Modrinth versions...");

        let count = Arc::new(AtomicU32::new(0));
        let date_started = OffsetDateTime::now_utc();

        let projects = get_modrinth_projects(db_pool).await?;
        let project_stream = stream::iter(projects);

        let result = project_stream
            .map(Ok)
            .try_for_each_concurrent(MODRINTH_VERSIONS_CONCURRENT_FUTURES, |project| self.process_modrinth_project(project, db_pool, &count))
            .await;

        let date_finished = OffsetDateTime::now_utc();
        let items_processed = count.load(Ordering::Relaxed);

        let ingest_log = IngestLog {
            action: IngestLogAction::Populate,
            repository: IngestLogRepository::Modrinth,
            item: IngestLogItem::Version,
            date_started,
            date_finished,
            items_processed: items_processed.try_into()?
        };
        insert_ingest_log(db_pool, &ingest_log).await?;

        info!("Modrinth versions populated: {}", items_processed);

        result
    }

    async fn process_modrinth_project(&self, project: ModrinthProject, db_pool: &Pool, count: &Arc<AtomicU32>) -> Result<()> {
        if let Some(ref version_id) = project.version_id {
            let version_result = self.get_latest_modrinth_project_version_from_api(&project.id, version_id).await;

            match version_result {
                Ok(version_name) => {
                    let mut new_project = project.clone();
                    new_project.version_name = Some(version_name);
                    let db_result = upsert_modrinth_project(db_pool, &new_project).await;

                    match db_result {
                        Ok(_) => {
                            count.fetch_add(1, Ordering::Relaxed);
                        },
                        Err(err) => warn!("{}", err)
                    }
                }
                Err(err) => warn!("{}", err)
            }
        } else {
            warn!("Skipping project '{}': Version ID not found.", project.id);
        }

        Ok(())
    }

    #[instrument(
        skip(self)
    )]
    pub async fn get_latest_modrinth_project_version_from_api(&self, project_id: &str, version_id: &str) -> Result<String> {
        self.rate_limiter.until_ready().await;

        let path = &["version/", version_id].concat();
        let url = self.http_server.base_url().join(path)?;

        let raw_response = self.api_client.get(url)
            .send()
            .await?;

        let status = raw_response.status();
        match status {
            StatusCode::OK => {
                let response: GetModrinthVersionResponse = raw_response.json().await?;
                Ok(response.version_number)
            }
            StatusCode::NOT_FOUND => {
                Err(
                    GetModrinthVersionError::LatestVersionNotFound {
                        project_id: project_id.to_string(),
                        version_id: version_id.to_string()
                    }.into()
                )
            }
            _ => {
                Err(
                    GetModrinthVersionError::UnexpectedStatusCode {
                        project_id: project_id.to_string(),
                        version_id: version_id.to_string(),
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
    use crate::modrinth::test::ModrinthTestServer;

    use speculoos::prelude::*;
    use wiremock::{Mock, ResponseTemplate};
    use wiremock::matchers::{method, path};

    #[derive(Clone, Serialize)]
    struct ModrinthVersionErrorResponse;

    #[tokio::test]
    async fn should_get_latest_project_version_name_from_api() -> Result<()> {
        // Arrange
        let modrinth_server = ModrinthTestServer::new().await;

        let expected_version_name = "v1.2.3";
        let expected_response = GetModrinthVersionResponse {
            version_number: expected_version_name.to_string()
        };
        let response_template = ResponseTemplate::new(200)
            .set_body_json(expected_response);

        let project_id = "aaaaaaaa";
        let version_id = "aaaa1111";
        let api_path = &["/version/", version_id].concat();
        Mock::given(method("GET"))
            .and(path(api_path))
            .respond_with(response_template)
            .mount(modrinth_server.mock())
            .await;

        // Act
        let modrinth_client = ModrinthClient::new(modrinth_server)?;
        let result = modrinth_client.get_latest_modrinth_project_version_from_api(project_id, version_id).await;

        // Assert
        assert_that(&result).is_ok().is_equal_to(expected_version_name.to_string());

        Ok(())
    }

    #[tokio::test]
    async fn should_return_error_if_latest_project_version_is_not_found() -> Result<()> {
        // Arrange
        let modrinth_server = ModrinthTestServer::new().await;

        let response = ModrinthVersionErrorResponse;
        let response_template = ResponseTemplate::new(404)
            .set_body_json(response);

        let expected_project_id = "aaaaaaaa";
        let expected_version_id = "aaaa1111";
        let api_path = &["/version/", expected_version_id].concat();
        Mock::given(method("GET"))
            .and(path(api_path))
            .respond_with(response_template)
            .mount(modrinth_server.mock())
            .await;

        // Act
        let modrinth_client = ModrinthClient::new(modrinth_server)?;
        let result = modrinth_client.get_latest_modrinth_project_version_from_api(expected_project_id, expected_version_id).await;

        // Assert
        assert_that(&result).is_err();

        let error = result.unwrap_err();
        let downcast_error = error.downcast_ref::<GetModrinthVersionError>().unwrap();

        if let GetModrinthVersionError::LatestVersionNotFound{project_id, version_id} = downcast_error {
            assert_that(&project_id).is_equal_to(project_id);
            assert_that(&version_id).is_equal_to(version_id);
        } else {
            panic!("expected error to be LatestVersionNotFound, but was {}", downcast_error);
        }

        Ok(())
    }
}