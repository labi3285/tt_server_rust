use tt_core::def::res::Result;

use tt_core_database::pool;
use tt_core_database::db::mysql;

#[allow(unused)]
pub async fn drop_table(table: &str) -> Result<(u64, u64)> {
    tt_core_database::pool::mysql::setup().await?;
    let mut conn = pool::mysql::get_connect().await?;
    let sql = format!("drop table if exists {}", table);
    let query = mysql::query(sql.as_str());
    let res = mysql::exec(&mut conn, query).await?;
    println!("drop_table {}:{:?}", table, res);
    Ok(res)
}

#[allow(unused)]
pub async fn exec_sql(sql: &str) -> Result<(u64, u64)> {
    tt_core_database::pool::mysql::setup().await?;
    let mut conn = pool::mysql::get_connect().await?;

    let query = mysql::query(sql);
    let res = mysql::exec(&mut conn, query).await?;
    println!("exec_sql:{:?}", res);
    Ok(res)
}