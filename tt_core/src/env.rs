use std::env;
use std::str::FromStr;
use lazy_static::lazy_static;

use dotenv::dotenv;

use crate::def::res::Result;
use crate::def::err::{ code, Error };
use crate::util::val;

#[allow(unused)]
pub fn str(key: &str) -> Result<String> {
    return _str(key);
}
#[allow(unused)]
pub fn strs(key: &str, sep: &str) -> Result<Vec<String>> {
    let str = _str(key)?;
    let arr: Vec<&str> = str.split(sep).collect();
    return arr.iter().map(|&a| {
        Ok(a.to_string())
    }).collect();
}

#[allow(unused)]
pub fn val<F: FromStr>(key: &str) -> Result<F> {
    let str = _str(key)?;
    return val::str2val::<F>(&str);
}

#[allow(unused)]
pub fn vals<F: FromStr>(key: &str, sep: &str) -> Result<Vec<F>> {
    let str = _str(key)?;
    let arr: Vec<&str> = str.split(sep).collect();
    return arr.iter().map(|&a|val::str2val::<F>(&(a.to_string()))).collect();
}


fn _str(key: &str) -> Result<String> {
    if !*ENV_EXSIT {
        let trace = format!(".env.key:{} parse error:  dotenv load failed", key);
        return Err(Error::trace(&code::ENV, "未找到环境变量", &Some(trace)));
    }
    let res = env::var(key);
    match res {
        Ok(val) => {
            return Ok(val);
        },
        Err(err) => {
            let trace = format!(".env.key:{} parse: {:?}", key, err);
            return Err(Error::trace(&code::ENV, "解析环境变量失败", &Some(trace)));
        }
    }
}

lazy_static! {
    static ref ENV_EXSIT: bool = {
        let res = dotenv().ok();
        match res {
            Some(_) => {
                return true;
            },
            None => {
                return false;
            }
        }
    };
}


