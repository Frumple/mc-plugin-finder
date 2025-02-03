use crate::error_template::{AppError, ErrorTemplate};
use crate::util::format_number;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use serde::{Serialize, Deserialize};
use time::format_description::BorrowedFormatItem;
use std::str::FromStr;
use time::OffsetDateTime;
use time::macros::format_description;

#[cfg(feature = "ssr")]
use mc_plugin_finder::database::common::search_result::{SearchParams, SearchParamsSort, SearchResult, SearchResultSpigot, SearchResultModrinth, SearchResultHangar};

// For Modrinth and Hangar project icons, attempt to retrieve a cached version from the image proxy first.
// This reduces unnecessary load on the Modrinth and Hangar CDNs.

// Set this to false to retrieve icons directly from the Modrinth and Hangar CDNs.
const USE_IMAGEPROXY: bool = true;
const IMAGEPROXY_URL_PREFIX: &str = "https://img.mcpluginfinder.com/75,fit";

const NO_ICON_IMAGE_URL: &str = "images/no-icon.svg";
const ABANDONED_IMAGE_URL: &str = "images/abandoned.svg";
const PREMIUM_IMAGE_URL: &str = "images/premium.svg";

const VERSION: &str = env!("CARGO_PKG_VERSION");

const SEARCH_RESULT_DATE_FORMAT_DESCRIPTION: &[BorrowedFormatItem] = format_description!("[year]-[month]-[day]");
const SEARCH_RESULT_TIME_FORMAT_DESCRIPTION: &[BorrowedFormatItem] = format_description!("[hour]:[minute]:[second]");
const LAST_INGEST_DATE_FORMAT_DESCRIPTION: &[BorrowedFormatItem] = format_description!("[year]-[month]-[day] [hour]:[minute]:[second] UTC");

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Params)]
pub struct WebSearchParams {
    pub query: Option<String>,
    pub spigot: Option<bool>,
    pub modrinth: Option<bool>,
    pub hangar: Option<bool>,
    pub name: Option<bool>,
    pub description: Option<bool>,
    pub author: Option<bool>,
    pub sort: Option<String>,
    pub limit: Option<u32>,
    pub page: Option<u32>
}

impl WebSearchParams {
    fn offset(&self) -> Option<u32> {
        if let Some(page) = self.page {
            if let Some(limit) = self.limit {
                return Some(page.saturating_sub(1).saturating_mul(limit))
            }
        }
        None
    }

    // TODO: Handle number conversions properly
    fn total_pages(&self, full_count: i64) -> Option<u32> {
        if let Some(limit) = self.limit {
            return Some((full_count as f64 / limit as f64).ceil() as u32)
        }
        None
    }

    fn form_url(&self) -> String {
        "?".to_string() +
        &serde_urlencoded::to_string(self).unwrap_or(
            "query=&spigot=true&modrinth=true&hangar=true&name=true&sort=relevance&limit=25&page=1".to_string())
    }

    fn first_url(&self) -> String {
        let mut params = self.clone();
        params.page = Some(1);
        params.form_url()
    }

    fn previous_url(&self) -> String {
        let mut params = self.clone();
        params.page = Some(params.page.unwrap_or_default().saturating_sub(1));
        params.form_url()
    }

    fn next_url(&self) -> String {
        let mut params = self.clone();
        params.page = Some(params.page.unwrap_or_default().saturating_add(1));
        params.form_url()
    }

    fn last_url(&self, full_count: i64) -> String {
        let mut params = self.clone();
        params.page = Some(params.total_pages(full_count).unwrap_or(1));
        params.form_url()
    }
}

impl Default for WebSearchParams {
    fn default() -> Self {
        WebSearchParams {
            query: Some("".to_string()),
            spigot: Some(false),
            modrinth: Some(false),
            hangar: Some(false),
            name: Some(false),
            description: Some(false),
            author: Some(false),
            sort: Some("relevance".to_string()),
            limit: Some(25),
            page: Some(1)
        }
    }
}

