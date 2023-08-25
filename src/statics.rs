use axum::{
    http::{header, StatusCode, Uri},
    response::{IntoResponse, Response},
};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "static/"]
pub struct Asset;

pub struct StaticFile<T>(pub T);

/// from https://github.com/pyrossh/rust-embed/blob/master/examples/axum.rs
/// convert static files to axum responses
impl<T> IntoResponse for StaticFile<T>
where
    T: Into<String>,
{
    fn into_response(self) -> Response {
        let path = self.0.into();
        match Asset::get(path.as_str()) {
            Some(content) => {
                let mime = mime_guess::from_path(path).first_or_octet_stream();
                ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
            }
            None => (StatusCode::NOT_FOUND, "404 Not Found").into_response(),
        }
    }
}

// We use a wildcard matcher ("/static/*file") to match against everything
// within our defined assets directory. This is the directory on our Asset
// struct, where folder = "static/".
pub async fn static_handler(uri: Uri) -> impl IntoResponse {
    StaticFile(uri.path().trim_start_matches("/static/").to_string())
}
