use axum::{
    extract::{Path, State},
    http::{header, StatusCode},
    response::IntoResponse,
};
use std::format;

use crate::{db, AppState};

#[tracing::instrument]
pub async fn do_redirect(
    State(AppState { pool, config: _ }): State<AppState>,
    Path(source): Path<String>,
) -> impl IntoResponse {
    //try to get redirect URL from db
    match db::get_sink(&source, &pool).await {
        Ok(Some(sink)) => {
            //if no error and we got a sink, redirect to it
            tracing::debug!("Got sink for {source}, sending redirect to {sink}");

            //update metrics, redirect anyway if we can't but log the error
            if let Err(e) = db::bump_count(&source, &pool).await {
                tracing::error!("Failed to increment redirect count for {source}: {e}");
            } else {
                tracing::info!("Update usage metrics for {source}");
            }

            //response is (status code, headers, body)
            (
                StatusCode::FOUND,
                [(header::LOCATION, &sink)],
                format!("Redirecting to {}", sink).into_response(),
            )
                .into_response()
        }
        Ok(None) => {
            //DB stuff was fine but link does not exist
            tracing::warn!("Request made to non-existent go link {source}");
            (
                StatusCode::NOT_FOUND,
                format!("No go link found for {}", source).into_response(),
            )
        }
        .into_response(),
        Err(e) => {
            //DB stuff failed, server error
            tracing::error!("Failure when handling redirect for {source}: {e}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
