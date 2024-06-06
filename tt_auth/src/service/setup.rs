
use tt_core::def::res::Result;

use tt_core_database::pool::mysql;

use crate::db;


pub async fn check_or_add_admin() -> Result<bool> {
    let mut conn = mysql::get_connect().await?;
    let account = "admin".to_string();
    let password = "123456".to_string();
    let user = db::user::find_by_account(&mut conn, &account).await?;
    match user {
        Some(_) => Ok(false),
        None => {
            let mut tx = mysql::get_transaction(&mut conn).await?;
            match async {
                let menu_code = "admin".to_string();
                let permission_code = "admin.admin".to_string();
                db::permission::add(&mut tx, &permission_code, &menu_code, &"超级管理员".to_string(), true).await?;
                db::permission_menu::add(&mut tx, &menu_code, &"超级管理员".to_string(), true).await?;
                let role_id = db::user_role::add(&mut tx, &"超级管理员".to_string(), &Some(vec![permission_code]), &None, true).await?;
                let group_id = db::user_role_group::add(&mut tx, &"超级管理员".to_string(), &Some(vec![role_id]), &None, true).await?;
                let _ = db::user::add(
                    &mut tx, 
                    &Some(vec![group_id]),
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
    }
}


