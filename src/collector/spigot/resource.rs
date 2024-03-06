use crate::collector::HttpServer;
use crate::collector::spigot::SpigotClient;
use crate::collector::util::extract_source_repository_from_url;
use crate::database::spigot::resource::{SpigotResource, get_latest_spigot_resource_update_date, upsert_spigot_resource};

use anyhow::Result;
use deadpool_postgres::Client;
use futures::future;
use futures::stream::TryStreamExt;
use page_turner::prelude::*;
use regex::Regex;
use serde::{Serialize, Deserialize};
use time::OffsetDateTime;
use std::cell::Cell;
use std::fmt::Debug;
use thiserror::Error;
use tracing::{info, warn, instrument};

const SPIGOT_RESOURCES_REQUEST_FIELDS: &str = "id,name,tag,releaseDate,updateDate,file,author,version,premium,sourceCodeLink";
const SPIGOT_POPULATE_RESOURCES_REQUESTS_AHEAD: usize = 2;

#[derive(Clone, Debug, Serialize)]
struct GetSpigotResourcesRequest {
    size: u32,
    page: u32,
    sort: String,
    fields: String
}

impl GetSpigotResourcesRequest {
    fn create_populate_request() -> Self {
        Self {
            size: 1000,
            page: 1,
            sort: "+id".to_string(),
            fields: SPIGOT_RESOURCES_REQUEST_FIELDS.to_string()
        }
    }

    fn create_update_request() -> Self {
        Self {
            size: 100,
            page: 1,
            sort: "-updateDate".to_string(),
            fields: SPIGOT_RESOURCES_REQUEST_FIELDS.to_string()
        }
    }
}

