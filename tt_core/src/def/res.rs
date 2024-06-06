use super::err;

pub type Result<T, E = err::Error> = std::result::Result<T, E>;
