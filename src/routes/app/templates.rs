use askama::Template;

#[derive(Template)]
#[template(path = "app.html")]
pub struct AppTemplate {
    pub username: String,
    pub message: Option<String>,
}

#[derive(Template)]
#[template(path = "home.html")]
pub struct IndexTemplate;

#[derive(Template)]
#[template(path = "panel.html")]
pub struct PanelTemplate {
    pub message: Option<String>,
}

#[derive(Template)]
#[template(path = "table.html")]
pub struct TableTemplate {
    pub redirects: Vec<crate::types::Redirect>,
}
