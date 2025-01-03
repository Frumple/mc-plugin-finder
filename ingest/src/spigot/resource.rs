use crate::HttpServer;
use crate::spigot::SpigotClient;
use mc_plugin_finder::database::spigot::resource::{SpigotResource, upsert_spigot_resource};
use mc_plugin_finder::database::source_repository::{SourceRepository, extract_source_repository_from_url};

use anyhow::Result;
use deadpool_postgres::Pool;
use futures::future;
use futures::stream::TryStreamExt;
use page_turner::prelude::*;
use regex::{Regex, RegexBuilder};
use reqwest::StatusCode;
use serde::{Serialize, Deserialize};
use time::OffsetDateTime;
use std::cell::Cell;
use std::fmt::Debug;
use std::sync::LazyLock;
use thiserror::Error;
use tracing::{info, warn, instrument};
use unicode_segmentation::UnicodeSegmentation;

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
        let request = GetSpigotResourcesRequest::create_populate_request();

        let count_cell: Cell<u32> = Cell::new(0);

        let result = self
            .pages_ahead(SPIGOT_RESOURCES_REQUESTS_AHEAD, Limit::None, request)
            .items()
            .try_for_each_concurrent(SPIGOT_RESOURCES_CONCURRENT_FUTURES, |incoming_resource| self.process_incoming_resource(incoming_resource, db_pool, &count_cell, false))
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
            .pages_ahead(SPIGOT_RESOURCES_REQUESTS_AHEAD, Limit::None, request)
            .items()
            .try_take_while(|x| future::ready(Ok(x.update_date > update_date_later_than.unix_timestamp())))
            .try_for_each_concurrent(SPIGOT_RESOURCES_CONCURRENT_FUTURES, |incoming_resource| self.process_incoming_resource(incoming_resource, db_pool, &count_cell, true))
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

    if let Some(ref file) = incoming_resource.file {
        if let Some(slug) = extract_slug_from_file_download_url(&file.url) {
            let parsed_name = parse_resource_name(&incoming_resource.name);
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
                        name: repo.name
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

/*
    Attempts to find the actual Spigot resource name amidst the hideous mess of emojis, special characters, and irrelevant text that are so common in the name field.

    This function performs several pre-processing steps, followed by finding the resource name itself, and then some post-processing steps.

    Pre-processing steps:
    1. Replace emoji with `|` separator characters.
    2. Replace `[]` or `()` brackets and their contents with `|` separator characters.
      - Unfortunately, there are a few resources that put their resource name in brackets.
        We ignore any text within brackets, so these resource names will not be parsed.
        Examples:
        - "[RealisticMC] • 1.8 - 1.12 • No Lag • Epic Effects • Configurable • Multi-World • (Inactive)"
        - "[ Better Invisibility ] - 1.8 ~ 1.19 (isnt a Vanish Plugin)""
        - "[PowerBoard] Scoreboard + Tablist + Prefix + Chat | Animated"
    3. Replace `-` dashes or  `_` underscores that are adjacent to whitespace with `|` separator characters.
      Examples:
      - "Foo - Bar" => "Foo | Bar"
      - "Foo- Bar"  => "Foo| Bar"
      - "Foo _Bar"  => "Foo |Bar"
      - "Foo-Bar" or "Foo_Bar" will remain unchanged.
    4. Remove abandonment text such as "abandoned", "discontinued", "deprecated", and "outdated" (lowercase or uppercase) so that it does not get included in the resource name.
    5. Remove discount text such as "SALE" and "OFF" (uppercase only) so that it does get included in the resource name.

    Name extraction step:
    A regex will then find the first alphabetical word(s) (that may be in between `|`, `-`, `_`, or other separators), and assume that is the actual name.
      - Allows names with any number of internal `-` dashes and `_` underscores, provided that they are not adjacent to whitespace from pre-processing step #3 above.
        Examples:
        - "Quickshop-Hikari"
        - "Phoenix Anti-Cheat"
        - "Ultimate_Economy"
        - "MegaFFA By ImRoyal_Raddar"
      - Allows names with any number of internal `&` ampersands, `'` and `’` apostrophes.
        Examples:
        - "Minions & Hunger"
        - "Lib's Disguises"
        - "RS’s AntiCheat"
      - Allows names with any number of trailing `+` characters.
        Examples:
        - "Disguise+"
        - "Economy++"

    Post-processing steps:
    1. If the name ends with whitespace followed by a single "v" or "V" character, remove both the whitespace and character.
      Examples (original => name extraction => post-processing):
      - "PlotSquared v4"   => "PlotSquared v" => "PlotSquared"
      - "FactionMenu V1.2" => "FactionMenu V" => "FactionMenu"
 */
fn parse_resource_name(name: &str) -> Option<String> {
    let mut preprocessed_text = replace_emoji_with_separators(name);
    preprocessed_text = replace_brackets_and_bracket_contents_with_separators(&preprocessed_text);
    preprocessed_text = replace_dashes_and_underscores_adjacent_to_whitespace_with_separators(&preprocessed_text);
    preprocessed_text = remove_abandonment_text(&preprocessed_text);
    preprocessed_text = remove_discount_text(&preprocessed_text);

    let parsed_name = extract_resource_name(&preprocessed_text);

    remove_trailing_whitespace_and_single_v_character(&parsed_name)
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

static BRACKETS_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[\[\(].*?[\)\]]").unwrap());

fn replace_brackets_and_bracket_contents_with_separators(input: &str) -> String {
    let re = &*BRACKETS_REGEX;
    re.replace_all(input, "|").into_owned()
}

static DASHES_AND_UNDERSCORES_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(\s+[-_])|([-_]\s+)").unwrap());

fn replace_dashes_and_underscores_adjacent_to_whitespace_with_separators(input: &str) -> String {
    let re = &*DASHES_AND_UNDERSCORES_REGEX;
    re.replace_all(input, "|").into_owned()
}

static ABANDONMENT_REGEX: LazyLock<Regex> = LazyLock::new(||
    RegexBuilder::new(r"abandoned|archived|deprecated|discontinued|outdated")
    .case_insensitive(true)
    .build()
    .unwrap());

fn remove_abandonment_text(input: &str) -> String {
    let re = &*ABANDONMENT_REGEX;
    re.replace_all(input, "").into_owned()
}

static DISCOUNT_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new( r"SALE|OFF").unwrap());

fn remove_discount_text(input: &str) -> String {
    let re = &*DISCOUNT_REGEX;
    re.replace_all(input, "").into_owned()
}

static RESOURCE_NAME_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[\p{L}]+[\p{L}\s\d&'’_-]*[\p{L}]+\+*").unwrap());

fn extract_resource_name(input: &str) -> Option<String> {
    let re = &*RESOURCE_NAME_REGEX;
    let mat = re.find(input)?;
    Some(mat.as_str().to_string())
}

static TRAILING_WHITESPACE_V_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\s+[vV]$").unwrap());

fn remove_trailing_whitespace_and_single_v_character(input: &Option<String>) -> Option<String> {
    if let Some(name) = input {
        let re = &*TRAILING_WHITESPACE_V_REGEX;
        return Some(re.replace_all(name, "").into_owned());
    }

    None
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

    // Cases where the name is preserved:

    // Single words are preserved
    #[case::word("Foo", "Foo")]
    #[case::two_letter_word("Fo", "Fo")]

    // Trailing `+` characters are preserved
    #[case::word_plus("Foo+", "Foo+")]
    #[case::word_plus_plus("Foo++", "Foo++")]

    // Multiple words are preserved
    #[case::word_space_word("Foo Bar", "Foo Bar")]
    #[case::word_space_word_space_word("Foo Bar Baz", "Foo Bar Baz")]

    // Internal hyphens are preserved
    #[case::word_hyphen_word("Foo-Bar", "Foo-Bar")]
    #[case::word_hyphen_word_space_word("Foo-Bar Baz", "Foo-Bar Baz")]
    #[case::word_space_word_hyphen_word("Foo Bar-Baz", "Foo Bar-Baz")]

    // Internal underscores are preserved
    #[case::word_underscore_word("Foo_Bar", "Foo_Bar")]
    #[case::word_underscore_word_space_word("Foo_Bar Baz", "Foo_Bar Baz")]
    #[case::word_space_word_underscore_word("Foo Bar_Baz", "Foo Bar_Baz")]

    // Internal digits are preserved
    #[case::word_number_word("Foo2Bar", "Foo2Bar")]
    #[case::word_space_number_word("Foo 2Bar", "Foo 2Bar")]
    #[case::word_number_space_word("Foo2 Bar", "Foo2 Bar")]
    #[case::word_space_number_space_word("Foo 2 Bar", "Foo 2 Bar")]

    // Internal apostrophes are preserved
    #[case::words_with_apostrophe("Frumple's Foobar", "Frumple's Foobar")]
    #[case::words_with_right_single_quotation_mark("Frumple’s Foobar", "Frumple’s Foobar")]

    // Internal ampersands are preserved
    #[case::word_ampersand_word("Foo&Bar", "Foo&Bar")]
    #[case::word_space_ampersand_space_word("Foo & Bar", "Foo & Bar")]

    // International characters are preserved
    #[case::word_with_accent("Café", "Café")]
    #[case::word_with_umlaut("Über", "Über")]

    // Names ending with `v` or `V` with no whitespace are preserved
    #[case::ends_with_lowercase_v_no_whitespace("BetterNav", "BetterNav")]
    #[case::ends_with_uppercase_v_no_whitespace("DiscordSRV", "DiscordSRV")]

    // Cases where undesired elements are removed:

    // Emojis are removed
    #[case::emoji_word("✨Foo", "Foo")]
    #[case::emoji_space_word("✨ Foo", "Foo")]
    #[case::word_emoji("Foo✨", "Foo")]
    #[case::word_space_emoji("Foo ✨", "Foo")]

    // Leading and trailing dashes are removed
    #[case::hyphen_word("-Foo", "Foo")]
    #[case::word_hyphen("Foo-", "Foo")]

    // Leading and trailing underscores are removed
    #[case::underscore_word("_Foo", "Foo")]
    #[case::word_underscore("Foo_", "Foo")]

    // Internal dashes adjacent to whitespace are removed
    #[case::word_hyphen_space_word("Foo- Bar", "Foo")]
    #[case::word_space_hyphen_word("Foo -Bar", "Foo")]
    #[case::word_space_hyphen_space_word("Foo - Bar", "Foo")]

    // Internal underscores adjacent to whitespace are removed
    #[case::word_underscore_space_word("Foo_ Bar", "Foo")]
    #[case::word_space_underscore_word("Foo _Bar", "Foo")]
    #[case::word_space_underscore_space_word("Foo _ Bar", "Foo")]

    // Square brackets and their contents are removed
    #[case::square_brackets_word("[1.8.8 - 1.20.4]Foo", "Foo")]
    #[case::square_brackets_space_word("[1.8.8 - 1.20.4] Foo", "Foo")]
    #[case::word_square_brackets("Foo[1.8.8 - 1.20.4]", "Foo")]
    #[case::word_space_square_brackets("Foo [1.8.8 - 1.20.4]", "Foo")]

    // Round brackets and their contents are removed
    #[case::round_brackets_word("(1.8.8 - 1.20.4)Foo", "Foo")]
    #[case::round_brackets_space_word("(1.8.8 - 1.20.4) Foo", "Foo")]
    #[case::word_round_brackets("Foo(1.8.8 - 1.20.4)", "Foo")]
    #[case::word_space_round_brackets("Foo (1.8.8 - 1.20.4)", "Foo")]

    // Abandonment words are removed
    #[case::lowercase_abandoned_word("abandoned Foo", "Foo")]
    #[case::uppercase_abandoned_word("ABANDONED Foo", "Foo")]
    #[case::word_lowercase_abandoned("Foo abandoned", "Foo")]
    #[case::word_uppercase_abandoned("Foo ABANDONED", "Foo")]

    #[case::lowercase_archived_word("archived Foo", "Foo")]
    #[case::uppercase_archived_word("ARCHIVED Foo", "Foo")]
    #[case::word_lowercase_archived("Foo archived", "Foo")]
    #[case::word_uppercase_archived("Foo ARCHIVED", "Foo")]

    #[case::lowercase_deprecated_word("deprecated Foo", "Foo")]
    #[case::uppercase_deprecated_word("DEPRECATED Foo", "Foo")]
    #[case::word_lowercase_deprecated("Foo deprecated", "Foo")]
    #[case::word_uppercase_deprecated("Foo DEPRECATED", "Foo")]

    #[case::lowercase_discontinued_word("discontinued Foo", "Foo")]
    #[case::uppercase_discontinued_word("DISCONTINUED Foo", "Foo")]
    #[case::word_lowercase_discontinued("Foo discontinued", "Foo")]
    #[case::word_uppercase_discontinued("Foo DISCONTINUED", "Foo")]

    #[case::lowercase_outdated_word("outdated Foo", "Foo")]
    #[case::uppercase_outdated_word("OUTDATED Foo", "Foo")]
    #[case::word_lowercase_outdated("Foo outdated", "Foo")]
    #[case::word_uppercase_outdated("Foo OUTDATED", "Foo")]

    // Discount words are removed
    #[case::discount_sale_word("25% SALE Foo", "Foo")]
    #[case::discount_off_word("25% OFF Foo", "Foo")]
    #[case::word_discount_sale("Foo 25% SALE", "Foo")]
    #[case::word_discount_off("Foo 25% OFF", "Foo")]

    // Leading digits are removed
    #[case::number_word("2Foo", "Foo")]
    #[case::word_number("Foo2", "Foo")]

    // Leading and trailing apostrophes are removed
    #[case::apostrophe_word("'Foo", "Foo")]
    #[case::word_apostrophe("Foo'", "Foo")]

    // Leading and trailing right single quotation marks are removed
    #[case::right_single_quotation_mark_word("’Foo", "Foo")]
    #[case::word_right_single_quotation_mark("Foo’", "Foo")]

    // Leading and trailing ampersands are removed
    #[case::ampersand_word("&Foo", "Foo")]
    #[case::word_ampersand("Foo&", "Foo")]

    // Leading and trailing version numbers are removed
    #[case::no_v_version_word("1.2.3 Foo", "Foo")]
    #[case::lowercase_v_version_word("v1.2.3 Foo", "Foo")]
    #[case::uppercase_v_version_word("V1.2.3 Foo", "Foo")]
    #[case::word_no_v_version("Foo 1.2.3", "Foo")]
    #[case::word_lowercase_v_version("Foo v1.2.3", "Foo")]
    #[case::word_uppercase_v_version("Foo V1.2.3", "Foo")]

    // Internal version numbers are removed (in addition to later words)
    #[case::word_no_v_version_word("Foo 1.2.3 Bar", "Foo")]
    #[case::word_lowercase_v_version_word("Foo v1.2.3 Bar", "Foo")]
    #[case::word_uppercase_v_version_word("Foo V1.2.3 Bar", "Foo")]

    #[case::everything("SALE 30% ⚡ [1.15.1-1.20.4+] ⛏️ Foo's Bar Baz++ v2.0 - Best Moderation Plugin | ✅ Database Support!", "Foo's Bar Baz++")]
    fn should_parse_resource_name(#[case] input: &str, #[case] expected_name: &str) {
        let parsed_name = parse_resource_name(input);
        assert_that(&parsed_name).is_some().is_equal_to(expected_name.to_string());
    }

    #[rstest]
    #[case::one_letter_word("F")]
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
                name: "foo".to_string()
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
                name: "foo".to_string()
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