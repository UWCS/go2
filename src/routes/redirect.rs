use std::format;

use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::{http::header, response::IntoResponse};

use crate::{db, AppState};

#[tracing::instrument]
pub async fn do_redirect(
    State(AppState { pool, config: _ }): State<AppState>,
    Path(source): Path<String>,
) -> impl IntoResponse {
    match db::get_sink(&source, &pool).await {
        Ok(Some(sink)) => {
            tracing::debug!("Got sink for {source}, sending redirect to {sink}");

            if let Err(e) = db::bump_count(&source, &pool).await {
                tracing::error!("Failed to increment redirect count for {source}: {e}");
            }

            (
                StatusCode::FOUND,
                [(header::LOCATION, "https://google.com")],
                format!("Redirecting to {}", sink).into_response(),
            )
                .into_response()
        }
        Ok(None) => {
            tracing::warn!("Request made to non-existent go link {source}");
            (
                StatusCode::NOT_FOUND,
                format!("No go link found for {}", source).into_response(),
            )
        }
        .into_response(),
        Err(e) => {
            tracing::error!("Failure when handling redirect for {source}: {e}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
