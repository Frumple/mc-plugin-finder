use crate::error_template::{AppError, ErrorTemplate};

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use serde::{Serialize, Deserialize};
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
        <SearchBox />
    }
}

#[server(SearchAction)]
pub async fn search_action(query: String) -> Result<String, ServerFnError> {
    Ok(query)
}

#[server(GetProjects)]
pub async fn get_projects(query: String) -> Result<Vec<WebProject>, ServerFnError> {
    use self::ssr::*;
    use mc_plugin_finder::database::common::project::search_common_projects;

    let db_pool = db().await?;
    let common_projects = search_common_projects(&db_pool, &query).await;

    match common_projects {
        Ok(projects) => Ok(projects.into_iter().map(|x| x.into()).collect()),
        Err(error) => Err(ServerFnError::ServerError(error.to_string()))
    }
}

#[component]
fn SearchBox() -> impl IntoView {
    let action = create_server_action::<SearchAction>();

    let projects = create_resource(
        move || action.value().get(),
        move |value| {
            // TODO: Handle Option and Result properly here
            let query = value.unwrap_or_else(|| Ok(String::from(""))).expect("err");
            get_projects(query)
        }
    );

    view! {
        <ActionForm action>
            <input type="text" name="query"/>
            <input type="submit" value="Search"/>
        </ActionForm>

        <Transition fallback=move || view! { <p>"Loading..."</p> }>
            <ErrorBoundary fallback=|errors| view!{<ErrorTemplate errors=errors/>}>
                {move || {
                    let results = {
                        move || {
                            projects.get()
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
                                                        <li>
                                                            {project.spigot_name}/{project.modrinth_name}/{project.hangar_name}
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
                        <ul>
                            {results}
                        </ul>
                    }
                }
            }
            </ErrorBoundary>
        </Transition>

        // <p>You submitted: {move || format!("{:?}", action.input().get())}</p>

        // "action.value(): "
        // {move || action.value().get()}
    }
}
