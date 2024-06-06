use sqlx::MySqlConnection;

use tt_core::def::res::Result;
use tt_core::def::resp_json::Page;
use tt_core::util::arr;
use tt_core_database::db::mysql;

use crate::def::types::*;
use crate::def::user::*;


pub async fn add(
    conn: &mut MySqlConnection,
    role_group_ids: &Option<Vec<u64>>,
    account: &Option<String>,
    phone: &Option<String>,
    email: &Option<String>,
    wx_open_id: &Option<String>,
    nickname: &Option<String>,
    gender: &Option<UserGender>,
    avator: &Option<String>,
    password: &Option<String>,
    is_reserved: bool
) -> Result<u64> {
    let role_group_ids_formatted: Option<String> = arr::format_ids(role_group_ids);
    let s = format!("insert into user (
        `role_group_ids_formatted`,
        `account`,
        `phone`,
        `email`,
        `wx_open_id`,
        `nickname`,
        `gender`,
        `avator`,
        `password`,
        `is_reserved`
    ) values (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)");
    let mut q = mysql::query(s.as_str());
    q = q.bind(role_group_ids_formatted);
    q = q.bind(account);
    q = q.bind(phone);
    q = q.bind(email);
    q = q.bind(wx_open_id);
    q = q.bind(nickname);
    q = q.bind(gender);
    q = q.bind(avator);
    q = q.bind(password);
    q = q.bind(is_reserved);
    let (_, last_id) = mysql::exec(&mut *conn, q).await?;
    Ok(last_id)
}

pub async fn edit_phone(conn: &mut MySqlConnection, id: u64, phone: &Option<String>) -> Result<u64> {
    let s = format!("update user set `phone` = ? where `id` = ?");
    let mut q = mysql::query(s.as_str());
    q = q.bind(phone);
    q = q.bind(id);
    let (_, last_id) = mysql::exec(&mut *conn, q).await?;
    Ok(last_id)
}

pub async fn edit_email(conn: &mut MySqlConnection, id: u64, email: &Option<String>) -> Result<u64> {
    let s = format!("update user set `email` = ? where `id` = ?");
    let mut q = mysql::query(s.as_str());
    q = q.bind(email);
    q = q.bind(id);
    let (affected, _) = mysql::exec(&mut *conn, q).await?;
    Ok(affected)
}

pub async fn edit_wx_open_id(conn: &mut MySqlConnection, id: u64, wx_open_id: &Option<String>) -> Result<u64> {
    let s = format!("update user set `wx_open_id` = ? where `id` = ?");
    let mut q = mysql::query(s.as_str());
    q = q.bind(wx_open_id);
    q = q.bind(id);
    let (affected, _) = mysql::exec(&mut *conn, q).await?;
    Ok(affected)
}

pub async fn edit_nickname(conn: &mut MySqlConnection, id: u64, nickname: &Option<String>) -> Result<u64> {
    let s = format!("update user set `nickname` = ? where `id` = ?");
    let mut q = mysql::query(s.as_str());
    q = q.bind(nickname);
    q = q.bind(id);
    let (_, last_id) = mysql::exec(&mut *conn, q).await?;
    Ok(last_id)
}

pub async fn edit_gender(conn: &mut MySqlConnection, id: u64, gender: &Option<UserGender>) -> Result<u64> {
    let s = format!("update user set `gender` = ? where `id` = ?");
    let mut q = mysql::query(s.as_str());
    q = q.bind(gender);
    q = q.bind(id);
    let (_, last_id) = mysql::exec(&mut *conn, q).await?;
    Ok(last_id)
}

pub async fn edit_avator(conn: &mut MySqlConnection, id: u64, avator: &Option<String>) -> Result<u64> {
    let s = format!("update user set `avator` = ? where `id` = ?");
    let mut q = mysql::query(s.as_str());
    q = q.bind(avator);
    q = q.bind(id);
    let (affected, _) = mysql::exec(&mut *conn, q).await?;
    Ok(affected)
}

