use crate::HttpServer;
use crate::spigot::SpigotClient;
use crate::util::extract_source_repository_from_url;
use mc_plugin_finder::database::spigot::resource::{SpigotResource, upsert_spigot_resource};

use anyhow::Result;
use deadpool_postgres::Pool;
use futures::future;
use futures::stream::TryStreamExt;
use page_turner::prelude::*;
use regex::Regex;
use reqwest::StatusCode;
use serde::{Serialize, Deserialize};
use time::OffsetDateTime;
use std::cell::Cell;
use std::fmt::Debug;
use std::sync::OnceLock;
use thiserror::Error;
use tracing::{info, warn, instrument};
use unicode_segmentation::UnicodeSegmentation;

const SPIGOT_RESOURCES_REQUEST_FIELDS: &str = "id,name,tag,releaseDate,updateDate,downloads,file,author,version,premium,sourceCodeLink";
const SPIGOT_POPULATE_RESOURCES_REQUESTS_AHEAD: usize = 2;

// TODO: Replace OnceLock with LazyCell when it stabilizes in std: https://github.com/rust-lang/rust/issues/109736
static BRACKETS_REGEX: OnceLock<Regex> = OnceLock::new();
static RESOURCE_NAME_REGEX: OnceLock<Regex> = OnceLock::new();
static DISCOUNT_REGEX: OnceLock<Regex> = OnceLock::new();
static SLUG_REGEX: OnceLock<Regex> = OnceLock::new();

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
    release_date: i64,
    update_date: i64,
    downloads: i32,
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
        let request = GetSpigotResourcesRequest::create_populate_request();

        let count_cell: Cell<u32> = Cell::new(0);

        let result = self
            .pages_ahead(SPIGOT_POPULATE_RESOURCES_REQUESTS_AHEAD, Limit::None, request)
            .items()
            .try_for_each_concurrent(None, |incoming_resource| self.process_incoming_resource(incoming_resource, db_pool, &count_cell, false))
            .await;

        let count = count_cell.get();
        info!("Spigot resources populated: {}", count);

        result
    }

    #[instrument(
        skip(self, db_pool)
    )]
    pub async fn update_spigot_resources(&self, db_pool: &Pool, update_date_later_than: OffsetDateTime) -> Result<()> {
        let request = GetSpigotResourcesRequest::create_update_request();

        let count_cell: Cell<u32> = Cell::new(0);

        let result = self
            .pages(request)
            .items()
            .try_take_while(|x| future::ready(Ok(x.update_date > update_date_later_than.unix_timestamp())))
            .try_for_each(|incoming_resource| self.process_incoming_resource(incoming_resource, db_pool, &count_cell, true))
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

    async fn process_incoming_resource(&self, incoming_resource: IncomingSpigotResource, db_pool: &Pool, count_cell: &Cell<u32>, get_version: bool) -> Result<()> {
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
                    Ok(_) => count_cell.set(count_cell.get() + 1),
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

    if let Some(file) = incoming_resource.file {
        if let Some(slug) = extract_slug_from_file_download_url(&file.url) {
            let parsed_name = parse_resource_name(&incoming_resource.name);

            let mut resource = SpigotResource {
                id: incoming_resource.id,
                name: incoming_resource.name,
                parsed_name,
                description: incoming_resource.tag,
                slug,
                date_created: OffsetDateTime::from_unix_timestamp(incoming_resource.release_date)?,
                date_updated: OffsetDateTime::from_unix_timestamp(incoming_resource.update_date)?,
                downloads: incoming_resource.downloads,
                author_id: incoming_resource.author.id,
                version_id: incoming_resource.version.id,
                version_name: version_name.clone(),
                premium: incoming_resource.premium,
                source_url: incoming_resource.source_code_link.clone(),
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
                ConvertIncomingSpigotResourceError::InvalidSlugFromURL {
                    resource_id,
                    url: file.url
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

/*
    Attempts to find the actual Spigot resource name amidst the mess of emojis, special characters, and irrelevant text that are so common in the name field.

    This function performs the following preparatory steps before running a regex to get the name:
    1. Replace emoji with `|` separator characters.
    2. Replace `[]` or `()` brackets and their contents with `|` separator characters.
      - Unfortunately, there are a few resources that put their resource name in brackets. We won't attempt to try parsing the name from these resources.
    3. Remove discount text such as "SALE" and "OFF" so that it does not get matched in the regex.

    The regex will then find the first alphabetical word(s) (that may be in between `|` separators), and assume that is the actual name.
 */
fn parse_resource_name(name: &str) -> Option<String> {
    let mut text = replace_emoji_with_separators(name);
    text = replace_brackets_and_bracket_contents_with_separators(&text);
    text = remove_discount_text(&text);

    extract_resource_name(&text)
}

fn replace_emoji_with_separators(input: &str) -> String {
    let graphemes = input.graphemes(true);

    graphemes.map(|x: &str| {
        match emojis::get(x) {
            Some(_) => "|",
            None => x
        }
    }).collect()
}

fn replace_brackets_and_bracket_contents_with_separators(input: &str) -> String {
    let re = BRACKETS_REGEX.get_or_init(|| Regex::new(r"[\[\(].*?[\)\]]").unwrap());
    re.replace_all(input, "|").into_owned()
}

fn remove_discount_text(input: &str) -> String {
    let re = DISCOUNT_REGEX.get_or_init(|| Regex::new( r"SALE|OFF").unwrap());
    re.replace_all(input, "").into_owned()
}

/*
    Breakdown of this regex:

    `\p{letter}\p{mark}`
        - Matches international characters such as `é` or `ü`. It is preferred over [A-Za-z].

    `[\p{letter}\p{mark}]+[\p{letter}\p{mark}&-_'’]*[\p{letter}\p{mark}]+`
        - Includes first words that begin and end with letters/marks, and may contain dashes/underscores.
        - This allows us to include resources that have dashes/underscores within their name, but not dashes/underscores that are intended to be used as separators from other text.
        - Examples that are included:
            - "Anti-Xray-Webhook"
            - "T-ExplosiveSheep"
            - "QuickShop-Hikari"
            - "Admin_Panel"
            - "IP_Checker"
        - Examples that are excluded:
            - "ZMusic - 1.20 Ready - Powerful Music System"
            - "Quickshop-Hikari - A powerful, user-friendly and relieable ChestShop plugin"
            - "BackupSystem by ShadowX__"

    `...[\p{letter}\p{mark}&'’\s]*[\p{letter}\p{mark}]+\+*`
        - Includes resource names with multiple words.
        - Examples:
            - "HeadDatabase"
            - "AFK Rewards Premium"
        - Also includes names with trailing `+` characters.
        - Examples:
            - "Disguise+"
            - "Economy++"
 */
fn extract_resource_name(input: &str) -> Option<String> {
    let re = RESOURCE_NAME_REGEX.get_or_init(|| Regex::new(r"[\p{letter}\p{mark}]+[\p{letter}\p{mark}&-_'’]*[\p{letter}\p{mark}]+[\p{letter}\p{mark}&'’\s]*[\p{letter}\p{mark}]+\+*").unwrap());
    let mat = re.find(input)?;
    Some(mat.as_str().to_string())
}

fn extract_slug_from_file_download_url(url: &str) -> Option<String> {
    let re = SLUG_REGEX.get_or_init(|| Regex::new(r"resources/(\S+\.\d+)/download.*").unwrap());
    let caps = re.captures(url)?;
    Some(caps[1].to_string())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::spigot::test::SpigotTestServer;

    use rstest::*;
    use speculoos::prelude::*;
    use time::macros::datetime;
    use wiremock::{Mock, ResponseTemplate};
    use wiremock::matchers::{method, path, query_param};

    #[rstest]
    #[case::word("Foo", "Foo")]

    #[case::number_word("2Foo", "Foo")]
    #[case::word_number("Foo2", "Foo")]

    #[case::word_plus("Foo+", "Foo+")]
    #[case::word_plus_plus("Foo++", "Foo++")]

    #[case::word_space_word("Foo Bar", "Foo Bar")]
    #[case::word_space_word_space_word("Foo Bar Baz", "Foo Bar Baz")]

    #[case::hyphen_word("-Foo", "Foo")]
    #[case::word_hyphen("Foo-", "Foo")]
    #[case::word_hyphen_word("Foo-Bar", "Foo-Bar")]
    #[case::word_hyphen_word_space_word("Foo-Bar Baz", "Foo-Bar Baz")]
    #[case::word_space_word_hyphen_word("Foo Bar-Baz", "Foo Bar")]
    #[case::word_hyphen_space_word("Foo- Bar", "Foo")]
    #[case::word_space_hyphen_word("Foo -Bar", "Foo")]
    #[case::word_space_hyphen_space_word("Foo - Bar", "Foo")]

    #[case::underscore_word("_Foo", "Foo")]
    #[case::word_underscore("Foo_", "Foo")]
    #[case::word_underscore_word("Foo_Bar", "Foo_Bar")]
    #[case::word_underscore_word_space_word("Foo_Bar Baz", "Foo_Bar Baz")]
    #[case::word_space_word_underscore_word("Foo Bar_Baz", "Foo Bar")]
    #[case::word_underscore_space_word("Foo_ Bar", "Foo")]
    #[case::word_space_underscore_word("Foo _Bar", "Foo")]
    #[case::word_space_underscore_space_word("Foo _ Bar", "Foo")]

    #[case::emoji_word("✨Foo", "Foo")]
    #[case::emoji_space_word("✨ Foo", "Foo")]
    #[case::word_emoji("Foo✨", "Foo")]
    #[case::word_space_emoji("Foo ✨", "Foo")]

    #[case::square_brackets_word("[1.8.8 - 1.20.4]Foo", "Foo")]
    #[case::square_brackets_space_word("[1.8.8 - 1.20.4] Foo", "Foo")]
    #[case::word_square_brackets("Foo[1.8.8 - 1.20.4]", "Foo")]
    #[case::word_space_square_brackets("Foo [1.8.8 - 1.20.4]", "Foo")]

    #[case::round_brackets_word("(1.8.8 - 1.20.4)Foo", "Foo")]
    #[case::round_brackets_space_word("(1.8.8 - 1.20.4) Foo", "Foo")]
    #[case::word_round_brackets("Foo(1.8.8 - 1.20.4)", "Foo")]
    #[case::word_space_round_brackets("Foo (1.8.8 - 1.20.4)", "Foo")]

    #[case::discount_sale_word("25% SALE Foo", "Foo")]
    #[case::discount_off_word("25% OFF Foo", "Foo")]
    #[case::word_discount_sale("Foo 25% SALE", "Foo")]
    #[case::word_discount_off("Foo 25% OFF", "Foo")]

    #[case::words_with_apostrophe("Frumple's Foobar", "Frumple's Foobar")]
    #[case::words_with_right_single_quotation_mark("Frumple’s Foobar", "Frumple’s Foobar")]
    #[case::words_with_ampersand("Foo & Bar", "Foo & Bar")]

    #[case::word_with_accent("Café", "Café")]
    #[case::word_with_umlaut("Über", "Über")]

    #[case::everything("SALE 30% ⚡ [1.15.1-1.20.4+] ⛏️ Foo-Bar Baz++ - Best Moderation Plugin | ✅ Database Support!", "Foo-Bar Baz++")]
    fn should_parse_resource_name(#[case] input: &str, #[case] expected_name: &str) {
        let parsed_name = parse_resource_name(input);
        assert_that(&parsed_name).is_some().is_equal_to(expected_name.to_string());
    }

    #[rstest]
    #[case::one_letter_word("F")]
    #[case::two_letter_word("Fo")]
    fn should_not_parse_resource_name(#[case] input: &str) {
        let parsed_name = parse_resource_name(input);
        assert_that(&parsed_name).is_none();
    }

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
        assert_that(&resource.id).is_equal_to(1);
        assert_that(&resource.name).is_equal_to("resource-1".to_string());
        assert_that(&resource.description).is_equal_to("resource-1-tag".to_string());
        assert_that(&resource.slug).is_equal_to("foo.1".to_string());
        assert_that(&resource.date_created).is_equal_to(datetime!(2020-01-01 0:00 UTC));
        assert_that(&resource.date_updated).is_equal_to(datetime!(2021-01-01 0:00 UTC));
        assert_that(&resource.downloads).is_equal_to(100);
        assert_that(&resource.author_id).is_equal_to(1);
        assert_that(&resource.version_id).is_equal_to(1);
        assert_that(&resource.version_name).is_some().is_equal_to(version_name.to_string());
        assert_that(&resource.premium).is_some().is_false();
        assert_that(&resource.source_url).is_some().is_equal_to("https://github.com/Frumple/foo".to_string());
        assert_that(&resource.source_repository_host).is_some().is_equal_to("github.com".to_string());
        assert_that(&resource.source_repository_owner).is_some().is_equal_to("Frumple".to_string());
        assert_that(&resource.source_repository_name).is_some().is_equal_to("foo".to_string());

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
                name: "resource-1".to_string(),
                tag: "resource-1-tag".to_string(),
                release_date: 1577836800,
                update_date: 1609459200,
                downloads: 100,
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
                downloads: 100,
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