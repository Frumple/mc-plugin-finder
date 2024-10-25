use crate::error_template::{AppError, ErrorTemplate};

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use serde::{Serialize, Deserialize};
use std::str::FromStr;
use time::OffsetDateTime;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebProject {
    pub id: Option<i32>,
    pub date_created: OffsetDateTime,
    pub date_updated: OffsetDateTime,
    pub spigot_id: Option<i32>,
    pub spigot_name: Option<String>,
    pub spigot_description: Option<String>,
    pub spigot_author: Option<String>,
    pub modrinth_id: Option<String>,
    pub modrinth_name: Option<String>,
    pub modrinth_description: Option<String>,
    pub modrinth_author: Option<String>,
    pub hangar_slug: Option<String>,
    pub hangar_name: Option<String>,
    pub hangar_description: Option<String>,
    pub hangar_author: Option<String>
}

#[cfg(feature = "ssr")]
impl From<mc_plugin_finder::database::common::project::CommonProject> for WebProject {
    fn from(project: mc_plugin_finder::database::common::project::CommonProject) -> Self {
        WebProject {
            id: project.id,
            date_created: project.date_created,
            date_updated: project.date_updated,
            spigot_id: project.spigot_id,
            spigot_name: project.spigot_name,
            spigot_description: project.spigot_description,
            spigot_author: project.spigot_author,
            modrinth_id: project.modrinth_id,
            modrinth_name: project.modrinth_name,
            modrinth_description: project.modrinth_description,
            modrinth_author: project.modrinth_author,
            hangar_slug: project.hangar_slug,
            hangar_name: project.hangar_name,
            hangar_description: project.hangar_description,
            hangar_author: project.hangar_author
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
    DateUpdated
}

impl FromStr for WebSearchParamsSortField {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "date_created" => Ok(Self::DateCreated),
            "date_updated" => Ok(Self::DateUpdated),
            _              => Err(())
        }
    }
}

#[cfg(feature = "ssr")]
impl From<WebSearchParamsSortField> for mc_plugin_finder::database::common::project::SearchParamsSortField {
    fn from(sort_field: WebSearchParamsSortField) -> Self {
        match sort_field {
            WebSearchParamsSortField::DateCreated => mc_plugin_finder::database::common::project::SearchParamsSortField::DateCreated,
            WebSearchParamsSortField::DateUpdated => mc_plugin_finder::database::common::project::SearchParamsSortField::DateUpdated
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

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <h1>"MC Plugin Finder"</h1>
        <SearchComponent />
    }
}

#[server(SearchAction)]
pub async fn search_action(params: WebSearchParams) -> Result<WebSearchParams, ServerFnError> {
    Ok(params)
}

#[server(SearchProjects)]
pub async fn search_projects(params: WebSearchParams) -> Result<Vec<WebProject>, ServerFnError> {
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
fn SearchComponent() -> impl IntoView {
    let action = create_server_action::<SearchAction>();

    let projects_resource = create_resource(
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
        <div class="main-page__main-container">
            <ActionForm action class="main-page__search-form">
                <input type="text" name="params[query]" class="main-page__query_field" />
                <input type="submit" value="Search" class="main-page__search_button" />

                <span class="main-page__repository-text">Repository:</span>

                <input id="spigot-checkbox" type="checkbox" name="params[spigot]" class="main-page__spigot-checkbox" value="true" checked />
                <label for="spigot-checkbox" class="main-page__spigot-label">Spigot</label>

                <input id="modrinth-checkbox" type="checkbox" name="params[modrinth]" class="main-page__modrinth-checkbox" value="true" checked />
                <label for="modrinth-checkbox" class="main-page__modrinth-label">Modrinth</label>

                <input id="hangar-checkbox" type="checkbox" name="params[hangar]" class="main-page__hangar-checkbox" value="true" checked />
                <label for="hangar-checkbox" class="main-page__hangar-label">Hangar</label>

                <span class="main-page__fields-text">Fields:</span>

                <input id="name-checkbox" type="checkbox" name="params[name]" class="main-page__name-checkbox" value="true" checked />
                <label for="name-checkbox" class="main-page__name-label">Name</label>

                <input id="description-checkbox" type="checkbox" name="params[description]" class="main-page__description-checkbox" value="true" checked />
                <label for="description-checkbox" class="main-page__description-label">Description</label>

                <input id="author-checkbox" type="checkbox" name="params[author]" class="main-page__author-checkbox" value="true" checked />
                <label for="author-checkbox" class="main-page__author-label">Author</label>

                <span class="main-page__sort-text">Sort by:</span>

                <select name="params[sort_field]" class="main-page__sort-field">
                    <option value="date_created">Newest</option>
                    <option value="date_updated" selected>Recently Updated</option>
                </select>
            </ActionForm>

            <Transition fallback=move || view! { <p>"Loading..."</p> }>
                <ErrorBoundary fallback=|errors| view!{<ErrorTemplate errors=errors/>}>
                    {move || {
                        let results = {
                            move || {
                                projects_resource.get()
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
                                                        let has_spigot = project.spigot_name.is_some();
                                                        let has_modrinth = project.modrinth_name.is_some();
                                                        let has_hangar = project.hangar_name.is_some();

                                                        view! {
                                                            <li class="main-page__search-result-list-item">
                                                                <div class="main-page__search-result-cell">
                                                                    <div class="main-page__search-result-cell-title">
                                                                        <span class="main-page__search-result-cell-name">{project.spigot_name}</span>
                                                                        <Show when=move || { has_spigot }>
                                                                          <span> by </span>
                                                                        </Show>
                                                                        <span class="main-page__search-result-cell-author">{project.spigot_author}</span>
                                                                    </div>
                                                                    <div class="main-page__search-result-cell-description">
                                                                        {project.spigot_description}
                                                                    </div>
                                                                </div>
                                                                <div class="main-page__search-result-cell">
                                                                    <div class="main-page__search-result-cell-title">
                                                                        <span class="main-page__search-result-cell-name">{project.modrinth_name}</span>
                                                                        <Show when=move || { has_modrinth }>
                                                                        <span> by </span>
                                                                        </Show>
                                                                        <span class="main-page__search-result-cell-author">{project.modrinth_author}</span>
                                                                    </div>
                                                                    <div class="main-page__search-result-cell-description">
                                                                        {project.modrinth_description}
                                                                    </div>
                                                                </div>
                                                                <div class="main-page__search-result-cell">
                                                                    <div class="main-page__search-result-cell-title">
                                                                        <span class="main-page__search-result-cell-name">{project.hangar_name}</span>
                                                                        <Show when=move || { has_hangar }>
                                                                        <span> by </span>
                                                                        </Show>
                                                                        <span class="main-page__search-result-cell-author">{project.hangar_author}</span>
                                                                    </div>
                                                                    <div class="main-page__search-result-cell-description">
                                                                        {project.hangar_description}
                                                                    </div>
                                                                </div>
                                                            </li>
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
                            <div class="main-page__search-result-container">
                                <div class="main-page__search-result-header">
                                    <span class="main-page__search-result-header-column">Spigot</span>
                                    <span class="main-page__search-result-header-column">Modrinth</span>
                                    <span class="main-page__search-result-header-column">Hangar</span>
                                </div>
                                <ul class="main-page__search-result-list">
                                    {results}
                                </ul>
                            </div>
                        }
                    }
                }
                </ErrorBoundary>
            </Transition>
        </div>
    }
}
