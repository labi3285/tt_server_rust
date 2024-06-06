use once_cell::sync::Lazy;
use sqlx::Acquire;
use std::{collections::HashMap, str::FromStr};

use sqlx::{
    self,
    Transaction,
    mysql::{MySqlConnectOptions, MySqlPoolOptions},
    pool::PoolConnection,
    MySql, Pool,
};

use tt_core::def::res::Result;
use tt_core::def::err::Error;
use tt_core::env;

use crate::def::err::code;
use crate::def;


static mut POOLS: Lazy<HashMap<&'static str, Pool<MySql>>> = Lazy::new(|| {
    HashMap::new()
});


#[allow(unused)]
pub async fn get_connect() -> Result<PoolConnection<MySql>> {
    _get_connect(def::DEFAULT).await
}

#[allow(unused)]
pub async fn get_connect_from_database(
    which_database: &'static str,
) -> Result<PoolConnection<MySql>> {
    _get_connect(which_database).await
}

#[allow(unused)]
pub async fn setup() -> Result<()> {
    _setup(def::DEFAULT).await
}

#[allow(unused)]
pub async fn setup_database(which_database: &'static str) -> Result<()> {
    _setup(which_database).await
}

#[allow(unused)]
pub async fn get_transaction<'q>(conn: &'q mut PoolConnection<MySql>) -> Result<Transaction<'q, sqlx::MySql>> {
    _get_transaction(&mut *conn).await
}

#[allow(unused)]
pub async fn commit<'q>(trans: Transaction<'q, MySql>) -> Result<()> {
    let res = trans.commit().await;
    match res {
        Ok(_) =>  Ok(()),
        Err(err) => {
            let trace = format!("commit: {:?}", err);
            return Err(Error::trace(&code::COMMIT, "数据提交报错", &Some(trace)));
        }
    }
}

#[allow(unused)]
pub async fn rollback<'q>(trans: Transaction<'q, MySql>) -> Result<()> {
    let res = trans.rollback().await;
    match res {
        Ok(_) =>  Ok(()),
        Err(err) => {
            let trace = format!("rollback_transaction: {:?}", err);
            return Err(Error::trace(&code::ROLLBACLK, "数据回滚报错", &Some(trace)));
        }
    }
}

async fn _get_transaction<'q>(conn: &'q mut PoolConnection<MySql>) -> Result<Transaction<'q, sqlx::MySql>> {
    let res = conn.begin().await;
    match res {
        Ok(tx) => Ok(tx),
        Err(err) => {
            let trace = format!("conn.begin error: {}", err);
            return Err(Error::trace(&code::TRANS, "数据回滚报错", &Some(trace)));
        }
    }
}

async fn _get_connect(which_database: &'static str) -> Result<PoolConnection<MySql>> {
    let res = unsafe { POOLS.get(which_database) };
    if let Some(pool) = res {
        let connect = pool.acquire().await;
        match connect {
            Ok(con) => Ok(con),
            Err(err) => {
                let trace = format!("pool.acquire: {:?}", err);
                return Err(Error::trace(&code::CONNECT, "获取数据连接失败", &Some(trace)));
            }
        }
    } else {
        return Err(Error::trace(&code::CONNECT, "获取连接池失败", &None));
    }
}

async fn _setup(which_database: &'static str) -> Result<()> {
    let mut which = "MYSQL".to_string();
    if which_database != def::DEFAULT {
        which = format!("MYSQL.{}", which_database);
    }
    let url = env::str(&format!("{}.URL", which))?;
    let database: String = env::str(&format!("{}.DATABASE", which))?;
    let user_name = env::str(&format!("{}.USER_NAME", which))?;
    let password = env::str(&format!("{}.PASSWORD", which))?;
    let max_connects = env::val::<u32>(&format!("{}.MAX_CONNECTS", which))?;
    let full_url = format!("mysql://{}:{}@{}/{}", user_name, password, url, database);

    let res = MySqlConnectOptions::from_str(&full_url);
    match res {
        Ok(connection_options) => {
            let res = MySqlPoolOptions::new()
                .max_connections(max_connects)
                .connect_with(connection_options)
                .await;
            match res {
                Ok(pool) => {
                    unsafe { POOLS.insert(which_database, pool) };
                    Ok(())
                }
                Err(err) => {
                    let trace = format!("MySqlPoolOptions::new error: {}", err);

                    Err(Error::trace(&code::CONNECT, "连接数据库报错", &Some(trace)))
                }
            }
        }
        Err(err) => {
            let trace = format!("MySqlConnectOptions::from_str:{} error: {}", &url, err);
            Err(Error::trace(&code::CONNECT, "解析数据库连接失败", &Some(trace)))
        }
    }
}
