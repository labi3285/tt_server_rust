
use tt_core::def::res::Result;
use tt_core::def::err::Error;

use tt_core::def::resp_json::Page;
use tt_core_database::pool::mysql;

use crate::def::err::code;
use crate::db;

use crate::def::user_role_group::*;


pub async fn check_or_add_admin() -> Result<bool> {
    let mut conn = mysql::get_connect().await?;
    let group_id: u64 = 1;
    let group = db::user_role_group::find(&mut conn, group_id).await?;
    match group {
        Some(_) => Ok(false),
        None => {
            let menu_code = "admin".to_string();
            let permission_code = "admin.admin".to_string();
            db::permission::add(&mut conn, &permission_code, &menu_code, &"超级管理员".to_string(), true).await?;
            db::permission_menu::add(&mut conn, &menu_code, &"超级管理员".to_string(), true).await?;
            let id = db::user_role::add(&mut conn, &"超级管理员".to_string(), &Some(vec![permission_code]), &None, true).await?;
            db::user_role_group::add(&mut conn, &"超级管理员".to_string(), &Some(vec![id]), &None, true).await?;
            Ok(true)
        }
    }
}

pub async fn add(name: &String, role_ids: &Option<Vec<u64>>, desc: &Option<String>) -> Result<u64> {
    let mut conn = mysql::get_connect().await?;
    let id = db::user_role_group::add(&mut conn, name, role_ids, desc, false).await?;
    Ok(id)
}

pub async fn edit(id: u64, name: &String, role_ids: &Option<Vec<u64>>, desc: &Option<String>) -> Result<u64> {
    let mut conn = mysql::get_connect().await?;
    let id = db::user_role_group::edit(&mut conn, id, name, role_ids, desc).await?;
    Ok(id)
}

pub async fn del(id: u64) -> Result<u64> {
    let mut conn = mysql::get_connect().await?;
    let group = db::user_role_group::find(&mut conn, id).await?.ok_or(Error::new(&code::NOT_FOUND, "角色组不存在"))?;
    if group.is_reserved {
        return Err(Error::new(&code::RESERVED, "保留数据"));
    }
    let id = db::user_role_group::del(&mut conn, id).await?;
    Ok(id)
}

pub async fn get(id: Option<u64>, search: &Option<String>) -> Result<Vec<UserRoleGroup>> {
    let mut conn = mysql::get_connect().await?;
    let arr = db::user_role_group::get(&mut conn, id, search).await?;
    Ok(arr)
}
pub async fn get_page(id: Option<u64>, search: &Option<String>, page_num: u64, page_size: u64) -> Result<Page<UserRoleGroup>> {
    let mut conn = mysql::get_connect().await?;
    let page = db::user_role_group::get_page(&mut conn, id, search, page_num, page_size).await?;
    Ok(page)
}

