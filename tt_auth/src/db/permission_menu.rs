use std::sync::Arc;

use sqlx::MySqlConnection;

use tt_core::def::res::Result;
use tt_core::def::resp_json::Page;
use tt_core_database::db::mysql;

use crate::def::permission_menu::*;


pub async fn add(conn: &mut MySqlConnection, code: &String, name: &String, is_reserved: bool) -> Result<u64> {
    let s = format!("insert into permission_menu (`code`, `name`, `is_reserved`) values (?, ?, ?)");
    let mut q = mysql::query(&s);
    q = q.bind(code);
    q = q.bind(name);
    q = q.bind(is_reserved);
    let (_, last_id) = mysql::exec(&mut *conn, q).await?;
    Ok(last_id)
}

pub async fn find(conn: &mut MySqlConnection, code: &String) -> Result<Option<PermissionMenu>> {
    let s = format!("select * from permission_menu where `code` = ?");
    let mut q = mysql::query_as::<PermissionMenu>(&s);
    q = q.bind(code);
    let a = mysql::exec_opt_one(&mut *conn, q).await?;
    Ok(a)
}

pub async fn del(conn: &mut MySqlConnection, code: &String) -> Result<u64> {
    let s = format!("delete from permission_menu where code = ?");
    let mut q = mysql::query(s.as_str());
    q = q.bind(code);
    let (affected, _) = mysql::exec(&mut *conn, q).await?;
    Ok(affected)
}

pub async fn get(conn: &mut MySqlConnection, code: &Option<String>, name: &Option<String>) -> Result<Vec<PermissionMenu>> {
    let s = format!("select * from permission_menu");
    let mut w = " where 1=1".to_string();
    if let Some(_) = code {
        w += " and code = ?"
    }
    if let Some(_) = name {
        w += " and locate(?, name)"
    }
    let a: String = s + &w;
    let mut q = mysql::query_as::<PermissionMenu>(&a);
    if let Some(v) = code {
        q = q.bind(v);
    }
    if let Some(v) = name {
        q = q.bind(v);
    }
    let list = mysql::exec_arr(&mut *conn, q).await?;
    Ok(list)
}
pub async fn get_page(conn: &mut MySqlConnection, code: &Option<String>, name: &Option<String>, page_num: u64, page_size: u64) -> Result<Page<PermissionMenu>> {
    let s = format!("select * from permission_menu");
    let mut w = " where 1=1".to_string();
    if let Some(_) = code      { w += " and code = ?" }
    if let Some(_) = name      { w += " and locate(?, name)" }
    let a: String = s + &w + " limit ? offset ?";
    let mut q = mysql::query_as::<PermissionMenu>(&a);
    if let Some(v) = code      { q = q.bind(v); }
    if let Some(v) = name      { q = q.bind(v); }
    q = q.bind(page_size).bind(page_num * page_size);
    let list = mysql::exec_arr(&mut *conn, q).await?;

    let s = format!("select count(*) from permission_menu");
    let a: String = s + &w;
    let mut q = mysql::query_as::<(i64,)>(&a);
    if let Some(v) = code      { q = q.bind(v); }
    if let Some(v) = name      { q = q.bind(v); }
    let total = mysql::exec_one(&mut *conn, q).await?.0;

    Ok(Page { page_num, page_size, total: total as u64, list })
}
