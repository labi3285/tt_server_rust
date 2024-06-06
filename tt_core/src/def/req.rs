use axum::http;

use super::err::{code, Error};
use super::res::Result;


pub fn get_http_header_val(headers: &http::HeaderMap, key: &str) -> Result<String> {
    let res = headers
        .get::<String>(key.to_string())
        .ok_or(Error::new(&code::REQUEST, "请求参数错误"));
    match res {
        Ok(e) => {
            let res = e.to_str();
            match res {
                Ok(e) => Ok(e.to_string()),
                Err(err) => {
                    let trace = format!("get_http_header_val.to_str {}:{:?}", key, err);
                    Err(Error::trace(&code::REQUEST, "请求参数错误", &Some(trace)))
                }
            }
        }
        Err(err) => {
            let trace = format!("get_http_header_val.get {}:{:?}", key, err);
            Err(Error::trace(&code::REQUEST, "请求参数错误", &Some(trace)))
        },
    }
}