impl RequestAhead for GetSpigotResourcesRequest {
    fn next_request(&self) -> Self {
        Self {
            size: self.size,
            page: self.page + 1,
            sort: self.sort.clone(),
            fields: self.fields.clone()
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
struct GetSpigotResourcesResponse {
    headers: GetSpigotResourcesResponseHeaders,
    resources: Vec<IncomingSpigotResource>
}

impl GetSpigotResourcesResponse {
    fn more_resources_available(&self) -> bool {
        self.headers.x_page_index <= self.headers.x_page_count
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
struct GetSpigotResourcesResponseHeaders {
    x_page_index: u32,
    x_page_count: u32
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IncomingSpigotResource {
    id: i32,
    name: String,
    tag: String,
    release_date: i64,
    update_date: i64,
    file: Option<IncomingSpigotResourceNestedFile>,
    author: IncomingSpigotResourceNestedAuthor,
    version: IncomingSpigotResourceNestedVersion,
    premium: Option<bool>,
    source_code_link: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct IncomingSpigotResourceNestedFile {
    url: String
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct IncomingSpigotResourceNestedAuthor {
    id: i32
}


#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct IncomingSpigotResourceNestedVersion {
    id: i32
}

#[derive(Debug, Error)]
enum IncomingSpigotResourceError {
    #[error("Skipping resource ID {resource_id}: Invalid slug from URL: {url}")]
    InvalidSlugFromURL {
        resource_id: i32,
        url: String
    },
    #[error("Skipping resource ID {resource_id}: File does not exist")]
    FileDoesNotExist {
        resource_id: i32
    }
}

// #[derive(Clone, Debug, Serialize)]
// struct GetLatestResourceVersionRequest {
//     resource: i32
// }

// #[derive(Debug, Deserialize)]
// struct SpigotResourceVersion {
//     name: String
// }

impl<T> SpigotClient<T> where T: HttpServer + Send + Sync {
    #[instrument(
        skip(self, db_client)
    )]
    pub async fn populate_spigot_resources(&self, db_client: &Client) -> Result<()> {
        let request = GetSpigotResourcesRequest::create_populate_request();

        let count_cell: Cell<u32> = Cell::new(0);

        let result = self
            .pages_ahead(SPIGOT_POPULATE_RESOURCES_REQUESTS_AHEAD, Limit::None, request)
            .items()
            .try_for_each_concurrent(None, |incoming_resource| process_incoming_resource(incoming_resource, db_client, &count_cell))
            .await;

        let count = count_cell.get();
        info!("Spigot resources populated: {}", count);

        result
    }

    #[instrument(
        skip(self, db_client)
    )]
    pub async fn update_spigot_resources(&self, db_client: &Client, update_date_later_than: OffsetDateTime) -> Result<()> {
        let request = GetSpigotResourcesRequest::create_update_request();

        let count_cell: Cell<u32> = Cell::new(0);

        let result = self
            .pages(request)
            .items()
            .try_take_while(|x| future::ready(Ok(x.update_date > update_date_later_than.unix_timestamp())))
            .try_for_each(|incoming_resource| process_incoming_resource(incoming_resource, db_client, &count_cell))
            .await;

        let count = count_cell.get();
        info!("Spigot resources updated: {}", count);

        result
    }

    #[instrument(
        skip(self)
    )]
    async fn get_resources_from_api(&self, request: GetSpigotResourcesRequest) -> Result<GetSpigotResourcesResponse> {
        self.rate_limiter.until_ready().await;

        let url = self.http_server.base_url().join("resources")?;
        let raw_response = self.api_client.get(url)
            .query(&request)
            .send()
            .await?;

        let raw_headers = raw_response.headers();
        let headers = GetSpigotResourcesResponseHeaders {
            // TODO: Convert from string to int using serde_aux::field_attributes::deserialize_number_from_string
            x_page_index: raw_headers["x-page-index"].to_str()?.parse::<u32>()?,
            x_page_count: raw_headers["x-page-count"].to_str()?.parse::<u32>()?,
        };

        let resources: Vec<IncomingSpigotResource> = raw_response.json().await?;

        let response = GetSpigotResourcesResponse {
            headers,
            resources
        };

        Ok(response)
    }

    // async fn get_latest_resource_version_name(&self, request: GetLatestResourceVersionRequest) -> Result<String> {
    //     self.rate_limiter.until_ready().await;

    //     let resource_id = request.resource;
    //     let url = format!("{SPIGOT_BASE_URL}/resources/{resource_id}/versions/latest");

    //     let raw_response = self.api_client.get(url)
    //         .send()
    //         .await?;

    //     let version: SpigotResourceVersion = raw_response.json().await?;

    //     Ok(version.name)
    // }
}

impl<T> PageTurner<GetSpigotResourcesRequest> for SpigotClient<T> where T: HttpServer + Send + Sync {
    type PageItems = Vec<IncomingSpigotResource>;
    type PageError = anyhow::Error;

