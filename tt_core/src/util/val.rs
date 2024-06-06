

use std::any::type_name;
use std::str::FromStr;

use crate::def::res::Result;
use crate::def::err::{ code, Error };

#[allow(unused)]
pub fn str2val<F: FromStr>(v: &String) -> Result<F> {
    let mut v = &v[..];
    if type_name::<F>() == type_name::<bool>() {
        if v == "1" || v == "true" || v == "T" || v == "TRUE" || v == "yes" || v == "YES" {
            v = "true";
        } else if v == "0" || v == "false" || v == "F" || v == "FALSE" || v == "NO" || v == "NO" {
            v = "false";
        } else {
            let trace = format!("{} cannot parse to bool", v);
            return Err(Error::trace(&code::PARSE, "解析失败", &Some(trace)));
        }
    }
    let res = v.parse::<F>();
    match res {
        Ok(val) => {
            return Ok(val);
        },
        Err(_) => {
            let trace = format!("{} cannot parse to val", v);
            return Err(Error::trace(&code::PARSE, "解析失败", &Some(trace)));
        }
    }
}

