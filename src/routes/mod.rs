mod api;
mod auth;
mod redirect;

pub use api::api_routes;
pub use auth::auth_routes;
pub use auth::oidc_client;
pub use redirect::do_redirect;
pub mod app;

fn handle_error(e: anyhow::Error) -> (axum::http::StatusCode, &'static str)
where
{
    tracing::error!("Internal Server Error: {e:?}");
    (
        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        "Internal server error",
    )
}