  async fn turn_page(&self, mut request: GetSpigotResourcesRequest) -> TurnedPageResult<Self, GetSpigotResourcesRequest> {
        let response = self.get_resources_from_api(request.clone()).await?;

        if response.more_resources_available() {
            request.page += 1;
            Ok(TurnedPage::next(response.resources, request))
        } else {
            Ok(TurnedPage::last(response.resources))
        }
    }
}

async fn process_incoming_resource(incoming_resource: IncomingSpigotResource, db_client: &Client, count_cell: &Cell<u32>) -> Result<()> {
    // let latest_resource_version_request = GetLatestResourceVersionRequest { resource: resource.id };
    // let latest_resource_version_name = self.get_latest_resource_version_name(latest_resource_version_request).await?;

    let process_result = convert_incoming_resource(incoming_resource).await;

    match process_result {
        Ok(resource) => {
            let db_result = upsert_spigot_resource(db_client, resource).await;

            match db_result {
                Ok(_) => count_cell.set(count_cell.get() + 1),
                Err(err) => warn!("{}", err)
            }
        }
        Err(err) => warn!("{}", err)
    }
    Ok(())
}

async fn convert_incoming_resource(incoming_resource: IncomingSpigotResource) -> Result<SpigotResource> {
    let resource_id = incoming_resource.id;

    if let Some(file) = incoming_resource.file {
        if let Some(slug) = extract_slug_from_file_download_url(&file.url) {
            let mut resource = SpigotResource {
                id: incoming_resource.id,
                name: incoming_resource.name,
                tag: incoming_resource.tag,
                slug,
                release_date: OffsetDateTime::from_unix_timestamp(incoming_resource.release_date)?,
                update_date: OffsetDateTime::from_unix_timestamp(incoming_resource.update_date)?,
                author_id: incoming_resource.author.id,
                version_id: incoming_resource.version.id,
                version_name: None::<String>,
                premium: incoming_resource.premium,
                source_code_link: incoming_resource.source_code_link.clone(),
                source_repository_host: None,
                source_repository_owner: None,
                source_repository_name: None
            };

            if let Some(url) = incoming_resource.source_code_link {
                let option_repo = extract_source_repository_from_url(url.as_str());

                if let Some(repo) = option_repo {
                    resource.source_repository_host = Some(repo.host);
                    resource.source_repository_owner = Some(repo.owner);
                    resource.source_repository_name = Some(repo.name);
                }
            }

            Ok(resource)
        } else {
            Err(
                IncomingSpigotResourceError::InvalidSlugFromURL {
                    resource_id,
                    url: file.url
                }.into()
            )
        }
    } else {
        Err(
            IncomingSpigotResourceError::FileDoesNotExist {
                resource_id
            }.into()
        )
    }
}

fn extract_slug_from_file_download_url(url: &str) -> Option<String> {
    let re = Regex::new(r"resources/(\S+\.\d+)/download.*").unwrap();
    let caps = re.captures(url)?;
    Some(caps[1].to_string())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::collector::spigot::test::SpigotTestServer;

    use rstest::*;
    use speculoos::prelude::*;
    use time::macros::datetime;
    use wiremock::{Mock, ResponseTemplate};
    use wiremock::matchers::{method, path, query_param};

    #[rstest]
    #[case::slug_single_word("resources/foo.1/download?version=1", "foo.1")]
    #[case::slug_with_dashes("resources/foo-bar-baz.1/download?version=1", "foo-bar-baz.1")]
    #[case::slug_with_special_character("resources/%C2%BB-foo.1/download?version=1", "%C2%BB-foo.1")]
    fn should_extract_slug_from_url(#[case] url: &str, #[case] expected_slug: &str) {
        let slug = extract_slug_from_file_download_url(url);
        assert_that(&slug).is_some().is_equal_to(expected_slug.to_string());
    }

    #[test]
    fn should_not_extract_slug_if_file_download_url_has_no_name() {
        let url = "resources/1/download?version=1";
        let slug = extract_slug_from_file_download_url(url);
        assert_that(&slug).is_none();
    }

    #[tokio::test]
    async fn should_get_resources_from_api() -> Result<()> {
        // Arrange
        let spigot_server = SpigotTestServer::new().await;

        let request = GetSpigotResourcesRequest::create_populate_request();

        let expected_response = GetSpigotResourcesResponse {
            headers: GetSpigotResourcesResponseHeaders {
                x_page_index: 1,
                x_page_count: 10
            },
            resources: create_test_resources()
        };

        let response_template = ResponseTemplate::new(200)
            .append_header("x-page-index", expected_response.headers.x_page_index.to_string().as_str())
            .append_header("x-page-count", expected_response.headers.x_page_count.to_string().as_str())
            .set_body_json(expected_response.resources.clone());

            Mock::given(method("GET"))
                .and(path("/resources"))
                .and(query_param("size", request.size.to_string().as_str()))
                .and(query_param("page", expected_response.headers.x_page_index.to_string().as_str()))
                .and(query_param("sort", request.sort.as_str()))
                .and(query_param("fields", SPIGOT_RESOURCES_REQUEST_FIELDS))
                .respond_with(response_template)
                .mount(spigot_server.mock())
                .await;

        // Act
        let spigot_client = SpigotClient::new(spigot_server)?;
        let response = spigot_client.get_resources_from_api(request).await?;

        // Assert
        assert_that(&response).is_equal_to(expected_response);

        Ok(())
    }

    #[tokio::test]
    async fn should_process_incoming_resource() -> Result<()> {
        // Arrange
        let incoming_resource = create_test_resources()[0].clone();

        // Act
        let resource = convert_incoming_resource(incoming_resource).await?;

        // Assert
        assert_that(&resource.id).is_equal_to(1);
        assert_that(&resource.name).is_equal_to("resource-1".to_string());
        assert_that(&resource.tag).is_equal_to("resource-1-tag".to_string());
        assert_that(&resource.slug).is_equal_to("foo.1".to_string());
        assert_that(&resource.release_date).is_equal_to(datetime!(2020-01-01 0:00 UTC));
        assert_that(&resource.update_date).is_equal_to(datetime!(2021-01-01 0:00 UTC));
        assert_that(&resource.author_id).is_equal_to(1);
        assert_that(&resource.version_id).is_equal_to(1);
        assert_that(&resource.premium).is_some().is_false();
        assert_that(&resource.source_code_link).is_some().is_equal_to("https://github.com/Frumple/foo".to_string());
        assert_that(&resource.source_repository_host).is_some().is_equal_to("github.com".to_string());
        assert_that(&resource.source_repository_owner).is_some().is_equal_to("Frumple".to_string());
        assert_that(&resource.source_repository_name).is_some().is_equal_to("foo".to_string());

        Ok(())
    }

    #[tokio::test]
    async fn should_not_process_resource_with_invalid_slug() -> Result<()> {
        // Arrange
        let mut incoming_resource: IncomingSpigotResource = create_test_resources()[0].clone();
        incoming_resource.file = Some(IncomingSpigotResourceNestedFile {
            url: "resources/1/download?version=1".to_string()
        });

        // Act
        let result = convert_incoming_resource(incoming_resource).await;
        let error = result.unwrap_err();

        // Assert
        assert!(matches!(error.downcast_ref::<IncomingSpigotResourceError>(), Some(IncomingSpigotResourceError::InvalidSlugFromURL { .. })));

        Ok(())
    }

    #[tokio::test]
    async fn should_not_process_resource_with_no_file() -> Result<()> {
        // Arrange
        let mut incoming_resource = create_test_resources()[0].clone();
        incoming_resource.file = None;

        // Act
        let result = convert_incoming_resource(incoming_resource).await;
        let error = result.unwrap_err();

        // Assert
        assert!(matches!(error.downcast_ref::<IncomingSpigotResourceError>(), Some(IncomingSpigotResourceError::FileDoesNotExist { .. })));

        Ok(())
    }

    fn create_test_resources() -> Vec<IncomingSpigotResource> {
        vec![
            IncomingSpigotResource {
                id: 1,
                name: "resource-1".to_string(),
                tag: "resource-1-tag".to_string(),
                release_date: 1577836800,
                update_date: 1609459200,
                file: Some(IncomingSpigotResourceNestedFile {
                    url: "resources/foo.1/download?version=1".to_string()
                }),
                author: IncomingSpigotResourceNestedAuthor {
                    id: 1
                },
                version: IncomingSpigotResourceNestedVersion {
                    id: 1
                },
                premium: Some(false),
                source_code_link: Some("https://github.com/Frumple/foo".to_string())
            },
            IncomingSpigotResource {
                id: 2,
                name: "resource-2".to_string(),
                tag: "resource-2-tag".to_string(),
                release_date: 1577836800,
                update_date: 1640995200,
                file: Some(IncomingSpigotResourceNestedFile {
                    url: "resources/bar.2/download?version=2".to_string()
                }),
                author: IncomingSpigotResourceNestedAuthor {
                    id: 2
                },
                version: IncomingSpigotResourceNestedVersion {
                    id: 2
                },
                premium: Some(false),
                source_code_link: Some("https://gitlab.com/Frumple/bar".to_string())
            }
        ]
    }
}