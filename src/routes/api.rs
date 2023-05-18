use std::collections::HashMap;

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

#[derive(Debug, serde::Deserialize)]
struct GoPair {
    source: String,
    sink: String,
}

#[tracing::instrument]
async fn add_redirect(
    State(AppState { pool, config: _ }): State<AppState>,
    Json(GoPair { source, sink }): Json<GoPair>,
) -> Result<StatusCode> {
    let r = db::add_new(&source, &sink, &pool).await;
    match r {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(sqlx::Error::Database(e)) if e.code() == Some("23505".into()) => {
            tracing::warn!("API request attempted to insert duplicate go link {source} -> {sink}");
            Ok(StatusCode::CONFLICT)
        }
        Err(e) => {
            tracing::error!("Failed to handle API request: {e:?}");
            Err(StatusCode::INTERNAL_SERVER_ERROR.into())
        }
    }
}

#[derive(Debug, serde::Deserialize)]
struct PaginateParams {
    limit: u32,
    offset: u32,
}

#[tracing::instrument]
async fn get_redirects(
    State(AppState { pool, config: _ }): State<AppState>,
    Query(q): Query<Option<PaginateParams>>,
) -> Result<Json<Vec<crate::Redirect>>> {
    match match q {
        Some(PaginateParams { limit, offset }) => {
            db::get_page(&pool, limit.into(), offset.into()).await
        }
        None => db::get_all(&pool).await,
    } {
        Ok(r) => Ok(Json(r)),
        Err(e) => {
            tracing::error!("Failed to handle API request: {e}");
            Err(StatusCode::INTERNAL_SERVER_ERROR.into())
        }
    }
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