#[cfg(feature = "ssr")]
impl From<WebSearchParams> for SearchParams {
    fn from(params: WebSearchParams) -> Self {
        let offset = params.offset();

        SearchParams {
            query: params.query.unwrap_or_default(),
            spigot: params.spigot.unwrap_or_default(),
            modrinth: params.modrinth.unwrap_or_default(),
            hangar: params.hangar.unwrap_or_default(),
            name: params.name.unwrap_or_default(),
            description: params.description.unwrap_or_default(),
            author: params.author.unwrap_or_default(),
            sort: SearchParamsSort::from_str(&params.sort.unwrap_or_default()).unwrap_or_default(),
            limit: params.limit.unwrap_or(25).into(),
            offset: offset.unwrap_or_default().into()
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebSearchResponse {
    pub results: Vec<WebSearchResult>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebSearchResult {
    pub full_count: i64,

    pub date_created: OffsetDateTime,
    pub date_updated: OffsetDateTime,
    pub latest_minecraft_version: Option<String>,
    pub downloads: i32,
    pub likes_and_stars: i32,
    pub follows_and_watchers: i32,

    pub spigot: Option<WebSearchResultSpigot>,
    pub modrinth: Option<WebSearchResultModrinth>,
    pub hangar: Option<WebSearchResultHangar>,
    pub source_repository: Option<WebSearchResultSourceRepository>
}

impl WebSearchResult {
    fn downloads_formatted(&self) -> String {
        format_number(&self.downloads)
    }

    fn likes_and_stars_formatted(&self) -> String {
        format_number(&self.likes_and_stars)
    }

    fn follows_and_watchers_formatted(&self) -> String {
        format_number(&self.follows_and_watchers)
    }

    fn project_name(&self) -> Option<String> {
        if let Some(spigot) = &self.spigot {
            return spigot.name.clone()
        }

        if let Some(modrinth) = &self.modrinth {
            return Some(modrinth.name.clone())
        }

        if let Some(hangar) = &self.hangar {
            return Some(hangar.name.clone())
        }
        None
    }
}

#[cfg(feature = "ssr")]
impl From<SearchResult> for WebSearchResult {
    fn from(search_result: SearchResult) -> Self {
        let spigot = search_result.spigot.map(|s| s.into());
        let modrinth = search_result.modrinth.map(|m| m.into());
        let hangar = search_result.hangar.map(|h| h.into());
        let source_repository = search_result.source_repository.map(|r| r.into());

        WebSearchResult {
            full_count: search_result.full_count,

            date_created: search_result.date_created,
            date_updated: search_result.date_updated,
            latest_minecraft_version: search_result.latest_minecraft_version,
            downloads: search_result.downloads,
            likes_and_stars: search_result.likes_and_stars,
            follows_and_watchers: search_result.follows_and_watchers,

            spigot,
            modrinth,
            hangar,
            source_repository
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebSearchResultSpigot {
    pub id: i32,
    pub slug: String,
    pub name: Option<String>,
    pub description: String,
    pub author: String,
    pub version: Option<String>,
    pub premium: bool,
    pub abandoned: bool,
    pub icon_data: Option<String>
}

impl WebSearchResultSpigot {
    fn url(&self) -> Option<String> {
        Some(format!("https://spigotmc.org/resources/{}", self.slug))
    }

    fn icon_img_url(&self) -> String {
        let data = self.icon_data.clone();

        if data.as_ref().map_or(true, String::is_empty) {
            return NO_ICON_IMAGE_URL.to_string();
        }

        format!("data:image/png;base64,{}", data.unwrap())
    }

    fn icon_alt_text(&self) -> Option<String> {
        alt_text(&self.name, "Spigot")
    }
}

#[cfg(feature = "ssr")]
impl From<SearchResultSpigot> for WebSearchResultSpigot {
    fn from(s: SearchResultSpigot) -> Self {
        WebSearchResultSpigot {
            id: s.id,
            slug: s.slug,
            name: s.name,
            description: s.description,
            author: s.author,
            version: s.version,
            premium: s.premium,
            abandoned: s.abandoned,
            icon_data: s.icon_data,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebSearchResultModrinth {
    pub id: String,
    pub slug: String,
    pub name: String,
    pub description: String,
    pub author: String,
    pub version: Option<String>,
    pub status: String,
    pub icon_url: Option<String>
}

impl WebSearchResultModrinth {
    fn url(&self) -> Option<String> {
        Some(format!("https://modrinth.com/plugin/{}", self.slug))
    }

    fn icon_img_url(&self) -> String {
        let url = self.icon_url.clone();

        if url.as_ref().map_or(true, String::is_empty) {
            return NO_ICON_IMAGE_URL.to_string();
        }

        if USE_IMAGEPROXY {
            return format!("{}/{}", IMAGEPROXY_URL_PREFIX, url.unwrap());
        }

        url.unwrap()
    }

    fn icon_alt_text(&self) -> Option<String> {
        alt_text(&Some(self.name.clone()), "Modrinth")
    }

    fn is_archived(&self) -> bool {
        self.status == "archived"
    }
}

#[cfg(feature = "ssr")]
impl From<SearchResultModrinth> for WebSearchResultModrinth {
    fn from(m: SearchResultModrinth) -> Self {
        WebSearchResultModrinth {
            id: m.id,
            slug: m.slug,
            name: m.name,
            description: m.description,
            author: m.author,
            version: m.version,
            status: m.status,
            icon_url: m.icon_url,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebSearchResultHangar {
    pub slug: String,
    pub name: String,
    pub description: String,
    pub author: String,
    pub version: Option<String>,
    pub icon_url: String
}

impl WebSearchResultHangar {
    fn url(&self) -> Option<String> {
        Some(format!("https://hangar.papermc.io/{}/{}", self.author, self.slug))
    }

    fn icon_img_url(&self) -> String {
        let url = self.icon_url.clone();

        if url.is_empty() {
            return NO_ICON_IMAGE_URL.to_string();
        }

        if USE_IMAGEPROXY {
            return format!("{}/{}", IMAGEPROXY_URL_PREFIX, url);
        }

        url
    }

    fn icon_alt_text(&self) -> Option<String> {
        alt_text(&Some(self.name.clone()), "Hangar")
    }
}

#[cfg(feature = "ssr")]
impl From<SearchResultHangar> for WebSearchResultHangar {
    fn from(h: SearchResultHangar) -> Self {
        WebSearchResultHangar {
            slug: h.slug,
            name: h.name,
            description: h.description,
            author: h.author,
            version: h.version,
            icon_url: h.icon_url,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebSearchResultSourceRepository {
    pub host: String,
    pub owner: String,
    pub name: String
}

impl WebSearchResultSourceRepository {
    fn url(&self) -> String {
        format!("https://{}/{}/{}", self.host, self.owner, self.name)
    }

    fn url_wbr(&self) -> String {
        format!("https://{}/<wbr>{}/<wbr>{}", self.host, self.owner, self.name)
    }

    fn img_attributes(&self, project_name: &Option<String>) -> ImgAttributes {
        match self.host.as_str() {
            "github.com" => ImgAttributes {
                src: Some("images/github-logo.svg".to_string()),
                title: Some("GitHub".to_string()),
                alt: alt_text(project_name, "GitHub")
            },
            "gitlab.com" => ImgAttributes {
                src: Some("images/gitlab-logo.svg".to_string()),
                title: Some("GitLab".to_string()),
                alt: alt_text(project_name, "GitLab")
            },
            "bitbucket.org" => ImgAttributes {
                src: Some("images/bitbucket-logo.svg".to_string()),
                title: Some("Bitbucket".to_string()),
                alt: alt_text(project_name, "Bitbucket")
            },
            "codeberg.org" => ImgAttributes {
                src: Some("images/codeberg-logo.svg".to_string()),
                title: Some("Codeberg".to_string()),
                alt: alt_text(project_name, "Codeberg")
            },
            _ => ImgAttributes {
                src: Some(NO_ICON_IMAGE_URL.to_string()),
                title: Some("Unknown Repository".to_string()),
                alt: Some("Unknown Repository".to_string())
            }
        }
    }
}

#[cfg(feature = "ssr")]
impl From<mc_plugin_finder::database::source_repository::SourceRepository> for WebSearchResultSourceRepository {
    fn from(repo: mc_plugin_finder::database::source_repository::SourceRepository) -> Self {
        WebSearchResultSourceRepository {
            host: repo.host,
            owner: repo.owner,
            name: repo.name
        }
    }
}

#[cfg(feature = "ssr")]
pub mod ssr {
    use deadpool_postgres::Pool;
    use leptos::use_context;

    #[derive(Clone)]
    pub struct WebContext {
        pub db_pool: Pool
    }

    pub async fn context() -> Option<WebContext> {
        use_context::<WebContext>()
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    // TODO: Move javascript to external file
    view! {
        <Script>
            "
            const submitForm = (form) => {
                form.requestSubmit()
            }

            const debounce = (callback, delay) => {
                let timeoutId = null;

                return (...args) => {
                    window.clearTimeout(timeoutId);

                    timeoutId = window.setTimeout(() => {
                        callback(...args);
                    }, delay);
                };
            }

            const SUBMIT_FORM_DEBOUNCE_DELAY = 250;
            const submitFormDebounce = debounce(submitForm, SUBMIT_FORM_DEBOUNCE_DELAY);
            "
        </Script>

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/web.css"/>

        // sets the document title
        <Title text="MC Plugin Finder"/>

        // Description shown in search engine results
        <Meta name="description" content="Find thousands of Minecraft server plugins with MC Plugin Finder, a search aggregator for plugins hosted on Spigot, Modrinth, and Hangar." />

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[server(SearchProjects)]
pub async fn search_projects(params: WebSearchParams) -> Result<WebSearchResponse, ServerFnError> {
    use self::ssr::*;
    use mc_plugin_finder::database::common::search_result::search_projects;

    if let Some(context) = context().await {
        let common_projects = search_projects(&context.db_pool, &params.into()).await;

        match common_projects {
            Ok(projects) => {
                let response = WebSearchResponse {
                    results: projects.into_iter().map(|x| x.into()).collect()
                };
                Ok(response)
            }
            Err(error) => Err(ServerFnError::ServerError(error.to_string()))
        }
    } else {
        Err(ServerFnError::ServerError("web context not found".to_string()))
    }
}

async fn fetch_projects(params_result: Result<WebSearchParams, ParamsError>) -> Result<WebSearchResponse, ServerFnError> {
    match params_result {
        Ok(params) => search_projects(params).await,
        Err(error) => Err(ServerFnError::ServerError(error.to_string()))
    }
}

#[server(GetLastIngestDate)]
pub async fn get_last_ingest_date() -> Result<String, ServerFnError> {
    use self::ssr::*;
    use mc_plugin_finder::database::ingest_log::get_last_ingest_log;

    if let Some(context) = context().await {
        let last_ingest_log = get_last_ingest_log(&context.db_pool).await;

        match last_ingest_log {
            Ok(log) => Ok(log.date_finished.format(&LAST_INGEST_DATE_FORMAT_DESCRIPTION)?),
            Err(error) => Err(ServerFnError::ServerError(error.to_string()))
        }
    } else {
        Err(ServerFnError::ServerError("web context not found".to_string()))
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let params_memo_original = use_query::<WebSearchParams>();

    let params_memo = create_memo(move |_| {
        params_memo_original.get().map( |params| {
            // When first loading the home page with no query paramemters,
            // perform a default search on all repositories using the name field only.
            if params.query.is_none() &&
               params.spigot.is_none() &&
               params.modrinth.is_none() &&
               params.hangar.is_none() &&
               params.name.is_none() &&
               params.description.is_none() &&
               params.author.is_none() &&
               params.sort.is_none() &&
               params.limit.is_none() &&
               params.page.is_none() {
                return WebSearchParams {
                    query: Some("".to_string()),
                    spigot: Some(true),
                    modrinth: Some(true),
                    hangar: Some(true),
                    name: Some(true),
                    description: None,
                    author: None,
                    sort: Some("relevance".to_string()),
                    limit: Some(25),
                    page: Some(1)
                }
            }
            params
        })
    });

    let search_resource = create_resource(
        move || params_memo.get(),
        fetch_projects
    );

    let last_ingest_date_resource = Resource::once(get_last_ingest_date);
    let last_ingest_date = move || last_ingest_date_resource.get();

    view! {
        <a href="https://github.com/Frumple/mc-plugin-finder" class="github-corner" aria-label="View source on GitHub" target="_blank">
            <svg class="github-corner__svg" width="80" height="80" viewBox="0 0 250 250" aria-hidden="true">
                <path d="M0,0 L115,115 L130,115 L142,142 L250,250 L250,0 Z"/>
                <path d="M128.3,109.0 C113.8,99.7 119.0,89.6 119.0,89.6 C122.0,82.7 120.5,78.6 120.5,78.6 C119.2,72.0 123.4,76.3 123.4,76.3 C127.3,80.9 125.5,87.3 125.5,87.3 C122.9,97.6 130.6,101.9 134.4,103.2" fill="currentColor" style="transform-origin: 130px 106px;" class="github-corner__octo-arm"/>
                <path d="M115.0,115.0 C114.9,115.1 118.7,116.5 119.8,115.4 L133.7,101.6 C136.9,99.2 139.9,98.4 142.2,98.6 C133.8,88.0 127.5,74.4 143.8,58.0 C148.5,53.4 154.0,51.2 159.7,51.0 C160.3,49.4 163.2,43.6 171.4,40.1 C171.4,40.1 176.1,42.5 178.8,56.2 C183.1,58.6 187.2,61.8 190.9,65.4 C194.5,69.0 197.7,73.2 200.1,77.6 C213.8,80.2 216.3,84.9 216.3,84.9 C212.7,93.1 206.9,96.0 205.4,96.6 C205.1,102.4 203.0,107.8 198.3,112.5 C181.9,128.9 168.3,122.5 157.7,114.1 C157.9,116.9 156.7,120.9 152.7,124.9 L141.0,136.5 C139.8,137.7 141.6,141.9 141.8,141.8 Z" fill="currentColor" class="github-corner__octo-body"/>
            </svg>
        </a>

        <h1 class="home-page__title">"MC Plugin Finder"</h1>
        <h2 class="home-page__subtitle">
          <span>"Search for Minecraft server plugins on "</span>
          <a href="https://www.spigotmc.org" target="_blank">Spigot</a>
          <span>", "</span>
          <a href="https://modrinth.com/plugins" target="_blank">Modrinth</a>
          <span>", and "</span>
          <a href="https://hangar.papermc.io" target="_blank">Hangar</a>
          <span>"."</span>
        </h2>

        <div class="home-page__container">
            <SearchForm params_memo />
            <SearchResults params_memo resource=search_resource />
        </div>
        <div class="home-page__footer">
            <div class="home-page__footer-last-ingest">
                <span>"Last Ingest: "</span>
                <Transition fallback=move || view! { <span>"Loading..."</span> }>
                    <span>{last_ingest_date}</span>
                </Transition>
            </div>
            <div class="home-page__footer-text">
                <span>MC Plugin Finder v{VERSION}</span>
                <span>" | "</span>
                <span>"© Frumple"</span>
                <span>" | "</span>
                <a href="https://github.com/Frumple/mc-plugin-finder" aria-label="View source on GitHub" target="_blank">GitHub</a>
            </div>
            <div class="home-page__footer-disclaimer">"MC Plugin Finder is not an official Minecraft service, and is not approved or associated with Mojang, Microsoft, SpigotMC, Modrinth, or PaperMC."</div>
        </div>
    }
}

/// Provides controls for performing a search.
#[component]
fn SearchForm(
    /// Memo that tracks the URL query parameters for the search.
    params_memo: Memo<Result<WebSearchParams, ParamsError>>
) -> impl IntoView {
    let params = move || params_memo.get().unwrap_or_default();

    view! {
        <Form action="" class="search-form">
            <input type="text" name="query" class="search-form__query-input" oninput="submitFormDebounce(this.form)" placeholder="Enter search query here..." value=move || params().query />

            <span class="search-form__repository-text">"Repository:"</span>

            <input id="spigot-checkbox" type="checkbox" name="spigot" class="search-form__spigot-checkbox" value="true" oninput="this.form.requestSubmit()" checked=move || params().spigot />
            <label for="spigot-checkbox" class="search-form__spigot-label">Spigot</label>

            <input id="modrinth-checkbox" type="checkbox" name="modrinth" class="search-form__modrinth-checkbox" value="true" oninput="this.form.requestSubmit()" checked=move || params().modrinth />
            <label for="modrinth-checkbox" class="search-form__modrinth-label">Modrinth</label>

            <input id="hangar-checkbox" type="checkbox" name="hangar" class="search-form__hangar-checkbox" value="true" oninput="this.form.requestSubmit()" checked=move || params().hangar />
            <label for="hangar-checkbox" class="search-form__hangar-label">Hangar</label>

            <span class="search-form__fields-text">"Fields:"</span>

            <input id="name-checkbox" type="checkbox" name="name" class="search-form__name-checkbox" value="true" oninput="this.form.requestSubmit()" checked=move || params().name />
            <label for="name-checkbox" class="search-form__name-label">Name</label>

            <input id="description-checkbox" type="checkbox" name="description" class="search-form__description-checkbox" value="true" oninput="this.form.requestSubmit()" checked=move || params().description />
            <label for="description-checkbox" class="search-form__description-label">Description</label>

            <input id="author-checkbox" type="checkbox" name="author" class="search-form__author-checkbox" value="true" oninput="this.form.requestSubmit()" checked=move || params().author />
            <label for="author-checkbox" class="search-form__author-label">Author</label>

            <div class="search-form__sort-limit-container">
                <label for="sort-select" class="search-form__sort-label">"Sort by:"</label>
                <select id="sort-select" name="sort" class="search-form__sort-select" onchange="this.form.requestSubmit()" prop:value=move || params().sort>
                    <option value="relevance">Relevance</option>
                    <option value="date_created">Newest</option>
                    <option value="date_updated">Recently Updated</option>
                    <option value="latest_minecraft_version">Latest MC Version</option>
                    <option value="downloads">Downloads</option>
                    <option value="likes_and_stars">Likes + Stars</option>
                    <option value="follows_and_watchers">Follows + Watchers</option>
                </select>

                <label for="limit-select" class="search-form__limit-label">"Show per page:"</label>
                <select id="limit-select" name="limit" class="search-form__limit-select" onchange="this.form.requestSubmit()" prop:value=move || params().limit>
                    <option value="25">25</option>
                    <option value="50">50</option>
                    <option value="100">100</option>
                </select>
            </div>

            <input type="hidden" name="page" value="1" />
        </Form>
    }
}

/// Displays projects matching the given search.
#[component]
fn SearchResults(
    /// Memo that tracks the URL query parameters for the search.
    params_memo: Memo<Result<WebSearchParams, ParamsError>>,
    /// The resource that performs the search when the search form is submitted.
    resource: Resource<Result<WebSearchParams, ParamsError>, Result<WebSearchResponse, ServerFnError>>
) -> impl IntoView {
    view! {
        <Transition fallback=move || view! { <div class="search-results__loading">"Loading..."</div> }>
            <ErrorBoundary fallback=|errors| view!{<ErrorTemplate errors=errors/>}>
                {move || {
                    let headers = {
                        move || {
                            params_memo.get()
                                .map(move |params| {
                                    let show_spigot = params.spigot.unwrap_or(false);
                                    let show_modrinth = params.modrinth.unwrap_or(false);
                                    let show_hangar = params.hangar.unwrap_or(false);

                                    view! {
                                        <div class="search-results__header-row">
                                            <h3 class="search-results__created-header">Created</h3>
                                            <h3 class="search-results__updated-header">Updated</h3>
                                            <h3 class="search-results__latest-minecraft-version-header">Latest MC Version</h3>
                                            <h3 class="search-results__downloads-header">Downloads</h3>
                                            <h3 class="search-results__likes-and-stars-header">Likes + Stars</h3>
                                            <h3 class="search-results__follows-and-watchers-header">Follows + Watchers</h3>
                                            <Show when=move || { show_spigot }>
                                                <h3 class="search-results__spigot-header">Spigot</h3>
                                            </Show>
                                            <Show when=move || { show_modrinth }>
                                                <h3 class="search-results__modrinth-header">Modrinth</h3>
                                            </Show>
                                            <Show when=move || { show_hangar }>
                                                <h3 class="search-results__hangar-header">Hangar</h3>
                                            </Show>
                                            <h3 class="search-results__source-header">Source Code</h3>
                                        </div>
                                    }
                                })
                        }
                    };

                    let results = {
                        move || {
                            resource.get()
                                .map(move |response| match response {
                                    Err(e) => {
                                        view! {<pre class="error">"Server Error: " {e.to_string()}</pre>}.into_view()
                                    }
                                    Ok(response) => {
                                        if response.results.is_empty() {
                                            view! { <div class="search-results__no-projects-found">"No projects were found."</div> }.into_view()
                                        } else {
                                            let full_count = response.results[0].full_count;

                                            let rows = response.results
                                                .into_iter()
                                                .map(move |project| {
                                                    view! {
                                                        <SearchRow params_memo search_result=project />
                                                    }
                                                })
                                                .collect_view();

                                            view! {
                                                <nav class="search-results__pagination">
                                                    <PaginationControls params_memo full_count />
                                                </nav>
                                                {headers}
                                                <ul class="search-results__list">
                                                    {rows}
                                                </ul>
                                                <nav class="search-results__pagination">
                                                    <PaginationControls params_memo full_count />
                                                </nav>
                                            }.into_view()
                                        }
                                    }
                                })
                                .unwrap_or_default()
                        }
                    };

                    view! {
                        <div class="search-results__container">
                            {results}
                        </div>
                    }
                }
            }
            </ErrorBoundary>
        </Transition>
    }
}

/// Pagination controls for the search results.
#[component]
fn PaginationControls(
    /// Memo that tracks the URL query parameters for the search.
    params_memo: Memo<Result<WebSearchParams, ParamsError>>,
    /// Total number of projects in the search results.
    full_count: i64
) -> impl IntoView {

    // For some unknown reason, the following line still causes a browser warning that states
    // we're accessing a signal or memo outside of a reactive tracking context (i.e. move || <closure>).
    let params = move || params_memo.get().unwrap_or_default();

    let page = move || params().page.unwrap_or_default();
    let total_pages = move || params().total_pages(full_count).unwrap_or_default();

    let first_url = move || params().first_url();
    let previous_url = move || params().previous_url();
    let current_url = move || params().form_url();
    let next_url = move || params().next_url();
    let last_url = move || params().last_url(full_count);

    view! {
        <Show when=move || { page() > 1 }>
            <a class="search-results__pagination-link-large" href=previous_url()>"<"</a>
            <a class="search-results__pagination-link" href=first_url()>"1"</a>
        </Show>

        <Show when=move || { page() > 3 }>
            <span class="search-results__pagination-item">"—"</span>
        </Show>

        <Show when=move || { page() > 2 }>
            <a class="search-results__pagination-link" href=previous_url()>{move || page() - 1}</a>
        </Show>

        <a class="search-results__pagination-link_current" href=current_url()>{move || page()}</a>

        <Show when=move || { page() < total_pages().saturating_sub(1) }>
            <a class="search-results__pagination-link" href=next_url()>{move || page() + 1}</a>
        </Show>

        <Show when=move || { page() < total_pages().saturating_sub(2) }>
            <span class="search-results__pagination-item">"—"</span>
        </Show>

        <Show when=move || { page() < total_pages() }>
            <a class="search-results__pagination-link" href=last_url()>{move || total_pages()}</a>
            <a class="search-results__pagination-link-large" href=next_url()>">"</a>
        </Show>
    }
}

/// A single row in the search results.
#[component]
fn SearchRow(
    /// Memo that tracks the URL query parameters for the search.
    params_memo: Memo<Result<WebSearchParams, ParamsError>>,
    /// The search result representing this row.
    search_result: WebSearchResult
) -> impl IntoView {
    let params = move || params_memo.get();

    let show_spigot = move || params().as_ref().map(|p| p.spigot.unwrap_or(false)).unwrap_or(false);
    let show_modrinth = move || params().as_ref().map(|p| p.modrinth.unwrap_or(false)).unwrap_or(false);
    let show_hangar = move || params().map(|p| p.hangar.unwrap_or(false)).unwrap_or(false);

    let date_created = search_result.date_created.format(&SEARCH_RESULT_DATE_FORMAT_DESCRIPTION);
    let time_created = search_result.date_created.format(&SEARCH_RESULT_TIME_FORMAT_DESCRIPTION);

    let date_updated = search_result.date_updated.format(&SEARCH_RESULT_DATE_FORMAT_DESCRIPTION);
    let time_updated = search_result.date_updated.format(&SEARCH_RESULT_TIME_FORMAT_DESCRIPTION);

    let latest_minecraft_version = search_result.latest_minecraft_version.clone().unwrap_or("N/A".to_string());

    let downloads = search_result.downloads_formatted();
    let likes_and_stars = search_result.likes_and_stars_formatted();
    let follows_and_watchers = search_result.follows_and_watchers_formatted();

    let project_name = search_result.project_name();

    let spigot = search_result.spigot;
    let modrinth = search_result.modrinth;
    let hangar = search_result.hangar;
    let source_repository = search_result.source_repository;

    let has_source = source_repository.is_some();

    view! {
        <li class="search-row__list-item">
            <div class="search-row__created-cell">
                <div class="search-row__date">{date_created}</div>
                <div class="search-row__time">{time_created}</div>
            </div>

            <div class="search-row__updated-cell">
                <div class="search-row__date">{date_updated}</div>
                <div class="search-row__time">{time_updated}</div>
            </div>

            <div class="search-row__latest-minecraft-version-cell">
                <span class="search-row__latest-minecraft-version">{latest_minecraft_version}</span>
            </div>

            <div class="search-row__downloads-cell">
                <span class="search-row__downloads">{downloads}</span>
            </div>

            <div class="search-row__likes-and-stars-cell">
                <span class="search-row__likes-and-stars">{likes_and_stars}</span>
            </div>

            <div class="search-row__follows-and-watchers-cell">
                <span class="search-row__follows-and-watchers">{follows_and_watchers}</span>
            </div>

            <Show when=move || { show_spigot() }>
                <div class="search-row__spigot-cell">
                    <SpigotResourceOuter spigot=spigot.clone() />
                </div>
            </Show>

            <Show when=move || { show_modrinth() }>
                <div class="search-row__modrinth-cell">
                    <ModrinthProjectOuter modrinth=modrinth.clone() />
                </div>
            </Show>

            <Show when=move || { show_hangar() }>
                <div class="search-row__hangar-cell">
                    <HangarProjectOuter hangar=hangar.clone() />
                </div>
            </Show>

            <div class="search-row__source-cell">
                <Show when=move || { has_source }>
                    <SourceRepository repo=source_repository.clone().unwrap() project_name=project_name.clone() />
                </Show>
            </div>
        </li>
    }
}

// These outer components are necessary to ensure that the Spigot/Modrinth/Hangar objects are not moved out of their environment when using nested <Show>s.
// TODO: Find a way to do this without needing outer components.

#[component]
fn SpigotResourceOuter(
    spigot: Option<WebSearchResultSpigot>
) -> impl IntoView {
    let has_spigot = spigot.is_some();

    view! {
        <Show when=move || { has_spigot }>
            <SpigotResourceInner spigot=spigot.clone().unwrap() />
        </Show>
    }
}

#[component]
fn ModrinthProjectOuter(
    modrinth: Option<WebSearchResultModrinth>
) -> impl IntoView {
    let has_modrinth = modrinth.is_some();

    view! {
        <Show when=move || { has_modrinth }>
            <ModrinthProjectInner modrinth=modrinth.clone().unwrap() />
        </Show>
    }
}

#[component]
fn HangarProjectOuter(
    hangar: Option<WebSearchResultHangar>
) -> impl IntoView {
    let has_hangar = hangar.is_some();

    view! {
        <Show when=move || { has_hangar }>
            <HangarProjectInner hangar=hangar.clone().unwrap() />
        </Show>
    }
}

#[component]
fn SpigotResourceInner(
    spigot: WebSearchResultSpigot
) -> impl IntoView {
    let is_premium = spigot.premium;
    let is_abandoned = spigot.abandoned;

    let name = spigot.name.clone();
    let url = spigot.url();
    let icon_img_url = spigot.icon_img_url();
    let icon_alt_text = spigot.icon_alt_text();
    let version = spigot.version;
    let author = spigot.author;
    let description = spigot.description;

    view! {
        <a class="search-row__spigot-link" href=url.clone() target="_blank">
            <img class="search-row__image" src=icon_img_url.clone() title=name.clone() alt=icon_alt_text.clone() width="75" height="75" loading="lazy" />
            <div class="search-row__text-contents">
                <div class="search-row__cell-title">
                    <Show when=move || { is_abandoned }>
                        <img class="search-row__plugin-abandoned" src=ABANDONED_IMAGE_URL title="Abandoned Plugin" alt="Warning icon indicating that this plugin has been abandoned by the developer" width="24" height="24" loading="lazy" />
                    </Show>
                    <Show when=move || { is_premium }>
                        <img class="search-row__plugin-premium" src=PREMIUM_IMAGE_URL title="Premium Plugin (requires Spigot login)" alt="Dollar sign icon indicating this plugin requires payment to download" width="24" height="24" loading="lazy" />
                    </Show>
                    <h3 class="search-row__plugin-name">{name.clone()}</h3>
                    <span>"  "</span>
                    <span class="search-row__plugin-version">{version.clone()}</span>
                    <span>" by "</span>
                    <span class="search-row__plugin-author">{author.clone()}</span>
                </div>
                <div class="search-row__cell-description">
                    {description.clone()}
                </div>
            </div>
        </a>
    }
}

/// A project hosted on Modrinth
#[component]
fn ModrinthProjectInner(
    /// The Modrinth portion of the search result
    modrinth: WebSearchResultModrinth
) -> impl IntoView {
    let is_archived = modrinth.is_archived();

    let name = modrinth.name.clone();
    let url = modrinth.url();
    let icon_img_url = modrinth.icon_img_url();
    let icon_alt_text = modrinth.icon_alt_text();
    let version = modrinth.version;
    let author = modrinth.author;
    let description = modrinth.description;

    view! {
        <a class="search-row__modrinth-link" href=url.clone() target="_blank">
            <img class="search-row__image" src=icon_img_url.clone() title=name.clone() alt=icon_alt_text.clone() width="75" height="75" loading="lazy" />
            <div class="search-row__text-contents">
                <div class="search-row__cell-title">
                    <Show when=move || { is_archived }>
                        <img class="search-row__plugin-abandoned" src=ABANDONED_IMAGE_URL title="Archived Plugin" alt="Warning icon indicating that this plugin has been archived by the developer" width="24" height="24" loading="lazy" />
                    </Show>
                    <h3 class="search-row__plugin-name">{name.clone()}</h3>
                    <span>" "</span>
                    <span class="search-row__plugin-version">{version.clone()}</span>
                    <span>" by "</span>
                    <span class="search-row__plugin-author">{author.clone()}</span>
                </div>
                <div class="search-row__cell-description">
                    {description.clone()}
                </div>
            </div>
        </a>
    }
}

/// A project hosted on Hangar
#[component]
fn HangarProjectInner(
    /// The Hangar portion of the search result
    hangar: WebSearchResultHangar
) -> impl IntoView {
    let name = hangar.name.clone();
    let url = hangar.url();
    let icon_img_url = hangar.icon_img_url();
    let icon_alt_text = hangar.icon_alt_text();
    let version = hangar.version;
    let author = hangar.author;
    let description = hangar.description;

    view! {
        <a class="search-row__hangar-link" href=url.clone() target="_blank">
            <img class="search-row__image" src=icon_img_url.clone() title=name.clone() alt=icon_alt_text.clone() width="75" height="75" loading="lazy" />
            <div class="search-row__text-contents">
                <div class="search-row__cell-title">
                    <h3 class="search-row__plugin-name">{name.clone()}</h3>
                    <span>" "</span>
                    <span class="search-row__plugin-version">{version.clone()}</span>
                    <span>" by "</span>
                    <span class="search-row__plugin-author">{author.clone()}</span>
                </div>
                <div class="search-row__cell-description">
                    {description.clone()}
                </div>
            </div>
        </a>
    }
}

/// The source code repository
#[component]
fn SourceRepository(
    /// The source repository portion of the search result
    repo: WebSearchResultSourceRepository,
    /// The name of the project
    project_name: Option<String>
) -> impl IntoView {
    let url = repo.url();
    let url_wbr = repo.url_wbr();
    let img_attributes = repo.img_attributes(&project_name);

    view! {
        <a class="search-row__source-link" href=url.clone() target="_blank">
            <img class="search-row__image" src=img_attributes.src.clone() title=img_attributes.title.clone() alt=img_attributes.alt.clone() width="75" height="75" loading="lazy" />
            <div class="search-row__source-text-contents">
                <div class="search-row__cell-title" inner_html=url_wbr.clone()>
                </div>
            </div>
        </a>
    }
}

struct ImgAttributes {
    src: Option<String>,
    title: Option<String>,
    alt: Option<String>
}

fn alt_text(project_name: &Option<String>, repository_name: &str) -> Option<String> {
    Some(format!("Icon for {} on {}", project_name.clone()?, repository_name))
}