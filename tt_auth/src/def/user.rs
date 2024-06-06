use chrono::{DateTime, Utc};
use serde;
use serde_with::{self, TimestampMilliSeconds, formats::Flexible};
use sqlx;

use tt_core::json::serialize::ids_formatted_serialize;

use super::types::*;


#[serde_with::serde_as]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: u64,

    #[serde(rename = "role_group_ids")]
    #[serde(serialize_with = "ids_formatted_serialize")]
    pub role_group_ids_formatted: Option<String>,

    pub account: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub wx_open_id: Option<String>,

    pub nickname: Option<String>,
    pub gender: Option<UserGender>,
    pub avator: Option<String>,

    #[serde(skip_serializing)]
    pub password: Option<String>,

    pub is_reserved: bool,
    pub status: Status,

    #[serde_as(as = "TimestampMilliSeconds<i64, Flexible>")]
    pub update_time: DateTime<Utc>,
    #[serde_as(as = "TimestampMilliSeconds<i64, Flexible>")]
    pub create_time: DateTime<Utc>,
}

