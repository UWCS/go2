mod api;
mod app;
mod auth;
mod redirect;

pub use api::api_routes;
pub use app::app_routes;
pub use auth::auth_routes;
pub use auth::oidc_client;
pub use redirect::do_redirect;