pub async fn edit_password(conn: &mut MySqlConnection, id: u64, password: &Option<String>) -> Result<u64> {
    let s = format!("update user set `password` = ? where `id` = ?");
    let mut q = mysql::query(s.as_str());
    q = q.bind(password);
    q = q.bind(id);
    let (affected, _) = mysql::exec(&mut *conn, q).await?;
    Ok(affected)
}

pub async fn edit_status(conn: &mut MySqlConnection, id: u64, status: &Status) -> Result<u64> {
    let s = format!("update user set `status` = ? where `id` = ?");
    let mut q = mysql::query(s.as_str());
    q = q.bind(status);
    q = q.bind(id);
    let (affected, _) = mysql::exec(&mut *conn, q).await?;
    Ok(affected)
}

pub async fn edit_role_ids(conn: &mut MySqlConnection, id: u64, role_ids: &Option<Vec<u64>>) -> Result<u64> {
    let role_ids_formatted = arr::format_ids(role_ids);
    let s = format!("update user set `role_ids_formatted` = ? where `id` = ?");
    let mut q = mysql::query(s.as_str());
    q = q.bind(role_ids_formatted);
    q = q.bind(id);
    let (affected, _) = mysql::exec(&mut *conn, q).await?;
    Ok(affected)
}

pub async fn find(conn: &mut MySqlConnection, id: u64) -> Result<Option<User>> {
    let s = format!("select * from user where `id` = ?");
    let mut q = mysql::query_as::<User>(&s);
    q = q.bind(id);
    let a = mysql::exec_opt_one(&mut *conn, q).await?;
    Ok(a)
}

pub async fn find_by_account(conn: &mut MySqlConnection, account: &String) -> Result<Option<User>> {
    let s = format!("select * from user where `account` = ?");
    let mut q = mysql::query_as::<User>(&s);
    q = q.bind(account);
    let a = mysql::exec_opt_one(&mut *conn, q).await?;
    Ok(a)
}

pub async fn find_by_phone(conn: &mut MySqlConnection, phone: &String) -> Result<Option<User>> {
    let s = format!("select * from user where `phone` = ?");
    let mut q = mysql::query_as::<User>(&s);
    q = q.bind(phone);
    let a = mysql::exec_opt_one(&mut *conn, q).await?;
    Ok(a)
}

pub async fn find_by_email(conn: &mut MySqlConnection, email: &String) -> Result<Option<User>> {
    let s = format!("select * from user where `email` = ?");
    let mut q = mysql::query_as::<User>(&s);
    q = q.bind(email);
    let a = mysql::exec_opt_one(&mut *conn, q).await?;
    Ok(a)
}

pub async fn find_by_wx_open_id(conn: &mut MySqlConnection, wx_open_id: &String) -> Result<Option<User>> {
    let s = format!("select * from user where `wx_open_id` = ?");
    let mut q = mysql::query_as::<User>(&s);
    q = q.bind(wx_open_id);
    let a = mysql::exec_opt_one(&mut *conn, q).await?;
    Ok(a)
}

