use crate::env;
use crate::def::res::Result;
use crate::def::err::{ code, Error };

use axum::Router;

#[allow(unused)]
pub async fn serve(router: Router) -> Result<()> {
    let port = env::str("SERVER_PORT")?;
    let addr = format!("127.0.0.1:{}", port);
    let res = tokio::net::TcpListener::bind(&addr).await;
    match res {
        Ok(listener) => {
            println!("serve at: http://{}", addr);
            // crate::log_info!("serve at: http://{}", addr);
            let res = axum::serve(listener, router).await;
            match res {
                Ok(_) => {
                    return Ok(())
                },
                Err(err) => {
                    let trace = format!("axum::serve: {:?}", err);
                    return Err(Error::trace(&code::SERVER, "启动服务失败", &Some(trace)));
                }
            }
        },
        Err(err) => {
            let trace = format!("TcpListener::bind: {:?}", err);
            return Err(Error::trace(&code::SERVER, "绑定端口失败", &Some(trace)));
        },
    }
}

