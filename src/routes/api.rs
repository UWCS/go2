use crate::AppState;
use axum::{routing::post, Router};

async fn add_redirect() {}
async fn update_redirect() {}
async fn get_redirects() {}

pub fn api_routes() -> Router<AppState> {
    Router::new().route("/new", post(add_redirect))
}
