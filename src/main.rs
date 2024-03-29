use axum::{http::StatusCode, routing::get, Router};
use axum_sessions::{async_session::MemoryStore, SameSite, SessionLayer};
use rand::Rng;

mod config;
mod db;
mod routes;
mod statics;
mod types;

/// Struct containing application state that may be needed by any handler
#[derive(Debug, Clone)]
pub struct AppState {
    pool: sqlx::PgPool,
    oidc_client: Option<openidconnect::core::CoreClient>,
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
        oidc_client: {
            if let Some(auth_config) = config.auth_config {
                Some(routes::oidc_client(auth_config).await?)
            } else {
                None
            }
        },
    };

    //create session store and layer
    //memory store is fine because persisting sessions is not important for this use
    let mut secret = [0u8; 128];
    rand::thread_rng().fill(&mut secret[..]);
    let session_layer =
        SessionLayer::new(MemoryStore::new(), &secret).with_same_site_policy(SameSite::Lax);

    let app = Router::new()
        .route("/:source", get(routes::do_redirect))
        .route("/", get(routes::app::home))
        .nest("/api", routes::api_routes(api_secret))
        .nest("/auth", routes::auth_routes())
        .nest("/app", routes::app::app_routes())
        .route("/static/*file", get(statics::static_handler))
        .with_state(state)
        .fallback(|| async { (StatusCode::NOT_FOUND, "Not Found") })
        .layer(session_layer)
        .layer(tower_http::catch_panic::CatchPanicLayer::new());

    tracing::info!("Starting server...");
    axum::Server::bind(&([0, 0, 0, 0], port).into())
        .serve(app.into_make_service())
        .await
        .map_err(Into::into)
}
