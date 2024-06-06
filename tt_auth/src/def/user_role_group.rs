
use chrono::{DateTime, Utc};
use serde;
use serde_with::{self, TimestampMilliSeconds, formats::Flexible};
use sqlx;

use tt_core::json::serialize::ids_formatted_serialize;

use super::types::*;


#[serde_with::serde_as]
#[derive(Debug, Clone, serde::Serialize, sqlx::FromRow)]
pub struct UserRoleGroup {
    pub id: u64,
    pub name: String,
    pub desc: Option<String>,

    #[serde(rename = "role_ids")]
    #[serde(serialize_with = "ids_formatted_serialize")]
    pub role_ids_formatted: Option<String>,

    pub is_reserved: bool,
    pub status: Status,

    #[serde_as(as = "TimestampMilliSeconds<i64, Flexible>")]
    pub update_time: DateTime<Utc>,
    #[serde_as(as = "TimestampMilliSeconds<i64, Flexible>")]
    pub create_time: DateTime<Utc>,
}
