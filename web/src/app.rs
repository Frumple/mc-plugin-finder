use crate::error_template::{AppError, ErrorTemplate};
use crate::util::format_number;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use serde::{Serialize, Deserialize};
use std::str::FromStr;
use time::{OffsetDateTime, format_description};

#[cfg(feature = "ssr")]
use mc_plugin_finder::database::common::project::{CommonProjectSearchResult, SearchParams, SearchParamsSort};

const NO_ICON_IMAGE_URL: &str = "images/no-icon.svg";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebSearchResult {
    pub full_count: i64,

    pub id: i32,
    pub date_created: OffsetDateTime,
    pub date_updated: OffsetDateTime,
    pub latest_minecraft_version: Option<String>,
    pub downloads: i32,
    pub likes_and_stars: i32,
    pub follows_and_watchers: i32,

    pub spigot: Option<WebSearchResultSpigot>,
    pub modrinth: Option<WebSearchResultModrinth>,
    pub hangar: Option<WebSearchResultHangar>,

    pub source_repository_host: Option<String>,
    pub source_repository_owner: Option<String>,
    pub source_repository_name: Option<String>
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

    fn source_repository_url(&self) -> Option<String> {
        Some(format!("https://{}/{}/{}", self.source_repository_host.clone()?, self.source_repository_owner.clone()?, self.source_repository_name.clone()?))
    }

    fn source_repository_url_wbr(&self) -> Option<String> {
        Some(format!("https://{}/<wbr>{}/<wbr>{}", self.source_repository_host.clone()?, self.source_repository_owner.clone()?, self.source_repository_name.clone()?))
    }

    fn source_img_attributes(&self) -> ImgAttributes {
        if let Some(host) = &self.source_repository_host {
            return match host.as_str() {
                "github.com" => ImgAttributes {
                    src: Some("images/github-logo.svg".to_string()),
                    title: Some("GitHub".to_string()),
                    alt: alt_text(&self.project_name(), "GitHub")
                },
                "gitlab.com" => ImgAttributes {
                    src: Some("images/gitlab-logo.svg".to_string()),
                    title: Some("GitLab".to_string()),
                    alt: alt_text(&self.project_name(), "GitLab")
                },
                "bitbucket.org" => ImgAttributes {
                    src: Some("images/bitbucket-logo.svg".to_string()),
                    title: Some("Bitbucket".to_string()),
                    alt: alt_text(&self.project_name(), "Bitbucket")
                },
                "codeberg.org" => ImgAttributes {
                    src: Some("images/codeberg-logo.svg".to_string()),
                    title: Some("Codeberg".to_string()),
                    alt: alt_text(&self.project_name(), "Codeberg")
                },
                _ => ImgAttributes {
                    src: Some(NO_ICON_IMAGE_URL.to_string()),
                    title: Some("Unknown Repository".to_string()),
                    alt: Some("Unknown Repository".to_string())
                }
            }
        }
        ImgAttributes {
            src: Some(NO_ICON_IMAGE_URL.to_string()),
            title: Some("No Repository".to_string()),
            alt: Some("No Repository".to_string())
        }
    }

}

