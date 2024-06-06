use axum::{
    body,
    extract::Request,
    http,
    middleware::Next,
    response::Response
};

#[allow(unused)]
pub async fn middleware_fn(
    request: Request,
    next: Next,
) -> Response {
    let body = body::Body::new("ok".to_string());
    match *request.method() {
        http::Method::OPTIONS => {
            let resp = Response::builder()
            .status(http::StatusCode::OK)
            .body(body)
            .unwrap();

            println!("OOOOO_1");
            resp
        },
        _ => {
            println!("OOOOO_2");

            let resp = next.run(request).await;

            println!("OOOOO_3");

            resp
        }
    }
}

