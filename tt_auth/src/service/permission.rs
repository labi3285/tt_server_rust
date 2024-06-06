
use tt_core::def::res::Result;
use tt_core::def::err::Error;
use tt_core::def::resp_json::Page;
use tt_core_database::pool::mysql;

use crate::def::err::code;

use crate::api::permission;
use crate::db;
use crate::def::permission::*;

pub async fn add(code: &String, menu_code: &String, name: &String, is_reserved: bool) -> Result<u64> {
    let mut conn = mysql::get_connect().await?;
    let full_code = format!("{}.{}", menu_code, code);
    let id = db::permission::add(&mut conn, &full_code, menu_code, name, is_reserved).await?;
    Ok(id)
}

pub async fn del(code: &String) -> Result<u64> {
    let mut conn = mysql::get_connect().await?;
    let permission = db::permission::find(&mut conn, code).await?.ok_or(Error::new(&code::NOT_FOUND, "权限不存在"))?;
    if permission.is_reserved {
        return Err(Error::new(&code::RESERVED, "保留数据"));
    }
    let id = db::permission::del(&mut conn, code).await?;
    Ok(id)
}

pub async fn get(code: &Option<String>, menu_code: &Option<String>, name: &Option<String>) -> Result<Vec<Permission>> {
    let mut conn = mysql::get_connect().await?;
    let arr = db::permission::get(&mut conn, code, menu_code, name).await?;
    Ok(arr)
}
pub async fn get_page(code: &Option<String>, menu_code: &Option<String>, name: &Option<String>, page_num: u64, page_size: u64) -> Result<Page<Permission>> {
    let mut conn = mysql::get_connect().await?;
    let page = db::permission::get_page(&mut conn, code, menu_code, name, page_num, page_size).await?;
    Ok(page)
}

pub async fn get_with_menu_name(code: &Option<String>, menu_code: &Option<String>, name: &Option<String>) -> Result<Vec<PermissionWithMenuName>> {
    let mut conn = mysql::get_connect().await?;
    let arr = db::permission::get_with_menu_name(&mut conn, code, menu_code, name).await?;
    Ok(arr)
}
pub async fn get_page_with_menu_name(code: &Option<String>, menu_code: &Option<String>, name: &Option<String>, page_num: u64, page_size: u64) -> Result<Page<PermissionWithMenuName>> {
    let mut conn = mysql::get_connect().await?;
    let page = db::permission::get_page_with_menu_name(&mut conn, code, menu_code, name, page_num, page_size).await?;
    Ok(page)
}