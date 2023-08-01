use axum::{
    extract::Extension,
    http::header,
    middleware,
    response::{IntoResponse, Redirect},
    routing::{get, get_service},
    Router,
};
use axum_flash::Key;
use axum_session::{SessionConfig, SessionLayer, SessionPgPool, SessionStore};
use axum_session_auth::*;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    ConnectOptions, PgPool,
};
use std::{iter::once, net::SocketAddr};
use tower_http::{
    catch_panic::CatchPanicLayer,
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    sensitive_headers::SetSensitiveHeadersLayer,
    services::ServeDir,
    trace::TraceLayer,
};

use axum_flash::Config;

#[macro_use]
extern crate log;

mod extensions;
mod forms;
mod inits;
mod models;
mod site;

use extensions::FullState;
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

    //let subscriber = console_subscriber::init();
    let subscriber = tracing_subscriber::fmt()
        // Use a more compact, abbreviated log format
        .compact()
        // Display source code file paths
        .with_file(true)
        // Display source code line numbers
        .with_line_number(true)
        // Display the thread ID an event was recorded on
        .with_thread_ids(true)
        // Don't display the event's target (module path)
        .with_target(false)
        // Build the subscriber
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();
    //generate these once and store them in a config file. Do this maybe Once or twice a year.
    let key = Key::generate().master().to_vec();
    //generate these once and store them in a config file. Do this maybe Once or twice a year.
    let database_key = Key::generate().master().to_vec();
    let poll = init_pool().await.unwrap();
    let session_config = SessionConfig::default()
        .with_table_name("test_table")
        .with_key(axum_session::Key::from(&key))
        .with_database_key(axum_session::Key::from(&database_key))
        .with_bloom_filter(true);
    let csrfconfig = axum_csrf::CsrfConfig::default();
    let flash_config = Config::new(Key::from(&key)).use_secure_cookies(false);
    let session_store =
        SessionStore::<SessionPgPool>::new(Some(poll.clone().into()), session_config)
            .await
            .unwrap();
    let server_state = ServerState::new().await;
    let auth_config = AuthConfig::<i64>::default().with_anonymous_user_id(Some(1));

    //todo: change this to be doen via a cli runner.
    session_store.initiate().await.unwrap();
    let system_state = SystemState::new(flash_config, csrfconfig);

    // build our application with some routes
    let app = Router::new()
        .route("/", get(root))
        .merge(main_routes())
        //.merge(user_routes())
        .fallback(site::handler_404)
        .layer(CatchPanicLayer::new())
        .layer(SetSensitiveHeadersLayer::new(once(header::AUTHORIZATION)))
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
        .layer(Extension(poll.clone()))
        .layer(middleware::from_fn(online_updater))
        .layer(Extension(server_state))
        .layer(
            AuthSessionLayer::<User, i64, SessionPgPool, PgPool>::new(Some(poll))
                .with_config(auth_config),
        )
        .layer(SessionLayer::new(session_store))
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any))
        .nest_service("/static", get_service(ServeDir::new("static")))
        .with_state(system_state);

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

pub async fn init_pool() -> anyhow::Result<sqlx::Pool<sqlx::Postgres>> {
    let mut connect_opts = PgConnectOptions::new();
    connect_opts = connect_opts.log_statements(log::LevelFilter::Debug);
    connect_opts = connect_opts.database("test");
    connect_opts = connect_opts.username("test");
    connect_opts = connect_opts.password("test");
    connect_opts = connect_opts.host("127.0.0.1");
    connect_opts = connect_opts.port(5432);

    Ok(PgPoolOptions::new()
        .max_connections(5)
        .connect_with(connect_opts)
        .await?)
}
