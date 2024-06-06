use jsonwebtoken;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use tt_core::def::res::Result;
use tt_core::def::err::Error;
use tt_core::time;
use tt_core::util::uuid;

use crate::def::err::code;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims<T: Serialize> {
    payload: T,

    iss: String, // jwt签发者
    sub: Option<String>, // jwt所面向的用户
    aud: Option<String>, // 接收jwt的一方

    iat: i64, // jwt的签发时间
    nbf: i64, // 定义在什么时间之前，该token都是不可用的
    exp: i64, // 过期时间

    jti: String, // 唯一标识
}

pub fn make_token<T: Serialize + Clone>(payload: &T, secret_key: &String, expire_secs: i64) -> Result<String> {
    let now = time::now();
    let expire_in = time::add(now, expire_secs);
    let aa1 = time::format(&now, &time::Pattern::Standard);
    let aa2 = time::format(&expire_in, &time::Pattern::Standard);
    println!("{}  {}", aa1, aa2);
    let header = jsonwebtoken::Header::default();
    let key = jsonwebtoken::EncodingKey::from_secret(secret_key.as_bytes());
    let claims = Claims {
        payload: payload.clone(),
        iss: String::from("com.tt.jwt"),
        sub: None,
        aud: None,
        iat: now.timestamp(),
        nbf: now.timestamp(),
        exp: expire_in.timestamp(),
        jti: uuid::v4(),
    };
    let res = jsonwebtoken::encode(&header, &claims, &key);
    match res {
        Ok(token) => Ok(token),
        Err(err) => {
            let trace = format!("make_token: {:?}", err);
            return Err(Error::trace(&code::TOKEN, "生成令牌失败", &Some(trace)));
        }
    }
}

pub fn parse_token<T: Serialize + DeserializeOwned>(token: &String, secret_key: &String) -> Result<T> {
    let key = jsonwebtoken::DecodingKey::from_secret(secret_key.as_bytes());
    let validation = jsonwebtoken::Validation { ..jsonwebtoken::Validation::default() };
    let res = jsonwebtoken::decode::<Claims<T>>(&token, &key, &validation);
    match res {
        Ok(v) => Ok(v.claims.payload),
        Err(err) => {
            let trace = format!("parse_token: {:?}", err);
            return Err(Error::trace(&code::TOKEN, "解析令牌失败", &Some(trace)));
        }
    }
}
