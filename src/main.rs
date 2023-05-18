use axum::{http::StatusCode, routing::get, Router};
use std::sync::Arc;

mod config;
mod db;
mod routes;
pub use db::Redirect;

/// Struct containing application state that may be needed by any handler
#[derive(Debug, Clone)]
pub struct AppState {
    pool: sqlx::PgPool,
    #[allow(dead_code)]
    config: std::sync::Arc<config::Config>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let config = config::Config::get_from_env()?;
    let port = config.port;

    //I've always wanted to do this
    //TODO: make this good (rewrite auth service to be an impl on a struct with this as a field)
    let api_secret: &'static str = Box::leak(config.api_secret.clone().into_boxed_str());

    tracing::debug!("Connecting to database at {}..", &config.db_url);
    let pool = sqlx::PgPool::connect(&config.db_url).await?;
    sqlx::migrate!().run(&pool).await?;
    tracing::info!("Database connected, migrations ran!");

    let state = AppState {
        pool,
        config: Arc::new(config),
    };

    let app = Router::new()
        .route("/:source", get(routes::do_redirect))
        .nest("/api", routes::api_routes(api_secret))
        .with_state(state)
        .fallback(|| async { (StatusCode::NOT_FOUND, "Not Found") })
        .layer(tower_http::catch_panic::CatchPanicLayer::new());

    tracing::info!("Starting server...");
    axum::Server::bind(&([0, 0, 0, 0], port).into())
        .serve(app.into_make_service())
        .await
        .map_err(Into::into)
}
