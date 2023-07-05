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
struct AppTemplate {
    username: String,
    message: Option<String>,
}

async fn app(session: ReadableSession, _: State<AppState>) -> Result<impl IntoResponse> {
    if session.get::<String>("username").is_none() {
        return Err(Redirect::to("/auth/login").into());
    }

    let username = session.get::<String>("username").unwrap();
    Ok(AppTemplate {
        username,
        message: None,
    })
}

#[derive(Template)]
#[template(path = "panel.html")]
struct PanelTemplate {
    message: Option<String>,
}

#[derive(Template)]
#[template(path = "table.html")]
struct TableTemplate {
    redirects: Vec<crate::types::Redirect>,
}

///this returns just the html for the table body. Used for lazy loading and reloading by HTMX.
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
    //return entire panel, including message and cleared form for htmx to swap in

    Ok(PanelTemplate {
        message: Some(
            match db::add_new(&f.source, &f.sink, &username, &s.pool).await {
                Ok(_) => format!("✅ Added go link \"{}\" link to database", f.source),
                Err(sqlx::Error::Database(e)) if e.code() == Some("23505".into()) => {
                    format!("❌ Go link with source \"{}\" already exists", f.source)
                }
                Err(e) => {
                    tracing::error!("Failed to handle form submission: {e:?}");
                    format!("❌ Failed to add go link to database: {e:?}")
                }
            },
        ),
    })
}

pub fn app_routes() -> Router<AppState> {
    Router::new()
        .route("/panel", get(app).post(handle_form))
        .route("/panel/table", get(table))
}
