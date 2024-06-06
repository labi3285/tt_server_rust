use tt_core::template::HtmlTemplate;

use askama::Template;
use axum::response::IntoResponse;

use tt_core::env;

#[allow(unused)]
#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    name: String,
}
#[allow(unused)]
pub async fn index() -> impl IntoResponse {
    let name = env::str("SERVER_NAME").unwrap_or(String::from("SERVER_NAME"));
    let temp = IndexTemplate { name };
    HtmlTemplate(temp)
}



