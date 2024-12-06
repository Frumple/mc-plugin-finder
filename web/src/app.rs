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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebSearchResult {
    pub full_count: i64,

    pub id: Option<i32>,
    pub date_created: OffsetDateTime,
    pub date_updated: OffsetDateTime,
    pub latest_minecraft_version: Option<String>,
    pub downloads: i32,
    pub likes_and_stars: i32,
    pub follows_and_watchers: i32,

    pub spigot_id: Option<i32>,
    pub spigot_slug: Option<String>,
    pub spigot_name: Option<String>,
    pub spigot_description: Option<String>,
    pub spigot_author: Option<String>,
    pub spigot_version: Option<String>,
    pub spigot_premium: Option<bool>,
    pub spigot_icon_data: Option<String>,

    pub modrinth_id: Option<String>,
    pub modrinth_slug: Option<String>,
    pub modrinth_name: Option<String>,
    pub modrinth_description: Option<String>,
    pub modrinth_author: Option<String>,
    pub modrinth_version: Option<String>,
    pub modrinth_icon_url: Option<String>,

    pub hangar_slug: Option<String>,
    pub hangar_name: Option<String>,
    pub hangar_description: Option<String>,
    pub hangar_author: Option<String>,
    pub hangar_version: Option<String>,
    pub hangar_avatar_url: Option<String>,

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

    fn spigot_url(&self) -> Option<String> {
        Some(format!("https://spigotmc.org/resources/{}", self.spigot_slug.clone()?))
    }

    fn spigot_icon_display_url(&self) -> String {
        // Fallback to a "no-icon" placeholder image if the incoming icon data is None or an empty string.
        if self.spigot_icon_data.is_none() || self.spigot_icon_data.as_ref().is_some_and(|x| x.is_empty()) {
            return "images/no-icon.svg".to_string()
        }

        format!("data:image/png;base64,{}", self.spigot_icon_data.clone().unwrap())
    }

    fn modrinth_url(&self) -> Option<String> {
        Some(format!("https://modrinth.com/plugin/{}", self.modrinth_slug.clone()?))
    }

    fn modrinth_icon_display_url(&self) -> String {
        Self::fallback_if_no_icon(&self.modrinth_icon_url)
    }

    fn hangar_url(&self) -> Option<String> {
        Some(format!("https://hangar.papermc.io/{}/{}", self.hangar_author.clone()?, self.hangar_slug.clone()?))
    }

    fn hangar_avatar_display_url(&self) -> String {
        Self::fallback_if_no_icon(&self.hangar_avatar_url)
    }

    fn source_repository_url(&self) -> Option<String> {
        Some(format!("https://{}/{}/{}", self.source_repository_host.clone()?, self.source_repository_owner.clone()?, self.source_repository_name.clone()?))
    }

    fn source_repository_url_wbr(&self) -> Option<String> {
        Some(format!("https://{}/<wbr>{}/<wbr>{}", self.source_repository_host.clone()?, self.source_repository_owner.clone()?, self.source_repository_name.clone()?))
    }

    fn fallback_if_no_icon(icon_url: &Option<String>) -> String {
        // Fallback to a "no-icon" placeholder image if the incoming icon URL is None or an empty string.
        if icon_url.is_none() || icon_url.as_ref().is_some_and(|x| x.is_empty()) {
            return "images/no-icon.svg".to_string()
        }

        icon_url.clone().unwrap()
    }
}

#[cfg(feature = "ssr")]
impl From<CommonProjectSearchResult> for WebSearchResult {
    fn from(search_result: CommonProjectSearchResult) -> Self {
        WebSearchResult {
            full_count: search_result.full_count,

            id: search_result.project.id,
            date_created: search_result.date_created,
            date_updated: search_result.date_updated,
            latest_minecraft_version: search_result.latest_minecraft_version,
            downloads: search_result.downloads,
            likes_and_stars: search_result.likes_and_stars,
            follows_and_watchers: search_result.follows_and_watchers,

            spigot_id: search_result.project.spigot_id,
            spigot_slug: search_result.project.spigot_slug,
            spigot_name: search_result.project.spigot_name,
            spigot_description: search_result.project.spigot_description,
            spigot_author: search_result.project.spigot_author,
            spigot_version: search_result.project.spigot_version,
            spigot_premium: search_result.project.spigot_premium,
            spigot_icon_data: search_result.project.spigot_icon_data,

            modrinth_id: search_result.project.modrinth_id,
            modrinth_slug: search_result.project.modrinth_slug,
            modrinth_name: search_result.project.modrinth_name,
            modrinth_description: search_result.project.modrinth_description,
            modrinth_author: search_result.project.modrinth_author,
            modrinth_version: search_result.project.modrinth_version,
            modrinth_icon_url: search_result.project.modrinth_icon_url,

            hangar_slug: search_result.project.hangar_slug,
            hangar_name: search_result.project.hangar_name,
            hangar_description: search_result.project.hangar_description,
            hangar_author: search_result.project.hangar_author,
            hangar_version: search_result.project.hangar_version,
            hangar_avatar_url: search_result.project.hangar_avatar_url,

            source_repository_host: search_result.source_repository_host,
            source_repository_owner: search_result.source_repository_owner,
            source_repository_name: search_result.source_repository_name
        }
    }
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

