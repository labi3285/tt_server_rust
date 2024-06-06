use serde;
use axum::{extract, http, routing, Router};

use tt_core::def::res::Result;
use tt_core::def::resp_json::{ ApplicationJson, PageOrNot, Payload };

use crate::def::types::*;
use crate::service;

use crate::def::token::*;
use crate::def::user::*;

use crate::validate_token_user;

pub fn route() -> Router {
    Router::new().nest(
        "/user",
        Router::new()
            .route("/register_by_account", routing::post(register_by_account))
            .route("/login_by_account", routing::post(login_by_account))
            .route("/get_my_info", routing::post(get_my_info))
            .route("/get", routing::post(get))

    )
}


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct RegisterByAccount {
    pub account: String,
    pub password: String,
}
async fn register_by_account(
    extract::Json(body): extract::Json<RegisterByAccount>,
) -> Result<ApplicationJson<Payload<WrapTokenUser>>> {
    let id = service::user::register_by_account(&body.account, &body.password).await?;
    let user = service::user::find(id).await?;
    let token = service::user::generate_token(&user)?;
    Ok(ApplicationJson::payload(WrapTokenUser { user, token }))
}


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct LoginByAccount {
    pub account: String,
    pub password: String,
}
async fn login_by_account(
    extract::Json(body): extract::Json<LoginByAccount>,
) -> Result<ApplicationJson<Payload<WrapTokenUser>>> {
    let user = service::user::login_by_account(&body.account, &body.password).await?;
    let token = service::user::generate_token(&user)?;
    Ok(ApplicationJson::payload(WrapTokenUser { user, token }))
}


async fn get_my_info(
    headers: http::HeaderMap,
) -> Result<ApplicationJson<Payload<User>>> {
    let token_user = validate_token_user(&headers).await?;
    let user = service::user::find(token_user.id).await?;
    Ok(ApplicationJson::payload(user))
}




#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct Get {
    pub id: Option<u64>,
    pub role_group_id: Option<u64>,
    pub account: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub nickname: Option<String>,
    pub gender: Option<UserGender>,
    pub page_num: Option<u64>,
    pub page_size: Option<u64>,
}
async fn get(
    extract::Json(body): extract::Json<Get>,
) -> Result<ApplicationJson<Payload<PageOrNot<User>>>> {
    if let Some(page_num) = body.page_num {
        let page_size = body.page_size.unwrap_or(15);
        let page = service::user::get_page(body.id, body.role_group_id, &body.account, &body.phone, &body.email, &body.nickname, &body.gender, page_num, page_size).await?;
        Ok(ApplicationJson::payload(PageOrNot::Page(page)))
    } else {
        let arr = service::user::get(body.id, body.role_group_id, &body.account, &body.phone, &body.email, &body.nickname, &body.gender).await?;
        Ok(ApplicationJson::payload(PageOrNot::Not(arr)))
    }
}