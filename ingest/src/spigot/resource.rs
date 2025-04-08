pub mod name;

use crate::HttpServer;
use crate::spigot::SpigotClient;
use crate::spigot::resource::name::{ABANDONMENT_REGEX, parse_spigot_resource_name};
use mc_plugin_finder::database::ingest_log::{IngestLog, IngestLogAction, IngestLogRepository, IngestLogItem, insert_ingest_log};
use mc_plugin_finder::database::spigot::resource::{SpigotResource, upsert_spigot_resource};
use mc_plugin_finder::database::source_repository::{SourceRepository, extract_source_repository_from_url};

use anyhow::Result;
use deadpool_postgres::Pool;
use futures::future;
use futures::stream::TryStreamExt;
use page_turner::prelude::*;
use regex::Regex;
use reqwest::StatusCode;
use serde::{Serialize, Deserialize};
use std::fmt::Debug;
use std::sync::{Arc, LazyLock};
use std::sync::atomic::{AtomicU32, Ordering};
use thiserror::Error;
use time::OffsetDateTime;
use tracing::{info, warn, instrument};

const SPIGOT_RESOURCES_REQUEST_FIELDS: &str = "id,name,tag,icon,releaseDate,updateDate,testedVersions,downloads,likes,file,author,version,premium,sourceCodeLink";
const SPIGOT_RESOURCES_REQUESTS_AHEAD: usize = 2;
const SPIGOT_RESOURCES_CONCURRENT_FUTURES: usize = 10;

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
            size: 500,
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
    icon: Option<IncomingSpigotResourceNestedIcon>,
    release_date: i64,
    update_date: i64,
    tested_versions: Option<Vec<String>>,
    downloads: i32,
    likes: Option<i32>,
    file: Option<IncomingSpigotResourceNestedFile>,
    author: IncomingSpigotResourceNestedAuthor,
    version: IncomingSpigotResourceNestedVersion,
    premium: Option<bool>,
    source_code_link: Option<String>,
}

