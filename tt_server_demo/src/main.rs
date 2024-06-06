use std::time::Duration;

use tt_auth;
use tt_core::def::res::Result;

use tt_core_database;

use axum::{routing, Router};
use tower_http::cors::{Any, CorsLayer};

mod api;
mod page;

#[tokio::main]
async fn main() -> Result<()> {

    tt_core_database::pool::mysql::setup().await?;

    tt_auth::setup().await?;

    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_origin(Any)
        .allow_headers(Any)
        .max_age(Duration::from_secs(60));

    let app: Router = tt_core::router::router()
        .route("/", routing::get(page::index::index))
        .merge(api::test::route())
        .merge(tt_auth::api::permission_menu::route())
        .merge(tt_auth::api::permission::route())
        .merge(tt_auth::api::user_role::route())
        .merge(tt_auth::api::user_role_group::route())
        .merge(tt_auth::api::user::route())

        .layer(cors);

    tt_core::http_server::serve(app).await.unwrap();

    Ok(())
}
