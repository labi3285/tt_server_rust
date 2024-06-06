use serde;
use axum::{extract, routing, Router};

use tt_core::def::res::Result;
use tt_core::def::resp_json::{ ApplicationJson, PageOrNot, Payload };

use crate::def::types::*;
use crate::service;

use crate::def::user_role::*;

pub fn route() -> Router {
    Router::new().nest(
        "/user_role",
        Router::new()
            .route("/add", routing::post(add))
            .route("/edit", routing::post(edit))
            .route("/edit_status", routing::post(edit_status))
            .route("/del", routing::post(del))
            .route("/get", routing::post(get))
            .route("/get_permission_codes", routing::post(get_permission_codes))

    )
}


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct Add {
    pub name: String,
    pub permission_codes: Option<Vec<String>>,
    pub desc: Option<String>,
}
async fn add(
    extract::Json(body): extract::Json<Add>,
) -> Result<ApplicationJson<Payload<u64>>> {
    let affected = service::user_role::add(&body.name, &body.permission_codes, &body.desc, false).await?;
    Ok(ApplicationJson::payload(affected))
}


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct Edit {
    pub id: u64,
    pub name: String,
    pub permission_codes: Option<Vec<String>>,
    pub desc: Option<String>,
}
async fn edit(
    extract::Json(body): extract::Json<Edit>,
) -> Result<ApplicationJson<Payload<u64>>> {
    let affected = service::user_role::edit(body.id,&body.name, &body.permission_codes, &body.desc).await?;
    Ok(ApplicationJson::payload(affected))
}


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct EditStatus {
    pub id: u64,
    pub status: Status,
}
async fn edit_status(
    extract::Json(body): extract::Json<EditStatus>,
) -> Result<ApplicationJson<Payload<u64>>> {
    let affected = service::user_role::edit_status(body.id, &body.status).await?;
    Ok(ApplicationJson::payload(affected))
}


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct Delete {
    pub id: u64,
}
async fn del(
    extract::Json(body): extract::Json<Delete>,
) -> Result<ApplicationJson<Payload<u64>>> {
    let affected = service::user_role::del(body.id).await?;
    Ok(ApplicationJson::payload(affected))
}


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct Get {
    pub id: Option<u64>,
    pub search: Option<String>,
    pub page_num: Option<u64>,
    pub page_size: Option<u64>,
}
async fn get(
    extract::Json(body): extract::Json<Get>,
) -> Result<ApplicationJson<Payload<PageOrNot<UserRole>>>> {
    if let Some(page_num) = body.page_num {
        let page_size = body.page_size.unwrap_or(15);
        let page = service::user_role::get_page(body.id, &body.search, page_num, page_size).await?;
        Ok(ApplicationJson::payload(PageOrNot::Page(page)))
    } else {
        let arr = service::user_role::get(body.id, &body.search).await?;
        Ok(ApplicationJson::payload(PageOrNot::Not(arr)))
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct GetPermissionCodes {
    pub role_id: u64,
}
async fn get_permission_codes(
    extract::Json(body): extract::Json<GetPermissionCodes>,
) -> Result<ApplicationJson<Payload<Vec<String>>>> {
    let arr = service::user_role::get_permission_codes(body.role_id).await?;
    Ok(ApplicationJson::payload(arr))
}