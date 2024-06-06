use axum::{
    response::IntoResponse,
    Router,
};

use crate::def::err::{code, Error};


#[allow(unused)]
pub fn router() -> Router {
    let router: Router = Router::new().fallback(resp_404);
    router
}

async fn resp_404() -> impl IntoResponse {
    let err = Error::trace(&code::NOT_FOUND, "NO", &None);
    err.into_response()
}