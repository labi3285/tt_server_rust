
use axum::http;
use tt_core::def::res::Result;
use tt_core::def::err::Error;

use tt_core::def::resp_json::Page;
use tt_core_database::pool::mysql;

use crate::def::err::code;
use crate::db;

use crate::def::token::TokenUser;
use crate::util::jwt;

use crate::def::types::*;
use crate::def::user::*;


pub async fn check_or_add_admin() -> Result<bool> {
    let mut conn = mysql::get_connect().await?;
    let account = "admin".to_string();
    let password = "123456".to_string();
    let user = db::user::find_by_account(&mut conn, &account).await?;
    match user {
        Some(_) => Ok(false),
        None => {
            let _ = db::user::add(
                &mut conn, 
                &None,
                &Some(account), 
                &None,
                &None,
                &None,
                &None,
                &None,
                &None,
                &Some(password), 
                true
            ).await?;
            Ok(true)
        }
    }
}


pub async fn register_by_account(account: &String, password: &String) -> Result<u64> {
    if account.len() < 6 {
        return Err(Error::new(&code::VALIDATE, "账户名无效"))
    }
    if password.len() < 6 {
        return Err(Error::new(&code::VALIDATE, "密码太短"))
    }
    let mut conn = mysql::get_connect().await?;
    let user = db::user::find_by_account(&mut conn, account).await?;
    if let Some(_) = user {
        return Err(Error::new(&code::NOT_FOUND, ""))
    }
    let id = db::user::add(
        &mut conn, 
        &None,
        &Some(account.clone()), 
        &None,
        &None,
        &None,
        &None,
        &None,
        &None,
        &Some(password.clone()),
        false,
    ).await?;
    Ok(id)
}


pub async fn login_by_account(account: &String, password: &String) -> Result<User> {
    println!("{}{}", account, password);
    if account.len() < 5 {
        return Err(Error::new(&code::VALIDATE, "账户名无效"))
    }
    if password.len() < 6 {
        return Err(Error::new(&code::VALIDATE, "密码错误"))
    }
    let mut conn = mysql::get_connect().await?;
    let user = db::user::find_by_account(&mut conn, account).await?;
    let user = user.ok_or(Error::new(&code::VALIDATE, "账户名不存在"))?;
    let user_password = user.password.clone().ok_or(Error::new(&code::VALIDATE, "密码错误"))?;
    if *password != *user_password {
        return Err(Error::new(&code::VALIDATE, "密码错误"))
    }
    Ok(user)
}

pub fn generate_token(user: &User) -> Result<String> {
    jwt::make_token(user, &"123456".to_string(), 60 * 60 * 24 * 30)
}

pub fn validate_token_user_from_token(token: &String) -> Result<TokenUser> {
    jwt::parse_token::<TokenUser>(token, &"123456".to_string())
}

pub async fn find(id: u64) -> Result<User> {
    let mut conn = mysql::get_connect().await?;
    let user = db::user::find(&mut conn, id).await?;
    let user = user.ok_or(Error::new(&code::NOT_FOUND, "用户不存在"))?;
    Ok(user)
}

pub async fn get(
    id: Option<u64>,
    role_group_id: Option<u64>,
    account: &Option<String>,
    phone: &Option<String>,
    email: &Option<String>,
    nickname: &Option<String>,
    gender: &Option<UserGender>,
) -> Result<Vec<User>> {
    let mut conn = mysql::get_connect().await?;
    let arr = db::user::get(
        &mut conn,
        id, 
        role_group_id,
        &account,
        &phone,
        &email,&nickname,
        &gender).await?;
    Ok(arr)
}

pub async fn get_page(
    id: Option<u64>,
    role_group_id: Option<u64>,
    account: &Option<String>,
    phone: &Option<String>,
    email: &Option<String>,
    nickname: &Option<String>,
    gender: &Option<UserGender>,
    page_num: u64, page_size: u64
) -> Result<Page<User>> {
    let mut conn = mysql::get_connect().await?;
    let arr = db::user::get_page(
        &mut conn,
        id,
        role_group_id,
        &account,
        &phone,
        &email,
        &nickname,
        &gender,
        page_num, page_size).await?;
    Ok(arr)
}
