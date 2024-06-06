
use chrono::{DateTime, Utc};
use serde;
use serde::{Serialize, Serializer};

use serde_with::{self, TimestampMilliSeconds, formats::Flexible};
use sqlx;

use tt_core::json::serialize::codes_formatted_serialize;

use super::types::*;


#[serde_with::serde_as]
#[derive(Debug, Clone, serde::Serialize, sqlx::FromRow)]
pub struct UserRole {
    pub id: u64,
    pub name: String,
    pub desc: Option<String>,

    #[serde(rename = "permission_codes")]
    #[serde(serialize_with = "codes_formatted_serialize")]
    pub permission_codes_formatted: Option<String>,

    pub is_reserved: bool,
    pub status: Status,

    #[serde_as(as = "TimestampMilliSeconds<i64, Flexible>")]
    pub update_time: DateTime<Utc>,
    #[serde_as(as = "TimestampMilliSeconds<i64, Flexible>")]
    pub create_time: DateTime<Utc>,
}

