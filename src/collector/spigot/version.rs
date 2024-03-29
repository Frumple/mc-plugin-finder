use crate::collector::HttpServer;
use crate::collector::spigot::SpigotClient;
use crate::database::spigot::resource::{SpigotResource, upsert_spigot_resource, get_spigot_resources};

use anyhow::Result;
use deadpool_postgres::Pool;
use futures::stream::{self, StreamExt, TryStreamExt};
use serde::{Serialize, Deserialize};
use std::cell::Cell;
use thiserror::Error;
use tracing::{info, warn, instrument};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
struct GetSpigotResourceVersionResponse {
    name: Option<String>
}

#[derive(Debug, Error)]
enum IncomingSpigotResourceVersionError {
    #[error("Resource ID {resource_id}: Latest version not found")]
    LatestVersionNotFound {
        resource_id: i32
    }
}

impl<T> SpigotClient<T> where T: HttpServer + Send + Sync {
    #[instrument(
        skip(self, db_pool)
    )]
    pub async fn populate_spigot_resource_versions(&self, db_pool: &Pool) -> Result<()> {
        let count_cell: Cell<u32> = Cell::new(0);

        let resources = get_spigot_resources(db_pool).await?;
        let resource_stream = stream::iter(resources);

        let result = resource_stream
            .map(Ok)
            .try_for_each_concurrent(None, |resource| self.process_spigot_resource(resource, db_pool, &count_cell))
            .await;

        let count = count_cell.get();
        info!("Spigot resource versions populated: {}", count);

        result
    }

    async fn process_spigot_resource(&self, resource: SpigotResource, db_pool: &Pool, count_cell: &Cell<u32>) -> Result<()> {
        let version_result = self.get_latest_spigot_resource_version_from_api(resource.id).await;

        match version_result {
            Ok(version_name) => {
                let mut new_resource = resource.clone();
                new_resource.version_name = Some(version_name);
                let db_result = upsert_spigot_resource(db_pool, &new_resource).await;

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
    pub async fn get_latest_spigot_resource_version_from_api(&self, resource_id: i32) -> Result<String> {
        self.rate_limiter.until_ready().await;

        let path = &["resources/", resource_id.to_string().as_str(), "/versions/latest"].concat();
        let url = self.http_server.base_url().join(path)?;

        let raw_response = self.api_client.get(url)
            .send()
            .await?;

        let response: GetSpigotResourceVersionResponse = raw_response.json().await?;

        if let Some(version_name) = response.name {
            Ok(version_name)
        } else {
            Err(
                IncomingSpigotResourceVersionError::LatestVersionNotFound {
                    resource_id
                }.into()
            )
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::collector::spigot::test::SpigotTestServer;

    use speculoos::prelude::*;
    use wiremock::{Mock, ResponseTemplate};
    use wiremock::matchers::{method, path};

    #[derive(Clone, Serialize)]
    struct SpigotResourceVersionErrorResponse {
        error: String
    }

    #[tokio::test]
    async fn should_get_latest_resource_version_name_from_api() -> Result<()> {
        // Arrange
        let spigot_server = SpigotTestServer::new().await;

        let expected_version_name = "v1.2.3";
        let expected_response = GetSpigotResourceVersionResponse {
            name: Some(expected_version_name.to_string())
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

        let response = SpigotResourceVersionErrorResponse {
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
        let downcast_error = error.downcast_ref::<IncomingSpigotResourceVersionError>().unwrap();

        if let IncomingSpigotResourceVersionError::LatestVersionNotFound{resource_id} = downcast_error {
            assert_that(&resource_id).is_equal_to(resource_id);
        } else {
            panic!("expected error to be LatestVersionNotFound, but was {}", downcast_error);
        }

        Ok(())
    }
}