use axum::{http::StatusCode, routing::get, Router};
use std::sync::Arc;

mod config;
mod db;
mod routes;

/// Struct containing application state that may be needed by any handler
#[derive(Debug, Clone)]
pub struct AppState {
    pool: sqlx::PgPool,
    config: std::sync::Arc<config::Config>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let config = config::Config::get_from_env()?;
    let port = config.port;

    tracing::debug!("Connecting to database...");
    let pool = sqlx::PgPool::connect(&config.db_url).await?;
    tracing::info!("Database connected!");

    let state = AppState {
        pool,
        config: Arc::new(config),
    };

    let app = Router::new()
        .route("/:source", get(routes::do_redirect))
        .nest("/api", routes::api_routes())
        .with_state(state)
        .fallback(|| async { (StatusCode::NOT_FOUND, "Not Found") });

    tracing::info!("Starting server...");
    axum::Server::bind(&([0, 0, 0, 0], port).into())
        .serve(app.into_make_service())
        .await
        .map_err(Into::into)
}