#[cfg(feature = "ssr")]
impl From<CommonProjectSearchResult> for WebSearchResult {
    fn from(search_result: CommonProjectSearchResult) -> Self {
        let spigot = search_result.spigot.map(|s| WebSearchResultSpigot {
            id: s.id,
            slug: s.slug,
            name: s.name,
            description: s.description,
            author: s.author,
            version: s.version,
            premium: s.premium,
            icon_data: s.icon_data,
        });

        let modrinth = search_result.modrinth.map(|m| WebSearchResultModrinth {
            id: m.id,
            slug: m.slug,
            name: m.name,
            description: m.description,
            author: m.author,
            version: m.version,
            icon_url: m.icon_url,
        });

        let hangar = search_result.hangar.map(|h| WebSearchResultHangar {
            slug: h.slug,
            name: h.name,
            description: h.description,
            author: h.author,
            version: h.version,
            avatar_url: h.avatar_url,
        });

        WebSearchResult {
            full_count: search_result.full_count,

            id: search_result.id,
            date_created: search_result.date_created,
            date_updated: search_result.date_updated,
            latest_minecraft_version: search_result.latest_minecraft_version,
            downloads: search_result.downloads,
            likes_and_stars: search_result.likes_and_stars,
            follows_and_watchers: search_result.follows_and_watchers,

            spigot,
            modrinth,
            hangar,

            source_repository_host: search_result.source_repository_host,
            source_repository_owner: search_result.source_repository_owner,
            source_repository_name: search_result.source_repository_name
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
    pub icon_data: Option<String>
}

impl WebSearchResultSpigot {
    fn url(&self) -> Option<String> {
        Some(format!("https://spigotmc.org/resources/{}", self.slug))
    }

    fn icon_img_url(&self) -> String {
        // Fallback to a "no-icon" placeholder image if the incoming icon data is None or an empty string.
        if self.icon_data.is_none() || self.icon_data.as_ref().is_some_and(|x| x.is_empty()) {
            return NO_ICON_IMAGE_URL.to_string()
        }

        format!("data:image/png;base64,{}", self.icon_data.clone().unwrap())
    }

    fn icon_alt_text(&self) -> Option<String> {
        alt_text(&self.name, "Spigot")
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
    pub icon_url: Option<String>
}

impl WebSearchResultModrinth {
    fn url(&self) -> Option<String> {
        Some(format!("https://modrinth.com/plugin/{}", self.slug))
    }

    fn icon_img_url(&self) -> String {
        fallback_if_no_icon(&self.icon_url.clone())
    }

    fn icon_alt_text(&self) -> Option<String> {
        alt_text(&Some(self.name.clone()), "Modrinth")
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebSearchResultHangar {
    pub slug: String,
    pub name: String,
    pub description: String,
    pub author: String,
    pub version: Option<String>,
    pub avatar_url: String
}

impl WebSearchResultHangar {
    fn url(&self) -> Option<String> {
        Some(format!("https://hangar.papermc.io/{}/{}", self.author, self.slug))
    }

    fn avatar_img_url(&self) -> String {
        fallback_if_no_icon(&Some(self.avatar_url.clone()))
    }

    fn icon_alt_text(&self) -> Option<String> {
        alt_text(&Some(self.name.clone()), "Hangar")
    }
}

fn fallback_if_no_icon(icon_url: &Option<String>) -> String {
    // Fallback to a "no-icon" placeholder image if the incoming icon URL is None or an empty string.
    if icon_url.is_none() || icon_url.as_ref().is_some_and(|x| x.is_empty()) {
        return NO_ICON_IMAGE_URL.to_string()
    }

    icon_url.clone().unwrap()
}

fn alt_text(project_name: &Option<String>, repository_name: &str) -> Option<String> {
    Some(format!("Icon for {} on {}", project_name.clone()?, repository_name))
}

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
            "query=&spigot=true&modrinth=true&hangar=true&name=true&sort=downloads&limit=25&page=1".to_string())
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
            sort: Some("downloads".to_string()),
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

struct ImgAttributes {
    src: Option<String>,
    title: Option<String>,
    alt: Option<String>
}

#[cfg(feature = "ssr")]
pub mod ssr {
    use mc_plugin_finder::database::get_db;
    use deadpool_postgres::{CreatePoolError, Pool};

    const LIVE_DB_NAME: &str = "mc_plugin_finder";

    pub async fn db() -> Result<Pool, CreatePoolError> {
        // TODO: Is there a way to initialize the database client globally instead of per request?
        let db = get_db();
        db.create_pool(LIVE_DB_NAME).await
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
pub async fn search_projects(params: WebSearchParams) -> Result<Vec<WebSearchResult>, ServerFnError> {
    use self::ssr::*;
    use mc_plugin_finder::database::common::project::search_common_projects;

    let db_pool = db().await?;
    let common_projects = search_common_projects(&db_pool, &params.into()).await;

    match common_projects {
        Ok(projects) => Ok(projects.into_iter().map(|x| x.into()).collect()),
        Err(error) => Err(ServerFnError::ServerError(error.to_string()))
    }
}

async fn fetch_projects(params_result: Result<WebSearchParams, ParamsError>) -> Result<Vec<WebSearchResult>, ServerFnError> {
    match params_result {
        Ok(params) => search_projects(params).await,
        Err(error) => Err(ServerFnError::ServerError(error.to_string()))
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let params_memo_original = use_query::<WebSearchParams>();

    let params_memo = create_memo(move |_| {
        params_memo_original().map( |params| {
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
                    sort: Some("downloads".to_string()),
                    limit: Some(25),
                    page: Some(1)
                }
            }
            params
        })
    });

    let resource = create_resource(
        params_memo,
        fetch_projects
    );

    view! {
        <h1 class="home-page__title">"MC Plugin Finder"</h1>

        <div class="home-page__container">
            <SearchForm params_memo />
            <SearchResults params_memo resource />
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
            <input type="text" name="query" class="search-form__query-input" oninput="submitFormDebounce(this.form)" value=move || params().query />

            <span class="search-form__repository-text">Repository:</span>

            <input id="spigot-checkbox" type="checkbox" name="spigot" class="search-form__spigot-checkbox" value="true" oninput="this.form.requestSubmit()" checked=move || params().spigot />
            <label for="spigot-checkbox" class="search-form__spigot-label">Spigot</label>

            <input id="modrinth-checkbox" type="checkbox" name="modrinth" class="search-form__modrinth-checkbox" value="true" oninput="this.form.requestSubmit()" checked=move || params().modrinth />
            <label for="modrinth-checkbox" class="search-form__modrinth-label">Modrinth</label>

            <input id="hangar-checkbox" type="checkbox" name="hangar" class="search-form__hangar-checkbox" value="true" oninput="this.form.requestSubmit()" checked=move || params().hangar />
            <label for="hangar-checkbox" class="search-form__hangar-label">Hangar</label>

            <span class="search-form__fields-text">Fields:</span>

            <input id="name-checkbox" type="checkbox" name="name" class="search-form__name-checkbox" value="true" oninput="this.form.requestSubmit()" checked=move || params().name />
            <label for="name-checkbox" class="search-form__name-label">Name</label>

            <input id="description-checkbox" type="checkbox" name="description" class="search-form__description-checkbox" value="true" oninput="this.form.requestSubmit()" checked=move || params().description />
            <label for="description-checkbox" class="search-form__description-label">Description</label>

            <input id="author-checkbox" type="checkbox" name="author" class="search-form__author-checkbox" value="true" oninput="this.form.requestSubmit()" checked=move || params().author />
            <label for="author-checkbox" class="search-form__author-label">Author</label>

            <div class="search-form__sort-limit-container">
                <label for="sort-select" class="search-form__sort-label">Sort by:</label>
                <select id="sort-select" name="sort" class="search-form__sort-select" onchange="this.form.requestSubmit()" prop:value=move || params().sort>
                    <option value="date_created">Newest</option>
                    <option value="date_updated">Recently Updated</option>
                    <option value="latest_minecraft_version">Latest MC Version</option>
                    <option value="downloads">Downloads</option>
                    <option value="likes_and_stars">Likes + Stars</option>
                    <option value="follows_and_watchers">Follows + Watchers</option>
                </select>

                <label for="limit-select" class="search-form__limit-label">Show per page:</label>
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
    resource: Resource<Result<WebSearchParams, ParamsError>, Result<Vec<WebSearchResult>, ServerFnError>>
) -> impl IntoView {
    view! {
        <Transition fallback=move || view! { <p>"Loading..."</p> }>
            <ErrorBoundary fallback=|errors| view!{<ErrorTemplate errors=errors/>}>
                {move || {
                    let results = {
                        move || {
                            resource.get()
                                .map(move |projects| match projects {
                                    Err(e) => {
                                        view! {<pre class="error">"Server Error: " {e.to_string()}</pre>}.into_view()
                                    }
                                    Ok(projects) => {
                                        if projects.is_empty() {
                                            view! { <p>"No projects were found."</p> }.into_view()
                                        } else {
                                            let full_count = projects[0].full_count;

                                            let rows = projects
                                                .into_iter()
                                                .map(move |project| {
                                                    view! {
                                                        <SearchRow search_result=project />
                                                    }
                                                })
                                                .collect_view();

                                            let pagination = {
                                                move || {
                                                    params_memo.get()
                                                        .map(move |params| {
                                                            let page = params.page.unwrap_or_default();
                                                            let total_pages = params.total_pages(full_count).unwrap_or_default();

                                                            let first_url = params.first_url();
                                                            let previous_url = params.previous_url();
                                                            let previous_url_clone = previous_url.clone();

                                                            let current_url = params.form_url();

                                                            let next_url = params.next_url();
                                                            let next_url_clone = next_url.clone();
                                                            let last_url = params.last_url(full_count);

                                                            view! {
                                                                <Show when=move || { page > 1 }>
                                                                    <a class="search-results__pagination-link" href=previous_url.clone()>"<"</a>
                                                                    <a class="search-results__pagination-link" href=first_url.clone()>"1"</a>
                                                                </Show>

                                                                <Show when=move || { page > 3 }>
                                                                    <span class="search-results__pagination-item">"—"</span>
                                                                </Show>

                                                                <Show when=move || { page > 2 }>
                                                                    <a class="search-results__pagination-link" href=previous_url_clone.clone()>{page - 1}</a>
                                                                </Show>

                                                                <a class="search-results__pagination-link_current" href=current_url.clone()>{page}</a>

                                                                <Show when=move || { page < total_pages.saturating_sub(1) }>
                                                                    <a class="search-results__pagination-link" href=next_url_clone.clone()>{page + 1}</a>
                                                                </Show>

                                                                <Show when=move || { page < total_pages.saturating_sub(2) }>
                                                                    <span class="search-results__pagination-item">"—"</span>
                                                                </Show>

                                                                <Show when=move || { page < total_pages }>
                                                                    <a class="search-results__pagination-link" href=last_url.clone()>{total_pages}</a>
                                                                    <a class="search-results__pagination-link" href=next_url.clone()>">"</a>
                                                                </Show>
                                                            }
                                                        })
                                                }
                                            };

                                            view! {
                                                <ul class="search-results__list">
                                                    {rows}
                                                </ul>
                                                <nav class="search-results__pagination">
                                                    {pagination}
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
                            <div class="search-results__header-row">
                                <span class="search-results__created-header">Created</span>
                                <span class="search-results__updated-header">Updated</span>
                                <span class="search-results__latest-minecraft-version-header">Latest MC Version</span>
                                <span class="search-results__downloads-header">Downloads</span>
                                <span class="search-results__likes-and-stars-header">Likes + Stars</span>
                                <span class="search-results__follows-and-watchers-header">Follows + Watchers</span>
                                <span class="search-results__spigot-header">Spigot</span>
                                <span class="search-results__modrinth-header">Modrinth</span>
                                <span class="search-results__hangar-header">Hangar</span>
                                <span class="search-results__source-header">Source Code</span>
                            </div>
                            {results}
                        </div>
                    }
                }
            }
            </ErrorBoundary>
        </Transition>
    }
}

/// A single row in the search results.
#[component]
fn SearchRow(
    /// The search result representing this row.
    search_result: WebSearchResult
) -> impl IntoView {
    let date_format = format_description::parse("[year]-[month]-[day]").unwrap();
    let time_format = format_description::parse("[hour]:[minute]:[second]").unwrap();

    let date_created = search_result.date_created.format(&date_format);
    let time_created = search_result.date_created.format(&time_format);

    let date_updated = search_result.date_updated.format(&date_format);
    let time_updated = search_result.date_updated.format(&time_format);

    let latest_minecraft_version = search_result.latest_minecraft_version.clone();

    let downloads = search_result.downloads_formatted();
    let likes_and_stars = search_result.likes_and_stars_formatted();
    let follows_and_watchers = search_result.follows_and_watchers_formatted();

    let has_source = search_result.source_repository_name.is_some();
    let source_repository_url = search_result.source_repository_url();
    let source_repository_url_wbr = search_result.source_repository_url_wbr();
    let source_img_attributes = search_result.source_img_attributes();

    let spigot = search_result.spigot;
    let modrinth = search_result.modrinth;
    let hangar = search_result.hangar;

    // TODO: Refactor this into separate components

    let has_spigot = spigot.is_some();
    let is_spigot_premium = spigot.as_ref().and_then(|s| Some(s.premium)).unwrap_or_default();
    let spigot_name = spigot.as_ref().and_then(|s| s.name.clone());
    let spigot_url = spigot.as_ref().and_then(|s| s.url());
    let spigot_icon_img_url = spigot.as_ref().and_then(|s| Some(s.icon_img_url()));
    let spigot_icon_alt_text = spigot.as_ref().and_then(|s| s.icon_alt_text());
    let spigot_version = spigot.as_ref().and_then(|s| s.version.clone());
    let spigot_author = spigot.as_ref().and_then(|s| Some(s.author.clone()));
    let spigot_description = spigot.as_ref().and_then(|s| Some(s.description.clone()));

    let has_modrinth = modrinth.is_some();
    let modrinth_name = modrinth.as_ref().and_then(|m| Some(m.name.clone()));
    let modrinth_url = modrinth.as_ref().and_then(|m| m.url());
    let modrinth_icon_img_url = modrinth.as_ref().and_then(|m| Some(m.icon_img_url()));
    let modrinth_icon_alt_text = modrinth.as_ref().and_then(|m| m.icon_alt_text());
    let modrinth_version = modrinth.as_ref().and_then(|m| m.version.clone());
    let modrinth_author = modrinth.as_ref().and_then(|m| Some(m.author.clone()));
    let modrinth_description = modrinth.as_ref().and_then(|m| Some(m.description.clone()));

    let has_hangar = hangar.is_some();
    let hangar_name = hangar.as_ref().and_then(|h| Some(h.name.clone()));
    let hangar_url = hangar.as_ref().and_then(|h| h.url());
    let hangar_avatar_img_url = hangar.as_ref().and_then(|h| Some(h.avatar_img_url()));
    let hangar_icon_alt_text = hangar.as_ref().and_then(|h| h.icon_alt_text());
    let hangar_version = hangar.as_ref().and_then(|h| h.version.clone());
    let hangar_author = hangar.as_ref().and_then(|h| Some(h.author.clone()));
    let hangar_description = hangar.as_ref().and_then(|h| Some(h.description.clone()));

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

            <div class="search-row__spigot-cell">
                <Show when=move || { has_spigot }>
                    <a class="search-row__spigot-link" href=spigot_url.clone() target="_blank">
                        <img class="search-row__image" src=spigot_icon_img_url.clone() title=spigot_name.clone() alt=spigot_icon_alt_text.clone() loading="lazy" />
                        <div class="search-row__text-contents">
                            <div class="search-row__cell-title">
                                <Show when=move || { is_spigot_premium }>
                                    <span class="search-row__plugin-premium">
                                        <svg xmlns="http://www.w3.org/2000/svg" height="20px" viewBox="0 -960 960 960" width="20px" fill="#000000" style="vertical-align: bottom;">
                                            <path d="M446-216h67v-47q49-8 81-42t32-79q0-45-27.5-77T514-514q-61-22-80.5-37.5T414-592q0-20 17.5-33t45.5-13q28 0 49 13.5t28 36.5l59-25q-12-33-38.5-55.5T513-697v-47h-66v48q-45 10-72 38.5T348-591q0 45 30.5 76.5T475-460q45 16 65.5 34t20.5 42q0 26-21 43.5T488-323q-33 0-58.5-22T395-402l-62 26q12 42 42 71.5t71 40.5v48Zm34 120q-79 0-149-30t-122.5-82.5Q156-261 126-331T96-480q0-80 30-149.5t82.5-122Q261-804 331-834t149-30q80 0 149.5 30t122 82.5Q804-699 834-629.5T864-480q0 79-30 149t-82.5 122.5Q699-156 629.5-126T480-96Zm0-72q130 0 221-91t91-221q0-130-91-221t-221-91q-130 0-221 91t-91 221q0 130 91 221t221 91Zm0-312Z"/>
                                        </svg>
                                    </span>
                                </Show>
                                <h3 class="search-row__plugin-name">{spigot_name.clone()}</h3>
                                <span>"  "</span>
                                <span class="search-row__plugin-version">{spigot_version.clone()}</span>
                                <span>" by "</span>
                                <span class="search-row__plugin-author">{spigot_author.clone()}</span>
                            </div>
                            <div class="search-row__cell-description">
                                {spigot_description.clone()}
                            </div>
                        </div>
                    </a>
                </Show>
            </div>

            <div class="search-row__modrinth-cell">
                <Show when=move || { has_modrinth }>
                    <a class="search-row__modrinth-link" href=modrinth_url.clone() target="_blank">
                        <img class="search-row__image" src=modrinth_icon_img_url.clone() title=modrinth_name.clone() alt=modrinth_icon_alt_text.clone() loading="lazy" />
                        <div class="search-row__text-contents">
                            <div class="search-row__cell-title">
                                <h3 class="search-row__plugin-name">{modrinth_name.clone()}</h3>
                                <span>" "</span>
                                <span class="search-row__plugin-version">{modrinth_version.clone()}</span>
                                <span>" by "</span>
                                <span class="search-row__plugin-author">{modrinth_author.clone()}</span>
                            </div>
                            <div class="search-row__cell-description">
                                {modrinth_description.clone()}
                            </div>
                        </div>
                    </a>
                </Show>
            </div>

            <div class="search-row__hangar-cell">
                <Show when=move || { has_hangar }>
                    <a class="search-row__hangar-link" href=hangar_url.clone() target="_blank">
                        <img class="search-row__image" src=hangar_avatar_img_url.clone() title=hangar_name.clone() alt=hangar_icon_alt_text.clone() loading="lazy" />
                        <div class="search-row__text-contents">
                            <div class="search-row__cell-title">
                                <h3 class="search-row__plugin-name">{hangar_name.clone()}</h3>
                                <span>" "</span>
                                <span class="search-row__plugin-version">{hangar_version.clone()}</span>
                                <span>" by "</span>
                                <span class="search-row__plugin-author">{hangar_author.clone()}</span>
                            </div>
                            <div class="search-row__cell-description">
                                {hangar_description.clone()}
                            </div>
                        </div>
                    </a>
                </Show>
            </div>

            <div class="search-row__source-cell">
                <Show when=move || { has_source }>
                    <a class="search-row__source-link" href=source_repository_url.clone() target="_blank">
                        <img class="search-row__image" src=source_img_attributes.src.clone() title=source_img_attributes.title.clone() alt=source_img_attributes.alt.clone() loading="lazy" />
                        <div class="search-row__source-text-contents">
                            <div class="search-row__cell-title" inner_html=source_repository_url_wbr.clone()>
                            </div>
                        </div>
                    </a>
                </Show>
            </div>
        </li>
    }
}
