use std::collections::HashMap;

use axum::{
    extract, http,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};

use axum::{routing, Router};

use tt_core::def::resp_json;
use tt_core::def::resp_text_plain;

pub fn route() -> Router {
    Router::new().nest(
        "/test",
        Router::new()
            .route("/test", routing::post(test))
            .route("/string", routing::post(string))
            .route("/bytes", routing::post(bytes))
            .route("/request", routing::post(request))
            .route("/path/:id", routing::get(path))

    )
}

#[derive(Debug, Deserialize, Serialize)]
struct Object {
    name: String,
    age: Option<u32>,
}

async fn test(
    headers: http::HeaderMap,
    extract::Query(query): extract::Query<HashMap<String, String>>,
    extract::Json(payload): extract::Json<Object>,
) -> impl IntoResponse {
    println!("headers:{:?}", headers);
    println!("query:{:?}", query);
    println!("payload:{:?}", payload);
    resp_json::ApplicationJson { payload }
}

async fn string(body: String) -> impl IntoResponse {
    resp_text_plain::TextPlain { payload: body }
}

async fn bytes(bytes: axum::body::Bytes) -> impl IntoResponse {
    let payload = format!("{:?}", bytes);
    resp_text_plain::TextPlain { payload }
}

async fn request(request: http::Request<axum::body::Body>) -> impl IntoResponse {
    let payload = format!("{:?}", request.body());
    resp_text_plain::TextPlain { payload }
}

async fn path(extract::Path(id): extract::Path<i32>,) -> impl IntoResponse {
    let payload: String = format!("{}", id);
    println!("{}", payload);
    resp_text_plain::TextPlain { payload }
}

