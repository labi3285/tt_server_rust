
use chrono::{DateTime, Utc};
use serde;
use serde_with::{self, TimestampMilliSeconds, formats::Flexible};
use sqlx;


#[serde_with::serde_as]
#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct Permission {
    pub code: String,
    pub menu_code: String,
    pub name: String,

    pub is_reserved: bool,

    #[serde_as(as = "TimestampMilliSeconds<i64, Flexible>")]
    pub update_time: DateTime<Utc>,
    #[serde_as(as = "TimestampMilliSeconds<i64, Flexible>")]
    pub create_time: DateTime<Utc>,
}

#[serde_with::serde_as]
#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct PermissionWithMenuName {
    pub code: String,
    pub menu_code: String,
    pub name: String,
    pub menu_name: String,

    #[serde_as(as = "TimestampMilliSeconds<i64, Flexible>")]
    pub update_time: DateTime<Utc>,
    #[serde_as(as = "TimestampMilliSeconds<i64, Flexible>")]
    pub create_time: DateTime<Utc>,
}
