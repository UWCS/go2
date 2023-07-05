use super::handle_error;
use crate::{db, types::GoPair, AppState};
use anyhow::Context;
use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Redirect, Result},
    routing::get,
    Form, Router,
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
}

async fn panel(
    session: ReadableSession,
    State(state): State<AppState>,
) -> Result<impl IntoResponse> {
    if session.get::<String>("username").is_none() {
        return Err(Redirect::to("/auth/login").into());
    }

    let username = session.get::<String>("username").unwrap();
    Ok(PanelTemplate { username })
}

#[derive(Template)]
#[template(path = "table.html")]
struct TableTemplate {
    redirects: Vec<crate::types::Redirect>,
}
async fn table(
    session: ReadableSession,
    State(state): State<AppState>,
) -> Result<impl IntoResponse> {
    if session.get::<String>("username").is_none() {
        return Err(Redirect::to("/auth/login").into());
    }

    let redirects = crate::db::get_all(&state.pool)
        .await
        .context("Could not get redirects from database")
        .map_err(handle_error)?;

    Ok(TableTemplate { redirects })
}

async fn handle_form(
    State(s): State<AppState>,
    session: ReadableSession,
    Form(f): Form<GoPair>,
) -> Result<impl IntoResponse> {
    let Some(username) = session.get::<String>("username") else { 
        return Err((
        StatusCode::UNAUTHORIZED,
        "You must be logged in to do that.",
    )
        .into())
    };

    db::add_new(&f.source, &f.sink, &username, &s.pool)
        .await
        .context("Could not add new redirect to database")
        .map_err(handle_error)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "An error occursed while adding your go link to the database.",
            )
        })?;
    Ok(Redirect::to("/app/panel"))
}

pub fn app_routes() -> Router<AppState> {
    Router::new()
        .route("/panel", get(panel).post(handle_form))
        .route("/panel/table", get(table))
}
