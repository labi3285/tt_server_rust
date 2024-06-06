use serde::{self, Serialize};

use crate::util::arr;

#[allow(unused)]
pub fn codes_formatted_serialize<S>(value: &Option<String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let val = arr::parse_codes(value);
    val.serialize(serializer)
}

#[allow(unused)]
pub fn ids_formatted_serialize<S>(value: &Option<String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let val = arr::parse_ids(value);
    val.serialize(serializer)
}