use axum::response::Result;
use axum::{
    http::{Request, StatusCode},
    middleware::{from_fn, Next},
    response::IntoResponse,
    routing::{get, post},
    Router,
};

async fn add_redirect() {}
async fn change_sink() {}
async fn get_redirects() {}

pub fn api_routes(tok: &str) -> Router<crate::AppState> {
    Router::new()
        .route("/new", post(add_redirect))
        .route("/update/:source", post(change_sink))
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
