

use super::user::User;


#[serde_with::serde_as]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct WrapTokenUser {
    pub user: User,
    pub token: String,
}

#[serde_with::serde_as]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct TokenUser {
    pub id: u64,
    pub role_group_ids_formatted: Option<String>,
    pub account: Option<String>,
}
impl From<User> for TokenUser {
    fn from(e: User) -> Self {
        TokenUser {
            id: e.id,
            account: e.account,
            role_group_ids_formatted: e.role_group_ids_formatted,
        }
    }
}