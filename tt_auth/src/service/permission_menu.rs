

use tt_core::def::res::Result;
use tt_core::def::err::Error;

use tt_core::def::resp_json::Page;
use tt_core_database::pool::mysql;

use crate::db;
use crate::def::err::code;
use crate::def::permission_menu::*;
use crate::def::permission::*;

pub async fn add(code: &String, name: &String, is_reserved: bool) -> Result<u64> {
    let mut conn = mysql::get_connect().await?;
    let id = db::permission_menu::add(&mut conn, code, name, is_reserved).await?;
    Ok(id)
}

pub async fn del(code: &String) -> Result<u64> {
    let mut conn = mysql::get_connect().await?;
    let menu = db::permission_menu::find(&mut conn, code).await?.ok_or(Error::new(&code::NOT_FOUND, "菜单不存在"))?;
    if menu.is_reserved {
        return Err(Error::new(&code::RESERVED, "保留数据"));
    }
    let id = db::permission_menu::del(&mut conn, code).await?;
    db::permission::del_by_menu_code(&mut conn, code).await?;
    Ok(id)
}

pub async fn get(code: &Option<String>, name: &Option<String>) -> Result<Vec<PermissionMenu>> {
    let mut conn = mysql::get_connect().await?;
    let arr = db::permission_menu::get(&mut conn, code, name).await?;
    Ok(arr)
}
pub async fn get_page(code: &Option<String>, name: &Option<String>, page_num: u64, page_size: u64) -> Result<Page<PermissionMenu>> {
    let mut conn = mysql::get_connect().await?;
    let page = db::permission_menu::get_page(&mut conn, code, name, page_num, page_size).await?;
    Ok(page)
}

pub async fn get_tree() -> Result<Vec<PermissionMenuWithPermissions>> {
    let mut conn = mysql::get_connect().await?;
    let menus = db::permission_menu::get(&mut conn, &None, &None).await?;
    let mut permissions = db::permission::get(&mut conn, &None, &None, &None).await?;
    let mut arr = Vec::<PermissionMenuWithPermissions>::new();
    for m in menus {
        let mut a = PermissionMenuWithPermissions::from(m);
        let mut a_arr = Vec::<Permission>::new();
        loop {
            if permissions.len() > 0 && permissions[0].menu_code == a.code {
                let p = permissions.pop().unwrap();
                a_arr.push(p);
            } else {
                break;
            }
        }
        a.permissions = a_arr;
        arr.push(a);
    }
    Ok(arr)
}