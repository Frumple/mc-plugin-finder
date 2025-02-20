use crate::HttpServer;
use crate::spigot::SpigotClient;
use mc_plugin_finder::database::ingest_log::{IngestLog, IngestLogAction, IngestLogRepository, IngestLogItem, insert_ingest_log};
use mc_plugin_finder::database::spigot::resource::{SpigotResource, upsert_spigot_resource, get_spigot_resources};

use anyhow::Result;
use deadpool_postgres::Pool;
use futures::stream::{self, StreamExt, TryStreamExt};
use reqwest::StatusCode;
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use thiserror::Error;
use time::OffsetDateTime;
use tracing::{info, warn, instrument};

const SPIGOT_VERSIONS_CONCURRENT_FUTURES: usize = 10;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
struct GetSpigotVersionResponse {
    name: String
}

#[derive(Debug, Error)]
enum GetSpigotVersionError {
    #[error("Resource ID {resource_id}: Latest version not found")]
    LatestVersionNotFound {
        resource_id: i32
    },
    #[error("Resource ID {resource_id}: Received unexpected status code {status_code}")]
    UnexpectedStatusCode {
        resource_id: i32,
        status_code: u16
    }
}

impl<T> SpigotClient<T> where T: HttpServer + Send + Sync {
    #[instrument(
        skip(self, db_pool)
    )]
    pub async fn populate_spigot_versions(&self, db_pool: &Pool) -> Result<()> {
        info!("Populating Spigot versions...");

        let count = Arc::new(AtomicU32::new(0));
        let date_started = OffsetDateTime::now_utc();

        let resources = get_spigot_resources(db_pool).await?;
        let resource_stream = stream::iter(resources);

        let result = resource_stream
            .map(Ok)
            .try_for_each_concurrent(SPIGOT_VERSIONS_CONCURRENT_FUTURES, |resource| self.process_spigot_resource(resource, db_pool, &count))
            .await;

        let date_finished = OffsetDateTime::now_utc();
        let items_processed = count.load(Ordering::Relaxed);

        let ingest_log = IngestLog {
            action: IngestLogAction::Populate,
            repository: IngestLogRepository::Spigot,
            item: IngestLogItem::Version,
            date_started,
            date_finished,
            items_processed: items_processed.try_into()?
        };
        insert_ingest_log(db_pool, &ingest_log).await?;

        info!("Spigot versions populated: {}", items_processed);

        result
    }

    async fn process_spigot_resource(&self, resource: SpigotResource, db_pool: &Pool, count: &Arc<AtomicU32>) -> Result<()> {
        let version_result = self.get_latest_spigot_resource_version_from_api(resource.id).await;

        match version_result {
            Ok(version_name) => {
                let mut new_resource = resource.clone();
                new_resource.version_name = Some(version_name);
                let db_result = upsert_spigot_resource(db_pool, &new_resource).await;

                match db_result {
                    Ok(_) => {
                        count.fetch_add(1, Ordering::Relaxed);
                    },
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
    pub async fn get_latest_spigot_resource_version_from_api(&self, resource_id: i32) -> Result<String> {
        self.rate_limiter.until_ready().await;

        let path = &["resources/", resource_id.to_string().as_str(), "/versions/latest"].concat();
        let url = self.http_server.base_url().join(path)?;

        let raw_response = self.api_client.get(url)
            .send()
            .await?;

        let status = raw_response.status();
        match status {
            StatusCode::OK => {
                let response: GetSpigotVersionResponse = raw_response.json().await?;
                Ok(response.name)
            }
            StatusCode::NOT_FOUND => {
                Err(
                    GetSpigotVersionError::LatestVersionNotFound {
                        resource_id
                    }.into()
                )
            }
            _ => {
                Err (
                    GetSpigotVersionError::UnexpectedStatusCode {
                        resource_id,
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
    use crate::spigot::test::SpigotTestServer;

    use speculoos::prelude::*;
    use wiremock::{Mock, ResponseTemplate};
    use wiremock::matchers::{method, path};

    #[derive(Clone, Serialize)]
    struct SpigotVersionErrorResponse {
        error: String
    }

    #[tokio::test]
    async fn should_get_latest_resource_version_name_from_api() -> Result<()> {
        // Arrange
        let spigot_server = SpigotTestServer::new().await;

        let expected_version_name = "v1.2.3";
        let expected_response = GetSpigotVersionResponse {
            name: expected_version_name.to_string()
        };
        let response_template = ResponseTemplate::new(200)
            .set_body_json(expected_response);

        let resource_id = 1;
        let api_path = &["/resources/", resource_id.to_string().as_str(), "/versions/latest"].concat();
        Mock::given(method("GET"))
            .and(path(api_path))
            .respond_with(response_template)
            .mount(spigot_server.mock())
            .await;

        // Act
        let spigot_client = SpigotClient::new(spigot_server)?;
        let result = spigot_client.get_latest_spigot_resource_version_from_api(resource_id).await;

        // Assert
        assert_that(&result).is_ok().is_equal_to(expected_version_name.to_string());

        Ok(())
    }

    #[tokio::test]
    async fn should_return_error_if_latest_resource_version_is_not_found() -> Result<()> {
        // Arrange
        let spigot_server = SpigotTestServer::new().await;

        let response = SpigotVersionErrorResponse {
            error: "version not found".to_string()
        };
        let response_template = ResponseTemplate::new(404)
            .set_body_json(response);

        let resource_id = 1;
        let api_path = &["/resources/", resource_id.to_string().as_str(), "/versions/latest"].concat();
        Mock::given(method("GET"))
            .and(path(api_path))
            .respond_with(response_template)
            .mount(spigot_server.mock())
            .await;

        // Act
        let spigot_client = SpigotClient::new(spigot_server)?;
        let result = spigot_client.get_latest_spigot_resource_version_from_api(resource_id).await;

        // Assert
        assert_that(&result).is_err();

        let error = result.unwrap_err();
        let downcast_error = error.downcast_ref::<GetSpigotVersionError>().unwrap();

        if let GetSpigotVersionError::LatestVersionNotFound{resource_id} = downcast_error {
            assert_that(&resource_id).is_equal_to(resource_id);
        } else {
            panic!("expected error to be LatestVersionNotFound, but was {}", downcast_error);
        }

        Ok(())
    }
}