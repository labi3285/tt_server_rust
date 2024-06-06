use serde;
use serde_repr;
use sqlx;

#[repr(u8)]
#[derive(Debug, Clone, serde_repr::Serialize_repr, serde_repr::Deserialize_repr, sqlx::Type)]
pub enum UserGender {
    Man = 0,
    Woman = 1,
    Other = 2,
}

#[repr(u8)]
#[derive(Debug, Clone, serde_repr::Serialize_repr, serde_repr::Deserialize_repr, sqlx::Type)]
pub enum DataType {
    Reserved = 0,
    Normal = 1,
}

#[repr(u8)]
#[derive(Debug, Clone, serde_repr::Serialize_repr, serde_repr::Deserialize_repr, sqlx::Type)]
pub enum Status {
    Forbidden = 0,
    Normal = 1,
}