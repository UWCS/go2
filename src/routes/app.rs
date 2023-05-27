use axum::{
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Router,
};
use axum_sessions::extractors::ReadableSession;

use crate::AppState;

async fn app_page(session: ReadableSession) -> impl IntoResponse {
    match session.get::<String>("username") {
        Some(name) => format!("Hello, {}!", name).into_response(),
        None => Redirect::to("/auth/login").into_response(),
    }
}

async fn handle_form(session: ReadableSession) {}

pub fn app_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(app_page))
        .route("/", post(handle_form))
}
