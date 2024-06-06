use axum::http;
use tt_core::def::res::Result;
use tt_core::def::req;

pub mod def;
pub mod util;

pub mod db;

pub mod api;
pub mod service;

use def::token::TokenUser;


pub async fn setup() -> Result<()> {
    if service::setup::check_or_add_admin().await? {
        println!("- create admin user")
    }
    Ok(())
}

pub async fn validate_token_user(headers: &http::HeaderMap) -> Result<TokenUser> {
    let token = req::get_http_header_val(headers, "tt-token")?;
    let user = service::user::validate_token_user_from_token(&token)?;
    Ok(user)
}

