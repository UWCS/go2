use std::collections::HashMap;

use crate::AppState;

use axum::{
    extract::{Query, State},
    headers::{authorization::Bearer, Authorization},
    http::{Request, StatusCode},
    middleware::{from_fn, Next},
    response::{IntoResponse, Result},
    routing::{get, post},
    Json, RequestPartsExt, Router, TypedHeader,
};

use crate::db;

#[derive(Debug, serde::Deserialize)]
struct GoPair {
    source: String,
    sink: String,
}

#[tracing::instrument]
async fn add_redirect(
    State(state): State<AppState>,
    Json(GoPair { source, sink }): Json<GoPair>,
) -> Result<StatusCode> {
    let r = db::add_new(&source, &sink, &state.pool).await;
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

#[tracing::instrument]
async fn get_redirects(
    State(state): State<AppState>,
    Query(q): Query<HashMap<String, u32>>,
) -> Result<Json<Vec<crate::Redirect>>> {
    tracing::info!("{q:?}");
    match match (q.get("limit"), q.get("offset")) {
        (Some(limit), Some(offset)) => {
            db::get_page(&state.pool, i64::from(*limit), i64::from(*offset)).await
        }
        (None, None) if q.is_empty() => db::get_all(&state.pool).await,
        _ => return Err(StatusCode::BAD_REQUEST.into()),
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
    let (mut parts, body) = req.into_parts();

    let auth: TypedHeader<Authorization<Bearer>> = parts
        .extract()
        .await
        .map_err(|_| (StatusCode::UNAUTHORIZED, "No auth token provided"))?;

    if auth.token() == tok {
        // reconstruct the request
        let req = Request::from_parts(parts, body);
        Ok(next.run(req).await)
    } else {
        Err((StatusCode::UNAUTHORIZED, "Invalid auth token").into())
    }
}
