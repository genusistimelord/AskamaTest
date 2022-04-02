#![feature(log_syntax)]

use axum::{
    extract::Extension,
    handler::Handler,
    http::{header, StatusCode},
    middleware,
    response::{IntoResponse, Redirect},
    routing::{get, get_service},
    Router,
};
use axum_database_sessions::{AxumSessionConfig, AxumSessionLayer, AxumSessionStore};
use axum_flash::Key;

use axum_sessions_auth::*;
//use backtrace::Backtrace;
use std::iter::once;
//use std::panic;

use std::net::SocketAddr;
use tower_http::{
    catch_panic::CatchPanicLayer, compression::CompressionLayer,
    sensitive_headers::SetSensitiveHeadersLayer, services::ServeDir, trace::TraceLayer,
};

#[macro_use]
extern crate log;

mod extensions;
mod forms;
mod inits;
mod models;
mod site;

use forms::*;
use inits::*;
use site::*;

//use models::*;

#[tokio::main]
async fn main() {
    // Set the RUST_LOG, if it hasn't been explicitly defined
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "example_templates=debug,tower_http=debug")
    }
    tracing_subscriber::fmt::init();

    let key = Key::generate();
    let session_config = AxumSessionConfig::default().with_table_name("test_table");
    let csrfconfig = axum_csrf::CsrfConfig::default();
    let session_store = AxumSessionStore::new(None, session_config);
    let state = ServerState::new().await;

    //todo: change this to be doen via a cli runner.
    session_store.migrate().await.unwrap();

    // build our application with some routes
    let app = Router::new()
        .route("/", get(root))
        .merge(main_routes())
        //.merge(user_routes())
        .nest(
            "/static",
            get_service(ServeDir::new("static")).handle_error(|error: std::io::Error| async move {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled internal error: {}", error),
                )
            }),
        )
        .fallback(site::handler_404.into_service())
        .layer(CatchPanicLayer::new())
        .layer(SetSensitiveHeadersLayer::new(once(header::AUTHORIZATION)))
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
        .layer(middleware::from_fn(online_updater))
        .layer(Extension(state))
        .layer(AuthSessionLayer::<User>::new(None, Some(1)))
        .layer(AxumSessionLayer::new(session_store))
        .layer(axum_csrf::CsrfLayer::new(csrfconfig))
        .layer(axum_flash::layer(key).with_cookie_manager())
        .layer(tower_cookies::CookieManagerLayer::new());

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> impl IntoResponse {
    Redirect::to("main")
}
