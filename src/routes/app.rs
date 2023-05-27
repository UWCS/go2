use super::handle_error;
use crate::AppState;
use anyhow::Context;
use askama::Template;
use axum::{
    extract::State,
    response::{IntoResponse, Redirect, Result},
    routing::get,
    Router,
};
use axum_sessions::extractors::ReadableSession;
pub async fn home() -> impl IntoResponse {
    #[derive(Template)]
    #[template(path = "home.html")]
    struct IndexTemplate;

    IndexTemplate
}

#[derive(Template)]
#[template(path = "app.html")]
struct PanelTemplate {
    username: String,
    redirects: Vec<crate::db::Redirect>,
}

async fn panel(
    session: ReadableSession,
    State(state): State<AppState>,
) -> Result<impl IntoResponse> {
    if session.get::<String>("username").is_none() {
        return Err(Redirect::to("/auth/login").into());
    }
    let username = session.get::<String>("username").unwrap();
    let redirects = crate::db::get_all(&state.pool)
        .await
        .context("Could not get redirects from database")
        .map_err(handle_error)?;

    Ok(PanelTemplate {
        username,
        redirects,
    })
}

async fn handle_form(session: ReadableSession) {}

pub fn app_routes() -> Router<AppState> {
    Router::new().route("/panel", get(panel).post(handle_form))
}
