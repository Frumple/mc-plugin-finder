use crate::error_template::{AppError, ErrorTemplate};

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use serde::{Serialize, Deserialize};
use std::str::FromStr;
use time::{OffsetDateTime, format_description};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebSearchResult {
    pub id: Option<i32>,
    pub date_created: OffsetDateTime,
    pub date_updated: OffsetDateTime,
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

    pub modrinth_id: Option<String>,
    pub modrinth_slug: Option<String>,
    pub modrinth_name: Option<String>,
    pub modrinth_description: Option<String>,
    pub modrinth_author: Option<String>,
    pub modrinth_version: Option<String>,

    pub hangar_slug: Option<String>,
    pub hangar_name: Option<String>,
    pub hangar_description: Option<String>,
    pub hangar_author: Option<String>,
    pub hangar_version: Option<String>,

    pub source_repository_host: Option<String>,
    pub source_repository_owner: Option<String>,
    pub source_repository_name: Option<String>
}

impl WebSearchResult {
    fn spigot_url(&self) -> Option<String> {
        Some(format!("https://spigotmc.org/resources/{}", self.spigot_slug.clone()?))
    }

    fn modrinth_url(&self) -> Option<String> {
        Some(format!("https://modrinth.com/plugin/{}", self.modrinth_slug.clone()?))
    }

    fn hangar_url(&self) -> Option<String> {
        Some(format!("https://hangar.papermc.io/{}/{}", self.hangar_author.clone()?, self.hangar_slug.clone()?))
    }

    fn source_repository_url(&self) -> Option<String> {
        Some(format!("https://{}/{}/{}", self.source_repository_host.clone()?, self.source_repository_owner.clone()?, self.source_repository_name.clone()?))
    }

    fn source_repository_url_wbr(&self) -> Option<String> {
        Some(format!("https://{}/<wbr>{}/<wbr>{}", self.source_repository_host.clone()?, self.source_repository_owner.clone()?, self.source_repository_name.clone()?))
    }
}

