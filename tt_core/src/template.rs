
use askama::Template;
use axum::{
    http::StatusCode,
    response::{ Html, IntoResponse, Response },
};

#[allow(unused)]
pub struct HtmlTemplate<T>(pub T);

#[allow(unused)]
impl<T> IntoResponse for HtmlTemplate<T> where T: Template {
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {err}"),
            ).into_response(),
        }
    }
}