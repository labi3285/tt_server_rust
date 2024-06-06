use lazy_static::lazy_static;
use chrono::prelude::*;

use crate::def::err::{ code, Error };
use crate::env;

#[allow(unused)]
pub enum Pattern {
    Standard,
    Date,
    Time,
}
impl Pattern {
    pub fn fmt<'a>(&self) -> &'a str {
        match &self {
            Pattern::Standard => {
                return "%Y-%m-%d %H:%M:%S";
            },
            Pattern::Date => {
                return "%Y-%m-%d";
            },
            Pattern::Time => {
                return "%H:%M:%S";
            }
        }
    }
}

#[allow(unused)]
pub fn now() -> DateTime<Utc> {
    let dt = Utc::now();
    return dt;
}

#[allow(unused)]
pub fn add(time: DateTime<Utc>, secs: i64) -> DateTime<Utc> {
    let dt = Utc::now();
    dt + chrono::Duration::seconds(secs)
}

#[allow(unused)]
pub fn format(time: &DateTime<Utc>, pattern: &Pattern) -> String {
    let tz_offset = FixedOffset::east_opt(*TIME_ZONE * 3600).unwrap();
    let tz_time = tz_offset.from_utc_datetime(&time.naive_utc());
    return tz_time.format( pattern.fmt()).to_string();
}

#[allow(unused)]
pub fn parse(time_str: &String, pattern: &Pattern) -> Result<DateTime<Utc>, Error> {
    let _dt_zone = format!(" +0{}00", *TIME_ZONE);
    let _time_str: String;
    let _fmt: String;
    match pattern {
        Pattern::Standard => {
            _time_str = format!("{}.000{}", time_str, _dt_zone);
            _fmt = format!("{}%.3f %z", pattern.fmt());
        },
        Pattern::Date => {
            _time_str = format!("{} 00:00:00.000{}", time_str, _dt_zone);
            _fmt = format!("{} %H:%M:%S%.3f %z", pattern.fmt());
        },
        Pattern::Time => {
            _time_str = format!("1971-01-01 {}.000{}", time_str, _dt_zone);
            _fmt = format!("%Y-%m-%d {}%.3f %z", pattern.fmt());
        }
    }
    let res = DateTime::parse_from_str(&_time_str, &_fmt);
    match res {
        Ok(dt) => {
            return Ok(dt.with_timezone(&Utc));
        },
        Err(err) => {
            let trace = format!("time_str:{} parse DateTime error: {}", time_str, err.to_string());
            return Err(Error::trace(&code::PARSE, "解析时间失败", &Some(trace)));
        },
    }
}

lazy_static! {
    static ref TIME_ZONE: i32 = env::val::<i32>("TIME_ZONE").unwrap_or(0);
}
