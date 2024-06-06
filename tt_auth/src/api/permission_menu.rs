use serde;
use axum::{extract, routing, Router};

use tt_core::def::res::Result;
use tt_core::def::resp_json::{ ApplicationJson, PageOrNot, Payload };

use crate::service;

use crate::def::permission_menu::*;

pub fn route() -> Router {
    Router::new().nest(
        "/permission_menu",
        Router::new()
            .route("/add", routing::post(add))
            .route("/del", routing::post(del))

            .route("/get", routing::post(get))
            .route("/get_tree", routing::post(get_tree))


    )
}


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct Add {
    pub code: String,
    pub name: String,
}
async fn add(
    extract::Json(body): extract::Json<Add>,
) -> Result<ApplicationJson<Payload<u64>>> {
    let affected = service::permission_menu::add(&body.code, &body.name, false).await?;
    Ok(ApplicationJson::payload(affected))
}


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct Delete {
    pub code: String,
}
async fn del(
    extract::Json(body): extract::Json<Delete>,
) -> Result<ApplicationJson<Payload<u64>>> {
    let affected = service::permission_menu::del(&body.code).await?;
    Ok(ApplicationJson::payload(affected))
}


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct Get {
    pub code: Option<String>,
    pub name: Option<String>,
    pub page_num: Option<u64>,
    pub page_size: Option<u64>,
}
async fn get(
    extract::Json(body): extract::Json<Get>,
) -> Result<ApplicationJson<Payload<PageOrNot<PermissionMenu>>>> {
    if let Some(page_num) = body.page_num {
        let page_size = body.page_size.unwrap_or(15);
        let page = service::permission_menu::get_page(&body.code, &body.name, page_num, page_size).await?;
        Ok(ApplicationJson::payload(PageOrNot::Page(page)))
    } else {
        let arr = service::permission_menu::get(&body.code, &body.name).await?;
        Ok(ApplicationJson::payload(PageOrNot::Not(arr)))
    }
}

async fn get_tree() -> Result<ApplicationJson<Payload<Vec<PermissionMenuWithPermissions>>>> {
    let arr = service::permission_menu::get_tree().await?;
    Ok(ApplicationJson::payload(arr))
}

