
use std::future::Future;

use sqlx::{MySqlConnection, Transaction};
use tt_core::util::arr;
use tt_core::def::err::{ Error, Code };
use tt_core::def::res::Result;
use tt_core::def::resp_json::Page;
use tt_core_database::pool::mysql;

use crate::def::err::code;
use crate::db;

use crate::def::types::*;
use crate::def::user_role::*;


pub async fn add(name: &String, permission_or_menu_codes: &Option<Vec<String>>, desc: &Option<String>, is_reserved: bool) -> Result<u64> {
    let mut conn = mysql::get_connect().await?;
    if let Some(permission_or_menu_codes) = permission_or_menu_codes {
        let mut tx = mysql::get_transaction(&mut conn).await?;
        match async {
            let permission_codes = _get_permission_codes_by_codes(&mut tx, permission_or_menu_codes).await?;
            let id = db::user_role::add(&mut tx, name, &Some(permission_codes), desc, is_reserved).await?;
            Ok(id)
        }.await {
            Ok(v) => {
                mysql::commit(tx).await?;
                Ok(v)
            },
            Err(err) => {
                mysql::rollback(tx).await?;
                Err(err)
            }
        }
    } else {
        let id = db::user_role::add(&mut conn, name, &None, desc, is_reserved).await?;
        Ok(id)
    }
}

pub async fn edit(id: u64, name: &String, permission_or_menu_codes: &Option<Vec<String>>, desc: &Option<String>) -> Result<u64> {
    let mut conn = mysql::get_connect().await?;
    if let Some(permission_or_menu_codes) = permission_or_menu_codes {
        let mut tx = mysql::get_transaction(&mut conn).await?;
        match async {
            let permission_codes = _get_permission_codes_by_codes(&mut tx, permission_or_menu_codes).await?;
            let affected = db::user_role::edit(&mut tx, id, name, &Some(permission_codes), desc).await?;
            Ok(affected)
        }.await {
            Ok(v) => {
                mysql::commit(tx).await?;
                Ok(v)
            },
            Err(err) => {
                mysql::rollback(tx).await?;
                Err(err)
            }
        }
    } else {
        let affected = db::user_role::edit(&mut conn, id, name, permission_or_menu_codes, desc).await?;
        Ok(affected)
    }
}

pub async fn edit_status(id: u64, status: &Status) -> Result<u64> {
    let mut conn = mysql::get_connect().await?;
    let mut tx = mysql::get_transaction(&mut conn).await?;
    match async {
        let affected = db::user_role::edit_status(&mut tx, id, status).await?;
        let role = db::user_role::find(&mut tx, id).await?;
        if let Some(role) = role {
            match status {
                Status::Forbidden => {
                    let groups = db::user_role_group::get_has_role(&mut tx, role.id).await?;
                    for group in groups {
                        let role_ids = arr::check_or_remove_id(&group.role_ids_formatted, id);
                        db::user_role_group::edit_role_ids(&mut tx, group.id, &role_ids).await?;
                    }
                },
                _ => { }
            }
            Ok(affected)
        } else {
            Err(Error::new(&code::NOT_FOUND, "角色不存在"))
        }
    }.await {
        Ok(v) => {
            mysql::commit(tx).await?;
            Ok(v)
        },
        Err(err) => {
            mysql::rollback(tx).await?;
            Err(err)
        }
    }
}


pub async fn del(id: u64) -> Result<u64> {
    let mut conn = mysql::get_connect().await?;
    let role = db::user_role::find(&mut conn, id).await?.ok_or(Error::new(&code::NOT_FOUND, "角色不存在"))?;
    if role.is_reserved {
        return Err(Error::new(&code::RESERVED, "保留数据"));
    }
    let mut tx = mysql::get_transaction(&mut conn).await?;
    match async {
        let id = db::user_role::del(&mut tx, id).await?;
        let groups = db::user_role_group::get_has_role(&mut tx, id).await?;
        for group in groups {
            let role_ids = arr::check_or_remove_id(&group.role_ids_formatted, id);
            db::user_role_group::edit_role_ids(&mut tx, group.id, &role_ids).await?;
        }
        Ok(id)
    }.await {
        Ok(v) => {
            mysql::commit(tx).await?;
            Ok(v)
        },
        Err(err) => {
            mysql::rollback(tx).await?;
            Err(err)
        }
    }
}

pub async fn get(id: Option<u64>, search: &Option<String>) -> Result<Vec<UserRole>> {
    let mut conn = mysql::get_connect().await?;
    let arr = db::user_role::get(&mut conn, id, search).await?;
    Ok(arr)
}

pub async fn get_page(id: Option<u64>, search: &Option<String>, page_num: u64, page_size: u64) -> Result<Page<UserRole>> {
    let mut conn = mysql::get_connect().await?;
    let page = db::user_role::get_page(&mut conn, id, search, page_num, page_size).await?;
    Ok(page)
}

pub async fn get_permission_codes(id: u64) -> Result<Vec<String>> {
    let mut conn = mysql::get_connect().await?;
    let role = db::user_role::find(&mut conn, id).await?;
    if let Some(role) = role {
        let codes = arr::parse_codes(&role.permission_codes_formatted);
        if let Some(codes) = codes {
            Ok(codes)
        } else {
            Ok(Vec::new())
        }
    } else {
        Err(Error::new(&code::NOT_FOUND, "角色不存在"))
    }
}


async fn _get_permission_codes_by_codes(conn: &mut MySqlConnection, permission_or_menu_codes: &Vec<String>) -> Result<Vec::<String>> {
    let mut permission_codes = Vec::<String>::new();
    let mut menu_codes = Vec::<String>::new();
    for code in permission_or_menu_codes {
        if code.contains(".") {
            permission_codes.push(code.clone());
        } else {
            menu_codes.push(code.clone());
        }
    }
    if menu_codes.len() > 0 {
        let ps = db::permission::batch_get_by_menu_codes(conn, &menu_codes).await?;
        for p in ps {
            permission_codes.push(p.code);
        }
    }
    Ok(permission_codes)
}