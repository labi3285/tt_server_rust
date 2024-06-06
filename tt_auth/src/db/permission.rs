use sqlx::MySqlConnection;

use tt_core::def::res::Result;
use tt_core::def::resp_json::Page;
use tt_core_database::db::mysql;

use crate::def::permission::*;


pub async fn add(conn: &mut MySqlConnection, code: &String, menu_code: &String, name: &String, is_reserved: bool) -> Result<u64> {
    let s = format!("insert into permission (`code`, `menu_code`, `name`, `is_reserved`) values (?, ?, ?, ?)");
    let mut q = mysql::query(s.as_str());
    q = q.bind(code);
    q = q.bind(menu_code);
    q = q.bind(name);
    q = q.bind(is_reserved);
    let (_, last_id) = mysql::exec(&mut *conn, q).await?;
    Ok(last_id)
}

pub async fn find(conn: &mut MySqlConnection, code: &String) -> Result<Option<Permission>> {
    let s = format!("select * from permission where `code` = ?");
    let mut q = mysql::query_as::<Permission>(&s);
    q = q.bind(code);
    let a = mysql::exec_opt_one(&mut *conn, q).await?;
    Ok(a)
}

pub async fn del(conn: &mut MySqlConnection, code: &String) -> Result<u64> {
    let s = format!("delete from permission where code = ?");
    let mut q = mysql::query(s.as_str());
    q = q.bind(code);
    let (affected, _) = mysql::exec(&mut *conn, q).await?;
    Ok(affected)
}
pub async fn del_by_menu_code(conn: &mut MySqlConnection, menu_code: &String) -> Result<u64> {
    let s = format!("delete from permission where menu_code = ?");
    let mut q = mysql::query(s.as_str());
    q = q.bind(menu_code);
    let (affected, _) = mysql::exec(&mut *conn, q).await?;
    Ok(affected)
}

pub async fn get(conn: &mut MySqlConnection, code: &Option<String>, menu_code: &Option<String>, name: &Option<String>) -> Result<Vec<Permission>> {
    let s = format!("select * from permission");
    let mut w = " where 1=1".to_string();
    if let Some(_) = code          { w += " and code = ?" }
    if let Some(_) = menu_code     { w += " and menu_code = ?" }
    if let Some(_) = name          { w += " and locate(?, name)" }
    let a = s + &w;
    let mut q = mysql::query_as::<Permission>(&a);
    if let Some(v) = code          { q = q.bind(v); }
    if let Some(v) = menu_code     { q = q.bind(v); }
    if let Some(v) = name           { q = q.bind(v); }
    let list = mysql::exec_arr(&mut *conn, q).await?;
    Ok(list)
}
pub async fn get_page(conn: &mut MySqlConnection, code: &Option<String>, menu_code: &Option<String>, name: &Option<String>, page_num: u64, page_size: u64) -> Result<Page<Permission>> {
    let s = format!("select * from permission");
    let mut w = " where 1=1".to_string();
    if let Some(_) = code          { w += " and code = ?" }
    if let Some(_) = menu_code     { w += " and menu_code = ?" }
    if let Some(_) = name          { w += " and locate(?, name)" }
    let a = s + &w + " limit ? offset ?";
    let mut q = mysql::query_as::<Permission>(&a);
    if let Some(v) = code          { q = q.bind(v); }
    if let Some(v) = menu_code     { q = q.bind(v); }
    if let Some(v) = name          { q = q.bind(v); }
    q = q.bind(page_size).bind(page_num * page_size);
    let list = mysql::exec_arr(&mut *conn, q).await?;

    let c = format!("select count(*) from permission");
    let a = c + &w;
    let mut q = mysql::query_as::<(i64,)>(&a);
    if let Some(v) = code          { q = q.bind(v); }
    if let Some(v) = menu_code     { q = q.bind(v); }
    if let Some(v) = name          { q = q.bind(v); }
    let total = mysql::exec_one(&mut *conn, q).await?.0;
    Ok(Page { page_num, page_size, total: total as u64, list })
}


pub async fn get_with_menu_name(conn: &mut MySqlConnection, code: &Option<String>, menu_code: &Option<String>, name: &Option<String>) -> Result<Vec<PermissionWithMenuName>> {
    let s = format!("select p.*, m.name as menu_name from permission p left join permission_menu m on p.menu_code = m.code");
    let mut w = " where 1=1".to_string();
    if let Some(_) = code          { w += " and p.code = ?" }
    if let Some(_) = menu_code     { w += " and p.menu_code = ?" }
    if let Some(_) = name          { w += " and locate(?, p.menu_code)" }

    let a = s + &w;
    let mut q = mysql::query_as::<PermissionWithMenuName>(&a);
    if let Some(v) = code          { q = q.bind(v); }
    if let Some(v) = menu_code     { q = q.bind(v); }
    if let Some(v) = name          { q = q.bind(v); }
    let list = mysql::exec_arr(&mut *conn, q).await?;
    Ok(list)
}
pub async fn get_page_with_menu_name(conn: &mut MySqlConnection, code: &Option<String>, menu_code: &Option<String>, name: &Option<String>, page_num: u64, page_size: u64) -> Result<Page<PermissionWithMenuName>> {
    let s = format!("select p.*, m.name as menu_name from permission p left join permission_menu m on p.menu_code = m.code");
    let mut w = " where 1=1".to_string();
    if let Some(_) = code          { w += " and p.code = ?" }
    if let Some(_) = menu_code     { w += " and p.menu_code = ?" }
    if let Some(_) = name          { w += " and locate(?, p.menu_code)" }

    let a: String = s + &w + " limit ? offset ?";
    let mut q = mysql::query_as::<PermissionWithMenuName>(&a);
    if let Some(v) = code          { q = q.bind(v); }
    if let Some(v) = menu_code     { q = q.bind(v); }
    if let Some(v) = name          { q = q.bind(v); }
    q = q.bind(page_size).bind(page_num * page_size);
    let list = mysql::exec_arr(&mut *conn, q).await?;

    let s = format!("select count(*) from permission p");
    let a = s + &w;
    let mut q = mysql::query_as::<(i64,)>(&a);
    if let Some(v) = code          { q = q.bind(v); }
    if let Some(v) = menu_code     { q = q.bind(v); }
    if let Some(v) = name          { q = q.bind(v); }
    let total = mysql::exec_one(&mut *conn, q).await?.0;
    Ok(Page { page_num, page_size, total: total as u64, list })
}

pub async fn batch_get_by_menu_codes(conn: &mut MySqlConnection, menu_codes: &Vec<String>) -> Result<Vec<Permission>> {
    let ps = menu_codes.into_iter().map(|_| "?").collect::<Vec<&str>>().join(",");
    let s = format!("select * from permission where `menu_code` in ({})", ps).to_string();
    let mut q = mysql::query_as::<Permission>(&s);
    for v in menu_codes {
        q = q.bind(v);
    }
    let list = mysql::exec_arr(&mut *conn, q).await?;
    println!("{:?}", list);
    Ok(list)
}