impl IncomingSpigotResource {
    fn is_abandoned(&self) -> bool {
        let re = &*ABANDONMENT_REGEX;
        re.is_match(&self.name)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct IncomingSpigotResourceNestedIcon {
    url: String,
    data: String
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
enum GetSpigotResourcesError {
    #[error("Could not get Spigot resources {request:?}: Received unexpected status code {status_code}")]
    UnexpectedStatusCode {
        request: GetSpigotResourcesRequest,
        status_code: u16
    }
}

#[derive(Debug, Error)]
enum ConvertIncomingSpigotResourceError {
    #[error("Skipping resource ID {resource_id}: Invalid slug from URL: {url}")]
    InvalidSlugFromURL {
        resource_id: i32,
        url: String
    },
    #[error("Skipping resource ID {resource_id}: File not found")]
    FileNotFound {
        resource_id: i32
    }
}

impl<T> SpigotClient<T> where T: HttpServer + Send + Sync {
    #[instrument(
        skip(self, db_pool)
    )]
    pub async fn populate_spigot_resources(&self, db_pool: &Pool) -> Result<()> {
        info!("Populating Spigot resources...");

        let request = GetSpigotResourcesRequest::create_populate_request();
        let count = Arc::new(AtomicU32::new(0));
        let date_started = OffsetDateTime::now_utc();

        let result = self
            .pages_ahead(SPIGOT_RESOURCES_REQUESTS_AHEAD, Limit::None, request)
            .items()
            .try_for_each_concurrent(SPIGOT_RESOURCES_CONCURRENT_FUTURES, |incoming_resource| self.process_incoming_resource(incoming_resource, db_pool, &count, false))
            .await;

        let date_finished = OffsetDateTime::now_utc();
        let items_processed = count.load(Ordering::Relaxed);

        let ingest_log = IngestLog {
            action: IngestLogAction::Populate,
            repository: IngestLogRepository::Spigot,
            item: IngestLogItem::Resource,
            date_started,
            date_finished,
            items_processed: items_processed.try_into()?,
            success: result.is_ok()
        };
        insert_ingest_log(db_pool, &ingest_log).await?;

        info!("Spigot resources populated: {}", items_processed);

        result
    }

    #[instrument(
        skip(self, db_pool)
    )]
    pub async fn update_spigot_resources(&self, db_pool: &Pool, update_date_later_than: OffsetDateTime) -> Result<()> {
        info!("Updating Spigot resources since: {}", update_date_later_than);

        let request = GetSpigotResourcesRequest::create_update_request();
        let count = Arc::new(AtomicU32::new(0));
        let date_started = OffsetDateTime::now_utc();

        let result = self
            .pages_ahead(SPIGOT_RESOURCES_REQUESTS_AHEAD, Limit::None, request)
            .items()
            .try_take_while(|x| future::ready(Ok(x.update_date > update_date_later_than.unix_timestamp())))
            .try_for_each_concurrent(SPIGOT_RESOURCES_CONCURRENT_FUTURES, |incoming_resource| self.process_incoming_resource(incoming_resource, db_pool, &count, true))
            .await;

        let date_finished = OffsetDateTime::now_utc();
        let items_processed = count.load(Ordering::Relaxed);

        let ingest_log = IngestLog {
            action: IngestLogAction::Update,
            repository: IngestLogRepository::Spigot,
            item: IngestLogItem::Resource,
            date_started,
            date_finished,
            items_processed: items_processed.try_into()?,
            success: result.is_ok()
        };
        insert_ingest_log(db_pool, &ingest_log).await?;

        info!("Spigot resources updated: {}", items_processed);

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

        let status = raw_response.status();
        if status == StatusCode::OK {
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
        } else {
            Err(
                GetSpigotResourcesError::UnexpectedStatusCode {
                    request,
                    status_code: status.into()
                }.into()
            )
        }
    }

    async fn process_incoming_resource(&self, incoming_resource: IncomingSpigotResource, db_pool: &Pool, count: &Arc<AtomicU32>, get_version: bool) -> Result<()> {
        let mut version_name = None;

        if get_version {
            let version_result = self.get_latest_spigot_resource_version_from_api(incoming_resource.id).await;

            match version_result {
                Ok(retrieved_version_name) => version_name = Some(retrieved_version_name),
                Err(err) => warn!("{}", err)
            }
        }

        let convert_result = convert_incoming_resource(incoming_resource, &version_name).await;

        match convert_result {
            Ok(resource) => {
                let db_result = upsert_spigot_resource(db_pool, &resource).await;

                match db_result {
                    Ok(_) => {
                        count.fetch_add(1, Ordering::Relaxed);
                    }
                    Err(err) => warn!("{}", err)
                }
            }
            Err(err) => warn!("{}", err)
        }

        Ok(())
    }
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

async fn convert_incoming_resource(incoming_resource: IncomingSpigotResource, version_name: &Option<String>) -> Result<SpigotResource> {
    let resource_id = incoming_resource.id;

    if let Some(ref file) = incoming_resource.file {
        if let Some(slug) = extract_slug_from_file_download_url(&file.url) {
            let parsed_name = parse_spigot_resource_name(&incoming_resource.name);
            let abandoned = incoming_resource.is_abandoned();

            let mut resource = SpigotResource {
                id: incoming_resource.id,
                name: incoming_resource.name,
                parsed_name,
                description: incoming_resource.tag,
                slug,
                date_created: OffsetDateTime::from_unix_timestamp(incoming_resource.release_date)?,
                date_updated: OffsetDateTime::from_unix_timestamp(incoming_resource.update_date)?,

                // "testedVersions" may not exist in the API response, default to an empty vec if this is the case.
                // Assume that the last entry in the given list of versions from the API is the latest version.
                latest_minecraft_version: incoming_resource.tested_versions.unwrap_or_default().last().cloned(),
                downloads: incoming_resource.downloads,

                // "likes" may not exist in the API response, default to 0 if this is the case.
                likes: incoming_resource.likes.unwrap_or_default(),

                author_id: incoming_resource.author.id,
                version_id: incoming_resource.version.id,
                version_name: version_name.clone(),

                // "premium" may not exist in the API response, default to false if this is the case.
                premium: incoming_resource.premium.unwrap_or_default(),

                // "abandoned" is true if the resource name contains a keyword that indicates abandonment.
                abandoned,

                // "icon" may not exist in the API response, set "icon_url" and "icon_data" to None if this is the case.
                icon_url: incoming_resource.icon.as_ref().map(|icon| icon.url.clone()),
                icon_data: incoming_resource.icon.map(|icon| icon.data),

                source_url: incoming_resource.source_code_link.clone(),
                source_repository: None
            };

            if let Some(url) = incoming_resource.source_code_link {
                let option_repo = extract_source_repository_from_url(url.as_str());

                if let Some(repo) = option_repo {
                    resource.source_repository = Some(SourceRepository {
                        host: repo.host,
                        owner: repo.owner,
                        name: repo.name,
                        id: None
                    });
                }
            }

            Ok(resource)
        } else {
            Err(
                ConvertIncomingSpigotResourceError::InvalidSlugFromURL {
                    resource_id,
                    url: file.url.clone()
                }.into()
            )
        }
    } else {
        Err(
            ConvertIncomingSpigotResourceError::FileNotFound {
                resource_id
            }.into()
        )
    }
}



