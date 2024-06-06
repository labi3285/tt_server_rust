use tt_core::def::res::Result;


mod db_mysql;

#[tokio::main]
async fn main() -> Result<()> {

    // db_mysql::drop_table("user").await?;
    db_mysql::exec_sql("



    delete from permission_menu where code = 'admin'

    ").await?;

    Ok(())


}


