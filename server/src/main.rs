use app::*;
use axum::Router;
use fileserv::file_and_error_handler;
use leptos::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use axum_server::tls_rustls::RustlsConfig;
use tracing_subscriber::EnvFilter;
use std::path::PathBuf;
pub mod fileserv;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new("debug,h2=error"))
        .compact()
        .init();

    let pool = sqlx::SqlitePool::connect("sqlite::memory:").await.unwrap();
    app::business_logic::migrations::migrate(&pool).await.unwrap();

    let config = RustlsConfig::from_pem_file(
            PathBuf::from("./cert.pem"),
            PathBuf::from("./key.pem")
        )
        .await
        .unwrap();
    
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
        .layer(axum::Extension(pool))
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    // axum_server::bind_rustl is a wrapper around that
    // in real use case we'd want to also run a server that redirects http requests with https to the https server
    println!("listening on https://{}", &addr);
    axum_server::bind_rustls(addr,config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
