use serde;
use axum::{extract, routing, Router};

use tt_core::def::res::Result;
use tt_core::def::resp_json::{ ApplicationJson, PageOrNot, Payload };

use crate::service;

use crate::def::user_role_group::*;

pub fn route() -> Router {
    Router::new().nest(
        "/user_role_group",
        Router::new()
            .route("/add", routing::post(add))
            .route("/edit", routing::post(edit))
            .route("/del", routing::post(del))
            .route("/get", routing::post(get))
    )
}


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct Add {
    pub name: String,
    pub role_ids: Option<Vec<u64>>,
    pub desc: Option<String>,
}
async fn add(
    extract::Json(body): extract::Json<Add>,
) -> Result<ApplicationJson<Payload<u64>>> {
    let affected = service::user_role_group::add(&body.name, &body.role_ids, &body.desc).await?;
    Ok(ApplicationJson::payload(affected))
}


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct Edit {
    pub id: u64,
    pub name: String,
    pub role_ids: Option<Vec<u64>>,
    pub desc: Option<String>,
}
async fn edit(
    extract::Json(body): extract::Json<Edit>,
) -> Result<ApplicationJson<Payload<u64>>> {
    let affected = service::user_role_group::edit(body.id, &body.name, &body.role_ids, &body.desc).await?;
    Ok(ApplicationJson::payload(affected))
}


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct Delete {
    pub id: u64,
}
async fn del(
    extract::Json(body): extract::Json<Delete>,
) -> Result<ApplicationJson<Payload<u64>>> {
    let affected = service::user_role_group::del(body.id).await?;
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
) -> Result<ApplicationJson<Payload<PageOrNot<UserRoleGroup>>>> {
    if let Some(page_num) = body.page_num {
        let page_size = body.page_size.unwrap_or(15);
        let page = service::user_role_group::get_page(body.id, &body.search, page_num, page_size).await?;
        Ok(ApplicationJson::payload(PageOrNot::Page(page)))
    } else {
        let arr = service::user_role_group::get(body.id, &body.search).await?;
        Ok(ApplicationJson::payload(PageOrNot::Not(arr)))
    }
}