static SLUG_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"resources/(\S+\.\d+)/download.*").unwrap());

fn extract_slug_from_file_download_url(url: &str) -> Option<String> {
    let re = &*SLUG_REGEX;
    let caps = re.captures(url)?;
    Some(caps[1].to_string())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::spigot::test::SpigotTestServer;
    use mc_plugin_finder::database::spigot::test::SPIGOT_BASE64_TEST_ICON_DATA;

    use rstest::*;
    use speculoos::prelude::*;
    use time::macros::datetime;
    use wiremock::{Mock, ResponseTemplate};
    use wiremock::matchers::{method, path, query_param};

    #[rstest]
    #[case::slug_single_word("resources/foo.1/download?version=1", "foo.1")]
    #[case::slug_with_hyphens("resources/foo-bar-baz.1/download?version=1", "foo-bar-baz.1")]
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
        let response = spigot_client.get_resources_from_api(request).await;

        // Assert
        assert_that(&response).is_ok().is_equal_to(expected_response);

        Ok(())
    }

    #[tokio::test]
    async fn should_convert_incoming_resource() -> Result<()> {
        // Arrange
        let incoming_resource = create_test_resources()[0].clone();
        let version_name = "v1.2.3";

        // Act
        let resource = convert_incoming_resource(incoming_resource, &Some(version_name.to_string())).await?;

        // Assert
        let expected_resource = SpigotResource {
            id: 1,
            name: "foo-spigot".to_string(),
            parsed_name: Some("foo-spigot".to_string()),
            description: "foo-spigot-description".to_string(),
            slug: "foo.1".to_string(),
            date_created: datetime!(2020-01-01 0:00 UTC),
            date_updated: datetime!(2020-02-03 0:00 UTC),
            latest_minecraft_version: Some("1.21".to_string()),
            downloads: 100,
            likes: 200,
            author_id: 1,
            version_id: 1,
            version_name: Some(version_name.to_string()),
            premium: false,
            abandoned: false,
            icon_url: Some("data/resource_icons/1/1.jpg".to_string()),
            icon_data: Some(SPIGOT_BASE64_TEST_ICON_DATA.to_string()),
            source_url: Some("https://github.com/alice/foo".to_string()),
            source_repository: Some(SourceRepository {
                host: "github.com".to_string(),
                owner: "alice".to_string(),
                name: "foo".to_string(),
                id: None
            })
        };

        assert_that(&resource).is_equal_to(expected_resource);

        Ok(())
    }

    #[tokio::test]
    #[rstest]
    #[case::abandoned("ABANDONED foo-spigot")]
    #[case::archived("ARCHIVED foo-spigot")]
    #[case::deprecated("DEPRECATED foo-spigot")]
    #[case::discontinued("DISCONTINUED foo-spigot")]
    #[case::outdated("OUTDATED foo-spigot")]
    async fn should_convert_incoming_abandoned_resource(#[case] resource_name: &str) -> Result<()> {
        // Arrange
        let mut incoming_resource = create_test_resources()[0].clone();
        incoming_resource.name = resource_name.to_string();
        let version_name = "v1.2.3";

        // Act
        let resource = convert_incoming_resource(incoming_resource, &Some(version_name.to_string())).await?;

        // Assert
        let expected_resource = SpigotResource {
            id: 1,
            name: resource_name.to_string(),
            parsed_name: Some("foo-spigot".to_string()),
            description: "foo-spigot-description".to_string(),
            slug: "foo.1".to_string(),
            date_created: datetime!(2020-01-01 0:00 UTC),
            date_updated: datetime!(2020-02-03 0:00 UTC),
            latest_minecraft_version: Some("1.21".to_string()),
            downloads: 100,
            likes: 200,
            author_id: 1,
            version_id: 1,
            version_name: Some(version_name.to_string()),
            premium: false,
            abandoned: true,
            icon_url: Some("data/resource_icons/1/1.jpg".to_string()),
            icon_data: Some(SPIGOT_BASE64_TEST_ICON_DATA.to_string()),
            source_url: Some("https://github.com/alice/foo".to_string()),
            source_repository: Some(SourceRepository {
                host: "github.com".to_string(),
                owner: "alice".to_string(),
                name: "foo".to_string(),
                id: None
            })
        };

        assert_that(&resource).is_equal_to(expected_resource);

        Ok(())
    }

    #[tokio::test]
    async fn should_not_convert_resource_with_invalid_slug() -> Result<()> {
        // Arrange
        let file_url = "resources/1/download?version=1".to_string();

        let mut incoming_resource: IncomingSpigotResource = create_test_resources()[0].clone();
        incoming_resource.file = Some(IncomingSpigotResourceNestedFile {
            url: file_url.clone()
        });

        let version_name = "v1.2.3";

        // Act
        let result = convert_incoming_resource(incoming_resource, &Some(version_name.to_string())).await;

        // Assert
        assert_that(&result).is_err();

        let error = result.unwrap_err();
        let downcast_error = error.downcast_ref::<ConvertIncomingSpigotResourceError>().unwrap();

        if let ConvertIncomingSpigotResourceError::InvalidSlugFromURL{ resource_id, url } = downcast_error {
            assert_that(resource_id).is_equal_to(resource_id);
            assert_that(url).is_equal_to(file_url);
        } else {
            panic!("expected error to be InvalidSlugFromURL, but was {}", downcast_error);
        }

        Ok(())
    }

    #[tokio::test]
    async fn should_not_convert_resource_with_no_file() -> Result<()> {
        // Arrange
        let mut incoming_resource = create_test_resources()[0].clone();
        incoming_resource.file = None;

        let version_name = "v1.2.3";

        // Act
        let result = convert_incoming_resource(incoming_resource, &Some(version_name.to_string())).await;

        // Assert
        assert_that(&result).is_err();

        let error = result.unwrap_err();
        let downcast_error = error.downcast_ref::<ConvertIncomingSpigotResourceError>().unwrap();

        if let ConvertIncomingSpigotResourceError::FileNotFound{ resource_id } = downcast_error {
            assert_that(resource_id).is_equal_to(resource_id);
        } else {
            panic!("expected error to be FileNotFound, but was {}", downcast_error);
        }

        Ok(())
    }

    fn create_test_resources() -> Vec<IncomingSpigotResource> {
        vec![
            IncomingSpigotResource {
                id: 1,
                name: "foo-spigot".to_string(),
                tag: "foo-spigot-description".to_string(),
                icon: Some(IncomingSpigotResourceNestedIcon {
                    url: "data/resource_icons/1/1.jpg".to_string(),
                    data: SPIGOT_BASE64_TEST_ICON_DATA.to_string(),
                }),
                release_date: 1577836800,
                update_date: 1580688000,
                tested_versions: Some(vec!["1.20".to_string(), "1.20.6".to_string(), "1.21".to_string()]),
                downloads: 100,
                likes: Some(200),
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
                source_code_link: Some("https://github.com/alice/foo".to_string())
            },
            IncomingSpigotResource {
                id: 2,
                name: "bar-spigot".to_string(),
                tag: "bar-spigot-description".to_string(),
                icon: Some(IncomingSpigotResourceNestedIcon {
                    url: "data/resource_icons/2/2.jpg".to_string(),
                    data: SPIGOT_BASE64_TEST_ICON_DATA.to_string()
                }),
                release_date: 1577923200,
                update_date: 1580601600,
                tested_versions: Some(vec!["1.6".to_string(), "1.7".to_string(), "1.8".to_string()]),
                downloads: 300,
                likes: Some(100),
                file: Some(IncomingSpigotResourceNestedFile {
                    url: "resources/bar.2/download?version=2".to_string()
                }),
                author: IncomingSpigotResourceNestedAuthor {
                    id: 2
                },
                version: IncomingSpigotResourceNestedVersion {
                    id: 1
                },
                premium: Some(false),
                source_code_link: Some("https://gitlab.com/bob/bar".to_string())
            }
        ]
    }
}