#[cfg(feature = "ssr")]
impl From<mc_plugin_finder::database::common::project::CommonProjectSearchResult> for WebSearchResult {
    fn from(search_result: mc_plugin_finder::database::common::project::CommonProjectSearchResult) -> Self {
        WebSearchResult {
            id: search_result.project.id,
            date_created: search_result.date_created,
            date_updated: search_result.date_updated,
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

            modrinth_id: search_result.project.modrinth_id,
            modrinth_slug: search_result.project.modrinth_slug,
            modrinth_name: search_result.project.modrinth_name,
            modrinth_description: search_result.project.modrinth_description,
            modrinth_author: search_result.project.modrinth_author,
            modrinth_version: search_result.project.modrinth_version,

            hangar_slug: search_result.project.hangar_slug,
            hangar_name: search_result.project.hangar_name,
            hangar_description: search_result.project.hangar_description,
            hangar_author: search_result.project.hangar_author,
            hangar_version: search_result.project.hangar_version,

            source_repository_host: search_result.source_repository_host,
            source_repository_owner: search_result.source_repository_owner,
            source_repository_name: search_result.source_repository_name
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WebSearchParams {
    pub query: String,
    #[serde(default)]
    pub spigot: bool,
    #[serde(default)]
    pub modrinth: bool,
    #[serde(default)]
    pub hangar: bool,
    #[serde(default)]
    pub name: bool,
    #[serde(default)]
    pub description: bool,
    #[serde(default)]
    pub author: bool,
    pub sort_field: WebSearchParamsSortField,
}

#[cfg(feature = "ssr")]
impl From<WebSearchParams> for mc_plugin_finder::database::common::project::SearchParams {
    fn from(params: WebSearchParams) -> Self {
        mc_plugin_finder::database::common::project::SearchParams {
            query: params.query,
            spigot: params.spigot,
            modrinth: params.modrinth,
            hangar: params.hangar,
            name: params.name,
            description: params.description,
            author: params.author,
            sort_field: params.sort_field.into()
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WebSearchParamsSortField {
    DateCreated,
    #[default]
    DateUpdated,
    Downloads,
    LikesAndStars,
    FollowsAndWatchers,
}

impl FromStr for WebSearchParamsSortField {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "date_created"         => Ok(Self::DateCreated),
            "date_updated"         => Ok(Self::DateUpdated),
            "downloads"            => Ok(Self::Downloads),
            "likes_and_stars"      => Ok(Self::LikesAndStars),
            "follows_and_watchers" => Ok(Self::FollowsAndWatchers),
            _                      => Err(())
        }
    }
}

#[cfg(feature = "ssr")]
impl From<WebSearchParamsSortField> for mc_plugin_finder::database::common::project::SearchParamsSortField {
    fn from(sort_field: WebSearchParamsSortField) -> Self {
        match sort_field {
            WebSearchParamsSortField::DateCreated => mc_plugin_finder::database::common::project::SearchParamsSortField::DateCreated,
            WebSearchParamsSortField::DateUpdated => mc_plugin_finder::database::common::project::SearchParamsSortField::DateUpdated,
            WebSearchParamsSortField::Downloads => mc_plugin_finder::database::common::project::SearchParamsSortField::Downloads,
            WebSearchParamsSortField::LikesAndStars => mc_plugin_finder::database::common::project::SearchParamsSortField::LikesAndStars,
            WebSearchParamsSortField::FollowsAndWatchers => mc_plugin_finder::database::common::project::SearchParamsSortField::FollowsAndWatchers
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

    view! {
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

#[server(SearchAction)]
pub async fn search_action(params: WebSearchParams) -> Result<WebSearchParams, ServerFnError> {
    Ok(params)
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

#[component]
fn HomePage() -> impl IntoView {
    let action = create_server_action::<SearchAction>();

    let resource = create_resource(
        move || action.value().get(),
        |value| async move {
            match value {
                Some(result) => {
                    match result {
                        Ok(params) => {
                            // Return no results if the query is an empty string
                            if params.query.is_empty() {
                                Ok(vec![])

                            // Otherwise, perform the search
                            } else {
                                search_projects(params).await
                            }
                        },
                        // Pass on any server errors to the view
                        Err(err) => Err(err)
                    }
                },
                // TODO: Don't show "No projects were found" on first load.
                None => Ok(vec![])
            }
        }
    );

    view! {
        <h1>"MC Plugin Finder"</h1>

        <div class="home-page__container">
            <SearchForm action=action />
            <SearchResults resource=resource />
        </div>
    }
}

/// Provides controls for performing a search.
#[component]
fn SearchForm(
    /// The action that is run when submitting this form.
    action: Action<SearchAction, Result<WebSearchParams, ServerFnError>>
) -> impl IntoView {
    view! {
        <ActionForm action class="search-form">
            <input type="text" name="params[query]" class="search-form__query-field" />
            <input type="submit" value="Search" class="search-form__search-button" />

            <span class="search-form__repository-text">Repository:</span>

            <input id="spigot-checkbox" type="checkbox" name="params[spigot]" class="search-form__spigot-checkbox" value="true" checked />
            <label for="spigot-checkbox" class="search-form__spigot-label">Spigot</label>

            <input id="modrinth-checkbox" type="checkbox" name="params[modrinth]" class="search-form__modrinth-checkbox" value="true" checked />
            <label for="modrinth-checkbox" class="search-form__modrinth-label">Modrinth</label>

            <input id="hangar-checkbox" type="checkbox" name="params[hangar]" class="search-form__hangar-checkbox" value="true" checked />
            <label for="hangar-checkbox" class="search-form__hangar-label">Hangar</label>

            <span class="search-form__fields-text">Fields:</span>

            <input id="name-checkbox" type="checkbox" name="params[name]" class="search-form__name-checkbox" value="true" checked />
            <label for="name-checkbox" class="search-form__name-label">Name</label>

            <input id="description-checkbox" type="checkbox" name="params[description]" class="search-form__description-checkbox" value="true" />
            <label for="description-checkbox" class="search-form__description-label">Description</label>

            <input id="author-checkbox" type="checkbox" name="params[author]" class="search-form__author-checkbox" value="true" />
            <label for="author-checkbox" class="search-form__author-label">Author</label>

            <span class="search-form__sort-text">Sort by:</span>

            <select name="params[sort_field]" class="search-form__sort-field">
                <option value="date_created">Newest</option>
                <option value="date_updated">Recently Updated</option>
                <option value="downloads" selected>Downloads</option>
                <option value="likes_and_stars">Likes + Stars</option>
                <option value="follows_and_watchers">Follows + Watchers</option>
            </select>
        </ActionForm>
    }
}

/// Displays projects matching the given search.
#[component]
fn SearchResults(
    /// The resource that performs the search when the search form is submitted.
    resource: Resource<Option<Result<WebSearchParams, ServerFnError>>, Result<Vec<WebSearchResult>, ServerFnError>>
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
                                            projects
                                                .into_iter()
                                                .map(move |project| {
                                                    view! {
                                                        <SearchRow search_result=project />
                                                    }
                                                })
                                                .collect_view()
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
                                <span class="search-results__downloads-header">Downloads</span>
                                <span class="search-results__likes-and-stars-header">Likes + Stars</span>
                                <span class="search-results__follows-and-watchers-header">Follows + Watchers</span>
                                <span class="search-results__spigot-header">Spigot</span>
                                <span class="search-results__modrinth-header">Modrinth</span>
                                <span class="search-results__hangar-header">Hangar</span>
                                <span class="search-results__source-header">Source Code</span>
                            </div>
                            <ul class="search-results__list">
                                {results}
                            </ul>
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
    /// The project representing this row.
    project: WebSearchResult
) -> impl IntoView {
    let date_format = format_description::parse("[year]-[month]-[day]").unwrap();
    let time_format = format_description::parse("[hour]:[minute]:[second]").unwrap();

    let date_created = project.date_created.format(&date_format);
    let time_created = project.date_created.format(&time_format);

    let date_updated = project.date_updated.format(&date_format);
    let time_updated = project.date_updated.format(&time_format);

    let has_spigot = project.spigot_name.is_some();
    let has_modrinth = project.modrinth_name.is_some();
    let has_hangar = project.hangar_name.is_some();

    let spigot_url = project.spigot_url();
    let modrinth_url = project.modrinth_url();
    let hangar_url = project.hangar_url();
    let source_repository_url = project.source_repository_url();
    let source_repository_url_wbr = project.source_repository_url_wbr();

    let is_spigot_premium = project.spigot_premium.unwrap_or_default();

    view! {
        <li class="search-results__list-item">
            <div class="search-results__created-cell">
                <div class="search-results__date">{date_created}</div>
                <div class="search-results__time">{time_created}</div>
            </div>

            <div class="search-results__updated-cell">
                <div class="search-results__date">{date_updated}</div>
                <div class="search-results__time">{time_updated}</div>
            </div>

            <div class="search-results__downloads-cell">
                <span class="search-results__downloads">{project.downloads}</span>
            </div>

            <div class="search-results__likes-and-stars-cell">
                <span class="search-results__likes-and-stars">{project.likes_and_stars}</span>
            </div>

            <div class="search-results__follows-and-watchers-cell">
                <span class="search-results__follows-and-watchers">{project.follows_and_watchers}</span>
            </div>

            <div class="search-results__spigot-cell">
                <div class="search-results__cell-title">
                    <Show when=move || { is_spigot_premium }>
                        <span class="search-results__plugin-premium">
                            <svg xmlns="http://www.w3.org/2000/svg" height="20px" viewBox="0 -960 960 960" width="20px" fill="#000000" style="vertical-align: bottom;">
                                <path d="M446-216h67v-47q49-8 81-42t32-79q0-45-27.5-77T514-514q-61-22-80.5-37.5T414-592q0-20 17.5-33t45.5-13q28 0 49 13.5t28 36.5l59-25q-12-33-38.5-55.5T513-697v-47h-66v48q-45 10-72 38.5T348-591q0 45 30.5 76.5T475-460q45 16 65.5 34t20.5 42q0 26-21 43.5T488-323q-33 0-58.5-22T395-402l-62 26q12 42 42 71.5t71 40.5v48Zm34 120q-79 0-149-30t-122.5-82.5Q156-261 126-331T96-480q0-80 30-149.5t82.5-122Q261-804 331-834t149-30q80 0 149.5 30t122 82.5Q804-699 834-629.5T864-480q0 79-30 149t-82.5 122.5Q699-156 629.5-126T480-96Zm0-72q130 0 221-91t91-221q0-130-91-221t-221-91q-130 0-221 91t-91 221q0 130 91 221t221 91Zm0-312Z"/>
                            </svg>
                        </span>
                    </Show>
                    <span class="search-results__plugin-name">
                        <a href=spigot_url target="_blank">{project.spigot_name}</a>
                    </span>
                    <Show when=move || { has_spigot }>
                        <span>"  "</span>
                    </Show>
                    <span class="search-results__plugin-version">{project.spigot_version}</span>
                    <Show when=move || { has_spigot }>
                        <span>" by "</span>
                    </Show>
                    <span class="search-results__plugin-author">{project.spigot_author}</span>
                </div>
                <div class="search-results__cell-description">
                    {project.spigot_description}
                </div>
            </div>

            <div class="search-results__modrinth-cell">
                <div class="search-results__cell-title">
                    <span class="search-results__plugin-name">
                        <a href=modrinth_url target="_blank">{project.modrinth_name}</a>
                    </span>
                    <Show when=move || { has_modrinth }>
                        <span>" "</span>
                    </Show>
                    <span class="search-results__plugin-version">{project.modrinth_version}</span>
                    <Show when=move || { has_modrinth }>
                        <span>" by "</span>
                    </Show>
                    <span class="search-results__plugin-author">{project.modrinth_author}</span>
                </div>
                <div class="search-results__cell-description">
                    {project.modrinth_description}
                </div>
            </div>

            <div class="search-results__hangar-cell">
                <div class="search-results__cell-title">
                    <span class="search-results__plugin-name">
                        <a href=hangar_url target="_blank">{project.hangar_name}</a>
                    </span>
                    <Show when=move || { has_hangar }>
                        <span>" "</span>
                    </Show>
                    <span class="search-results__plugin-version">{project.hangar_version}</span>
                    <Show when=move || { has_hangar }>
                        <span>" by "</span>
                    </Show>
                    <span class="search-results__plugin-author">{project.hangar_author}</span>
                </div>
                <div class="search-results__cell-description">
                    {project.hangar_description}
                </div>
            </div>

            <div class="search-results__source-cell">
                <div class="search-results__cell-title">
                    <a href=source_repository_url target="_blank" inner_html=source_repository_url_wbr></a>
                </div>
            </div>
        </li>
    }
}