pub async fn get(
    conn: &mut MySqlConnection,
    id: Option<u64>,
    role_group_id: Option<u64>,
    account: &Option<String>,
    phone: &Option<String>,
    email: &Option<String>,
    nickname: &Option<String>,
    gender: &Option<UserGender>,
) -> Result<Vec<User>> {
    let mut role_group_id_formatted = Option::<String>::None;
    if let Some(v) = role_group_id {
        role_group_id_formatted = Some(format!("/{}/", v));
    }
    let s = format!("select * from user");
    let mut w = " where 1=1".to_string();
    if let Some(_) = id                         { w += " and `id` = ?" }
    if let Some(_) = &role_group_id_formatted   { w += " and locate(?, `account`)" }
    if let Some(_) = account                    { w += " and locate(?, `account`)" }
    if let Some(_) = phone                      { w += " and locate(?, `phone`)" }
    if let Some(_) = email                      { w += " and locate(?, `email`)" }
    if let Some(_) = nickname                   { w += " and locate(?, `nickname`)" }
    if let Some(_) = gender                     { w += " and locate(?, `gender`)" }
    let a = s + &w;
    let mut q = mysql::query_as::<User>(&a);
    if let Some(v) = id                             { q = q.bind(v); }
    if let Some(v) = &role_group_id_formatted   { q = q.bind(v); }
    if let Some(v) = account                    { q = q.bind(v); }
    if let Some(v) = phone                      { q = q.bind(v); }
    if let Some(v) = email                      { q = q.bind(v); }
    if let Some(v) = nickname                   { q = q.bind(v); }
    if let Some(v) = gender                 { q = q.bind(v); }
    let list = mysql::exec_arr(&mut *conn, q).await?;
    Ok(list)
}

pub async fn get_page(
    conn: &mut MySqlConnection,
    id: Option<u64>,
    role_group_id: Option<u64>,
    account: &Option<String>,
    phone: &Option<String>,
    email: &Option<String>,
    nickname: &Option<String>,
    gender: &Option<UserGender>,
    page_num: u64, page_size: u64
) -> Result<Page<User>> {
    let mut role_group_id_formatted = Option::<String>::None;
    if let Some(v) = role_group_id {
        role_group_id_formatted = Some(format!("/{}/", v));
    }
    let s = format!("select * from user");
    let mut w = " where 1=1".to_string();
    if let Some(_) = id                         { w += " and `id` = ?" }
    if let Some(_) = role_group_id_formatted    { w += " and locate(?, `account`)" }
    if let Some(_) = account                    { w += " and locate(?, `account`)" }
    if let Some(_) = phone                      { w += " and locate(?, `phone`)" }
    if let Some(_) = email                      { w += " and locate(?, `email`)" }
    if let Some(_) = nickname                   { w += " and locate(?, `nickname`)" }
    if let Some(_) = gender                     { w += " and locate(?, `gender`)" }
    let a = s + &w + " limit ? offset ?";
    let mut q = mysql::query_as::<User>(&a);
    if let Some(v) = id                             { q = q.bind(v); }
    if let Some(v) = &role_group_id_formatted   { q = q.bind(v); }
    if let Some(v) = account                    { q = q.bind(v); }
    if let Some(v) = phone                      { q = q.bind(v); }
    if let Some(v) = email                      { q = q.bind(v); }
    if let Some(v) = nickname                   { q = q.bind(v); }
    if let Some(v) = gender                 { q = q.bind(v); }
    q = q.bind(page_size).bind(page_num * page_size);
    let list = mysql::exec_arr(&mut *conn, q).await?;

    let c = format!("select count(*) from user");
    let a = c + &w;
    let mut q = mysql::query_as::<(i64,)>(&a);
    if let Some(v) = id                             { q = q.bind(v); }
    if let Some(v) = &role_group_id_formatted   { q = q.bind(v); }
    if let Some(v) = account                    { q = q.bind(v); }
    if let Some(v) = phone                      { q = q.bind(v); }
    if let Some(v) = email                      { q = q.bind(v); }
    if let Some(v) = nickname                   { q = q.bind(v); }
    if let Some(v) = gender                 { q = q.bind(v); }
    let total = mysql::exec_one(&mut *conn, q).await?.0;
    Ok(Page { page_num, page_size, total: total as u64, list })
}

pub async fn get_has_role_group(conn: &mut MySqlConnection, role_group_id: u64) -> Result<Vec<User>> {
    let s = format!("select * from user where locate(?, `role_group_ids_formatted`)");
    let id_formatted = format!("/{}/", role_group_id);
    let mut q = mysql::query_as::<User>(&s);
    q = q.bind(&id_formatted);
    let list = mysql::exec_arr(&mut *conn, q).await?;
    Ok(list)
}