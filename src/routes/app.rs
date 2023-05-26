use axum::{
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use axum_sessions::extractors::ReadableSession;

use crate::AppState;

async fn app_page(session: ReadableSession) -> impl IntoResponse {
    if session.get::<bool>("signed_in").unwrap_or(false) {
        "Shh, it's secret!"
    } else {
        "Nothing to see here."
    }
}

async fn handle_form(session: ReadableSession) {}

pub fn app_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(app_page))
        .route("/", post(handle_form))
}
