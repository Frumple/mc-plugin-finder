use crate::HttpServer;
use crate::hangar::HangarClient;
use crate::hangar::project::HangarResponsePagination;
use mc_plugin_finder::database::hangar::project::{get_hangar_projects, upsert_hangar_project, HangarProject};

use anyhow::Result;
use deadpool_postgres::Pool;
use futures::stream::{self, StreamExt, TryStreamExt};
use reqwest::StatusCode;
use serde::{Serialize, Deserialize};
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use thiserror::Error;
use tracing::{info, warn, instrument};

const HANGAR_VERSIONS_CONCURRENT_FUTURES: usize = 10;

#[derive(Clone, Debug, Serialize)]
pub struct GetHangarVersionsRequest {
    limit: u32,
    offset: u32
}

impl GetHangarVersionsRequest {
    fn create_request() -> Self {
        Self {
            // Hangar's "projects/{slug}/versions" seems to order versions for newest to oldest.
            // This allows us to assume that the first version returned is the latest,
            // so we only get the first version in our request.
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
#[serde(rename_all = "camelCase")]
pub struct IncomingHangarVersion {
    name: String,
    platform_dependencies: IncomingHangarVersionProjectDependencies
    // TODO: Get visibility and channel?
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct IncomingHangarVersionProjectDependencies {
    paper: Option<Vec<String>>
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
        let count = Arc::new(AtomicU32::new(0));

        let projects = get_hangar_projects(db_pool).await?;
        let project_stream = stream::iter(projects);

        let result = project_stream
            .map(Ok)
            .try_for_each_concurrent(HANGAR_VERSIONS_CONCURRENT_FUTURES, |project| self.process_hangar_project(project, db_pool, &count))
            .await;

        info!("Hangar project versions populated: {}", count.load(Ordering::Relaxed));

        result
    }

    async fn process_hangar_project(&self, mut project: HangarProject, db_pool: &Pool, count: &Arc<AtomicU32>) -> Result<()> {
        let version_result = self.get_latest_hangar_project_version_from_api(&project.slug).await;

        match version_result {
            Ok(version) => {
                apply_incoming_hangar_version_to_hangar_project(&mut project, &version);

                let db_result = upsert_hangar_project(db_pool, &project).await;

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
    pub async fn get_latest_hangar_project_version_from_api(&self, slug: &str) -> Result<IncomingHangarVersion> {
        self.rate_limiter.until_ready().await;

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

                Ok(response.result[0].clone())
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

pub fn apply_incoming_hangar_version_to_hangar_project(project: &mut HangarProject, version: &IncomingHangarVersion) {
    project.version_name = Some(version.name.clone());

    // Paper versions from Hangar are in lexicographical order, meaning versions like "1.8" are considered later than versions like "1.21.3".
    // To get the proper latest version, we sort the versions in numerical order and get the last element.
    if let Some(paper) = &version.platform_dependencies.paper {
        let mut paper_versions = paper.clone();
        numeric_sort::sort(&mut paper_versions);
        project.latest_minecraft_version = paper_versions.last().cloned();
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::hangar::test::HangarTestServer;
    use mc_plugin_finder::database::source_repository::SourceRepository;

    use speculoos::prelude::*;
    use time::macros::datetime;
    use wiremock::{Mock, ResponseTemplate};
    use wiremock::matchers::{method, path, query_param};

    #[tokio::test]
    async fn should_get_latest_project_version_from_api() -> Result<()> {
        // Arrange
        let hangar_server = HangarTestServer::new().await;

        let request = GetHangarVersionsRequest::create_request();

        let expected_version = create_test_version();
        let expected_response = GetHangarVersionsResponse {
            pagination: HangarResponsePagination {
                limit: 1,
                offset: 0,
                count: 1
            },
            result: vec![expected_version.clone()]
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
        assert_that(&version).is_ok().is_equal_to(expected_version);

        Ok(())
    }

    #[tokio::test]
    async fn should_return_error_if_latest_project_version_is_not_found() -> Result<()> {
        // Arrange
        let hangar_server = HangarTestServer::new().await;

        let request = GetHangarVersionsRequest::create_request();

        let response = GetHangarVersionsResponse {
            pagination: HangarResponsePagination {
                limit: 1,
                offset: 0,
                count: 1
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

    #[tokio::test]
    async fn should_apply_incoming_hangar_version_to_hangar_project() -> Result<()> {
        // Arrange
        let mut project= create_test_project();

        let mut expected_project = project.clone();
        expected_project.version_name = Some("v1.2.3".to_string());
        expected_project.latest_minecraft_version = Some("1.21.3".to_string());

        let version = create_test_version();

        // Act
        apply_incoming_hangar_version_to_hangar_project(&mut project, &version);

        // Assert
        assert_that(&project).is_equal_to(expected_project);

        Ok(())
    }

    #[tokio::test]
    async fn should_apply_incoming_hangar_version_with_no_paper_versions_to_hangar_project() -> Result<()> {
        // Arrange
        let mut project= create_test_project();

        let mut expected_project = project.clone();
        expected_project.version_name = Some("v1.2.3".to_string());
        expected_project.latest_minecraft_version = None;

        let version = IncomingHangarVersion {
            name: "v1.2.3".to_string(),
            platform_dependencies: IncomingHangarVersionProjectDependencies {
                paper: None
            }
        };

        // Act
        apply_incoming_hangar_version_to_hangar_project(&mut project, &version);

        // Assert
        assert_that(&project).is_equal_to(expected_project);

        Ok(())
    }

    fn create_test_project() -> HangarProject {
        HangarProject {
            slug: "foo".to_string(),
            author: "alice".to_string(),
            name: "foo-hangar".to_string(),
            description: "foo-hangar-description".to_string(),
            date_created: datetime!(2022-01-01 0:00 UTC),
            date_updated: datetime!(2022-02-03 0:00 UTC),
            latest_minecraft_version: None,
            downloads: 100,
            stars: 200,
            watchers: 200,
            visibility: "public".to_string(),
            icon_url: "https://hangarcdn.papermc.io/avatars/project/1.webp?v=1".to_string(),
            version_name: None,
            source_url: Some("https://github.com/alice/foo".to_string()),
            source_repository: Some(SourceRepository {
                host: "github.com".to_string(),
                owner: "alice".to_string(),
                name: "foo".to_string()
            })
        }
    }

    pub fn create_test_version() -> IncomingHangarVersion {
        IncomingHangarVersion {
            name: "v1.2.3".to_string(),
            platform_dependencies: IncomingHangarVersionProjectDependencies {
                paper: Some(vec!["1.21.2".to_string(), "1.21.3".to_string(), "1.8".to_string(), "1.9".to_string()])
            }
        }
    }
}