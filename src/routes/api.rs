use std::todo;

use crate::AppState;

use axum::{
    extract::{Query, State},
    http::{Request, StatusCode},
    middleware::{from_fn, Next},
    response::{IntoResponse, Result},
    routing::{get, post},
    Json, Router,
};

use crate::db;

#[tracing::instrument]
pub async fn add_redirect(
    State(AppState { pool, config: _ }): State<AppState>,
) -> impl IntoResponse {
    todo!()
}

pub async fn get_redirects(
    State(AppState { pool, config: _ }): State<AppState>,
    Query(q): Query<Option<usize>>,
) -> Result<Json<Vec<crate::Redirect>>> {
    match q {
        Some(n) => db::get_recent(&pool, n as i64).await,
        None => db::get_all(&pool).await,
    }
    .map(axum::Json)
    .map_err(|e| {
        tracing::error!("Failed to handle API request: {e}");
        StatusCode::INTERNAL_SERVER_ERROR.into()
    })
}

pub fn api_routes(tok: &'static str) -> Router<AppState> {
    Router::new()
        .route("/new", post(add_redirect))
        .route("/list", get(get_redirects))
        .layer(from_fn(|r, n| auth(tok, r, n)))
}

pub async fn auth<B>(tok: &str, req: Request<B>, next: Next<B>) -> Result<impl IntoResponse> {
    let provided_tok = req
        .headers()
        .get("Authorization")
        .ok_or("No authorization token provided")
        .and_then(|h| h.to_str().map_err(|_| "Invalid header"))
        .and_then(|h| h.strip_prefix("Bearer: ").ok_or("No bearer token"))?;

    //something about async closures means i can't make this one obnoxiously large expression
    if provided_tok == tok {
        Ok(next.run(req).await)
    } else {
        Err((StatusCode::UNAUTHORIZED, "Invalid auth token").into())
    }
}
