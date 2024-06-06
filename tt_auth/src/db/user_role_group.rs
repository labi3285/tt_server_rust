use sqlx::MySqlConnection;

use tt_core::def::res::Result;
use tt_core::def::resp_json::Page;
use tt_core::util::arr;
use tt_core_database::db::mysql;

use crate::def::types::*;
use crate::def::user_role_group::*;


pub async fn add(conn: &mut MySqlConnection, name: &String, role_ids: &Option<Vec<u64>>, desc: &Option<String>, is_reserved: bool) -> Result<u64> {
    let role_ids_formatted = arr::format_ids(role_ids);
    let s = format!("insert into user_role_group (`name`, `role_ids_formatted`, `desc`, `is_reserved`) values (?, ?, ?, ?)");
    let mut q = mysql::query(s.as_str());
    q = q.bind(name);
    q = q.bind(role_ids_formatted);
    q = q.bind(desc);
    q = q.bind(is_reserved);
    let (_, last_id) = mysql::exec(&mut *conn, q).await?;
    Ok(last_id)
}

pub async fn find(conn: &mut MySqlConnection, id: u64) -> Result<Option<UserRoleGroup>> {
    let s = format!("select * from user_role_group where `id` = ?");
    let mut q = mysql::query_as::<UserRoleGroup>(&s);
    q = q.bind(id);
    let a = mysql::exec_opt_one(&mut *conn, q).await?;
    Ok(a)
}

pub async fn edit(conn: &mut MySqlConnection, id: u64, name: &String, role_ids: &Option<Vec<u64>>, desc: &Option<String>) -> Result<u64> {
    let role_ids_formatted = arr::format_ids(role_ids);
    let s = format!("update user_role_group set `name` = ?, `role_ids_formatted` = ?, `desc` = ?, `update_time` = current_timestamp where `id` = ?");
    let mut q = mysql::query(s.as_str());
    q = q.bind(name);
    q = q.bind(role_ids_formatted);
    q = q.bind(desc);
    q = q.bind(id);
    let (_, last_id) = mysql::exec(&mut *conn, q).await?;
    Ok(last_id)
}

pub async fn edit_status(conn: &mut MySqlConnection, id: u64, status: &Status) -> Result<u64> {
    let s = format!("update user_role_group set `status` = ? where `id` = ?");
    let mut q = mysql::query(s.as_str());
    q = q.bind(status);
    q = q.bind(id);
    let (affected, _) = mysql::exec(&mut *conn, q).await?;
    Ok(affected)
}

pub async fn edit_role_ids(conn: &mut MySqlConnection, id: u64, role_ids: &Option<Vec<u64>>) -> Result<u64> {
    let role_ids_formatted = arr::format_ids(role_ids);
    let s = format!("update user_role_group set `role_ids_formatted` = ? where `id` = ?");
    let mut q = mysql::query(s.as_str());
    q = q.bind(role_ids_formatted);
    q = q.bind(id);
    let (affected, _) = mysql::exec(&mut *conn, q).await?;
    Ok(affected)
}

pub async fn del(conn: &mut MySqlConnection, id: u64) -> Result<u64> {
    let s = format!("delete from user_role_group where `id` = ?");
    let mut q = mysql::query(s.as_str());
    q = q.bind(id);
    let (affected, _) = mysql::exec(&mut *conn, q).await?;
    Ok(affected)
}

pub async fn get(conn: &mut MySqlConnection, id: Option<u64>, search: &Option<String>) -> Result<Vec<UserRoleGroup>> {
    let s = format!("select * from user_role_group");
    let mut w = " where 1=1".to_string();
    if let Some(_) = id             { w += " and `id` = ?" }
    if let Some(_) = search         { w += " and (locate(?, `name`) or locate(?, `desc`))" }
    let a = s + &w;
    let mut q = mysql::query_as::<UserRoleGroup>(&a);
    if let Some(v) = id             { q = q.bind(v); }
    if let Some(v) = search     { q = q.bind(v); q = q.bind(v); }
    let list = mysql::exec_arr(&mut *conn, q).await?;
    Ok(list)
}

pub async fn get_page(conn: &mut MySqlConnection, id: Option<u64>, search: &Option<String>, page_num: u64, page_size: u64) -> Result<Page<UserRoleGroup>> {
    let s = format!("select * from user_role_group");
    let mut w = " where 1=1".to_string();
    if let Some(_) = id             { w += " and `id` = ?" }
    if let Some(_) = search         { w += " and (locate(?, `name`) or locate(?, `desc`))" }
    let a = s + &w + " limit ? offset ?";
    let mut q = mysql::query_as::<UserRoleGroup>(&a);
    if let Some(v) = id             { q = q.bind(v); }
    if let Some(v) = search     { q = q.bind(v); q = q.bind(v); }
    q = q.bind(page_size).bind(page_num * page_size);
    let list = mysql::exec_arr(&mut *conn, q).await?;

    let c = format!("select count(*) from user_role_group");
    let a = c + &w;
    let mut q = mysql::query_as::<(i64,)>(&a);
    if let Some(v) = id            { q = q.bind(v); }
    if let Some(v) = search     { q = q.bind(v); q = q.bind(v); }
    let total = mysql::exec_one(&mut *conn, q).await?.0;
    Ok(Page { page_num, page_size, total: total as u64, list })
}

pub async fn get_has_role(conn: &mut MySqlConnection, role_id: u64) -> Result<Vec<UserRoleGroup>> {
    let s = format!("select * from user_role_group where locate(?, `role_ids_formatted`)");
    let id_formatted = format!("/{}/", role_id);
    let mut q = mysql::query_as::<UserRoleGroup>(&s);
    q = q.bind(&id_formatted);
    let list = mysql::exec_arr(&mut *conn, q).await?;
    Ok(list)
}