    let has_spigot = search_result.spigot_name.is_some();
    let has_modrinth = search_result.modrinth_name.is_some();
    let has_hangar = search_result.hangar_name.is_some();

    let spigot_name = search_result.spigot_name.clone();
    let modrinth_name = search_result.modrinth_name.clone();
    let hangar_name = search_result.hangar_name.clone();

    let spigot_url = search_result.spigot_url();
    let modrinth_url = search_result.modrinth_url();
    let hangar_url = search_result.hangar_url();

    let spigot_icon_display_url = search_result.spigot_icon_display_url();
    let modrinth_icon_display_url = search_result.modrinth_icon_display_url();
    let hangar_avatar_display_url = search_result.hangar_avatar_display_url();

    let source_repository_url = search_result.source_repository_url();
    let source_repository_url_wbr = search_result.source_repository_url_wbr();

    let is_spigot_premium = search_result.spigot_premium.unwrap_or_default();

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
                    <a href=spigot_url.clone() target="_blank">
                        <img class="search-row__image" src=spigot_icon_display_url.clone() title=spigot_name.clone() loading="lazy" />
                    </a>
                    <div class="search-row__title-and-description">
                        <div class="search-row__cell-title">
                            <Show when=move || { is_spigot_premium }>
                                <span class="search-row__plugin-premium">
                                    <svg xmlns="http://www.w3.org/2000/svg" height="20px" viewBox="0 -960 960 960" width="20px" fill="#000000" style="vertical-align: bottom;">
                                        <path d="M446-216h67v-47q49-8 81-42t32-79q0-45-27.5-77T514-514q-61-22-80.5-37.5T414-592q0-20 17.5-33t45.5-13q28 0 49 13.5t28 36.5l59-25q-12-33-38.5-55.5T513-697v-47h-66v48q-45 10-72 38.5T348-591q0 45 30.5 76.5T475-460q45 16 65.5 34t20.5 42q0 26-21 43.5T488-323q-33 0-58.5-22T395-402l-62 26q12 42 42 71.5t71 40.5v48Zm34 120q-79 0-149-30t-122.5-82.5Q156-261 126-331T96-480q0-80 30-149.5t82.5-122Q261-804 331-834t149-30q80 0 149.5 30t122 82.5Q804-699 834-629.5T864-480q0 79-30 149t-82.5 122.5Q699-156 629.5-126T480-96Zm0-72q130 0 221-91t91-221q0-130-91-221t-221-91q-130 0-221 91t-91 221q0 130 91 221t221 91Zm0-312Z"/>
                                    </svg>
                                </span>
                            </Show>
                            <span class="search-row__plugin-name">
                                <a href=spigot_url.clone() target="_blank">{spigot_name.clone()}</a>
                            </span>
                            <span>"  "</span>
                            <span class="search-row__plugin-version">{search_result.spigot_version.clone()}</span>
                            <span>" by "</span>
                            <span class="search-row__plugin-author">{search_result.spigot_author.clone()}</span>
                        </div>
                        <div class="search-row__cell-description">
                            {search_result.spigot_description.clone()}
                        </div>
                    </div>
                </Show>
            </div>

            <div class="search-row__modrinth-cell">
                <Show when=move || { has_modrinth }>
                    <a href=modrinth_url.clone() target="_blank">
                        <img class="search-row__image" src=modrinth_icon_display_url.clone() title=modrinth_name.clone() loading="lazy" />
                    </a>
                    <div class="search-row__title-and-description">
                        <div class="search-row__cell-title">
                            <span class="search-row__plugin-name">
                                <a href=modrinth_url.clone() target="_blank">{modrinth_name.clone()}</a>
                            </span>
                            <span>" "</span>
                            <span class="search-row__plugin-version">{search_result.modrinth_version.clone()}</span>
                            <span>" by "</span>
                            <span class="search-row__plugin-author">{search_result.modrinth_author.clone()}</span>
                        </div>
                        <div class="search-row__cell-description">
                            {search_result.modrinth_description.clone()}
                        </div>
                    </div>
                </Show>
            </div>

            <div class="search-row__hangar-cell">
                <Show when=move || { has_hangar }>
                    <a href=hangar_url.clone() target="_blank">
                        <img class="search-row__image" src=hangar_avatar_display_url.clone() title=hangar_name.clone() loading="lazy" />
                    </a>
                    <div class="search-row__title-and-description">
                        <div class="search-row__cell-title">
                            <span class="search-row__plugin-name">
                                <a href=hangar_url.clone() target="_blank">{hangar_name.clone()}</a>
                            </span>
                            <span>" "</span>
                            <span class="search-row__plugin-version">{search_result.hangar_version.clone()}</span>
                            <span>" by "</span>
                            <span class="search-row__plugin-author">{search_result.hangar_author.clone()}</span>
                        </div>
                        <div class="search-row__cell-description">
                            {search_result.hangar_description.clone()}
                        </div>
                    </div>
                </Show>
            </div>

            <div class="search-row__source-cell">
                <div class="search-row__cell-title">
                    <a href=source_repository_url target="_blank" inner_html=source_repository_url_wbr></a>
                </div>
            </div>
        </li>
    }
}
