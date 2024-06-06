
use tt_core::def::res::Result;
use tt_core::def::resp_json::Page;
use tt_core_database::pool::mysql;

use crate::db;
use crate::def::user_role::*;

// pub async fn add(name: String, desc: Option<String>) -> Result<u64> {
//     let mut conn = mysql::get_connect().await?;
//     let id = db::user_role::add(&mut conn, name, desc).await?;
//     Ok(id)
// }

// pub async fn get() -> Result<Vec<AdminUserRole>> {
//     let mut conn = mysql::get_connect().await?;
//     let arr = db::user_role::get(&mut conn).await?;
//     Ok(arr)
// }
// pub async fn get_page(page_num: u64, page_size: u64) -> Result<Page<AdminUserRole>> {
//     let mut conn = mysql::get_connect().await?;
//     let page = db::user_role::get_page(&mut conn, page_num, page_size).await?;
//     Ok(page)
// }