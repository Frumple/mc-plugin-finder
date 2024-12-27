#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::EnvFilter;
    use tracing_subscriber::fmt::Layer;
    use tracing_subscriber::fmt::format::FmtSpan;
    use web::app::*;
    use web::fileserv::file_and_error_handler;

    dotenvy::dotenv().expect("could not read .env file");

    // Initialize tracing
    let appender = tracing_appender::rolling::daily("logs/web", "web.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(appender);

    let file_layer = Layer::new()
        .with_writer(non_blocking)
        .with_span_events(FmtSpan::CLOSE)
        .with_ansi(false);

    let console_layer = Layer::new()
        .with_writer(std::io::stdout)
        .with_span_events(FmtSpan::CLOSE);

    let subscriber = tracing_subscriber::registry()
        .with(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .with(file_layer)
        .with(console_layer);
    tracing::subscriber::set_global_default(subscriber).unwrap();

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    // build our application with a route
    let app = Router::new()
        .leptos_routes(&leptos_options, routes, App)
        .fallback(file_and_error_handler)
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    logging::log!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
