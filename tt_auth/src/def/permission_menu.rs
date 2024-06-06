
use chrono::{DateTime, Utc};
use serde;
use serde_with::{self, TimestampMilliSeconds, formats::Flexible};
use sqlx;

use super::permission::*;


#[serde_with::serde_as]
#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct PermissionMenu {
    pub code: String,
    pub name: String,

    pub is_reserved: bool,

    #[serde_as(as = "TimestampMilliSeconds<i64, Flexible>")]
    pub update_time: DateTime<Utc>,
    #[serde_as(as = "TimestampMilliSeconds<i64, Flexible>")]
    pub create_time: DateTime<Utc>,
}



#[serde_with::serde_as]
#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct PermissionMenuWithPermissions {
    pub code: String,
    pub name: String,
    pub permissions: Vec<Permission>,
}

impl From<PermissionMenu> for PermissionMenuWithPermissions {
    fn from(e: PermissionMenu) -> Self {
        PermissionMenuWithPermissions {
            code: e.code,
            name: e.name,
            permissions: vec![],
        }
    }
}