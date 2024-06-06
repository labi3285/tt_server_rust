
use axum::{body, http, response::{IntoResponse, Response}};

use super::resp_json::Payload;

pub mod code {
    use super::Code;
    
    pub static ENV: Code =             Code(1, "环境错误");
    pub static SERVER: Code =             Code(2, "环境错误");
    pub static REQUEST: Code =           Code(3, "请求错误");

    pub static PARSE: Code =           Code(4, "解析失败");
    pub static SERIALIZE: Code =       Code(5, "序列化失败");
    pub static NOT_FOUND: Code =       Code(6, "未找到");
}


#[derive(Debug)]
pub struct Code(pub i32, pub &'static str);

#[allow(unused)]
#[derive(Debug)]
pub struct Error {
    pub code: &'static Code,
    pub message: String,
    pub trace: Option<String>,
}
impl Error {
    #[allow(unused)]
    pub fn new(code: &'static Code, message: &str) -> Self {
        Error {
            code,
            message: message.to_string(),
            trace: None,
        }
    }
    #[allow(unused)]
    pub fn trace(code: &'static Code, message: &str, trace: &Option<String>) -> Self {
        Error {
            code,
            message: message.to_string(),
            trace: trace.clone(),
        }
    }
    #[allow(unused)]
    fn to_payload(self) -> Payload<String> {
        Payload {
            code: self.code.0,
            message: self.message,
            data: None,
            trace: self.trace
        }
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error:{}({}),{:?}", self.message, self.code.0, self.trace)
    }
}
impl IntoResponse for Error {
    fn into_response(self) -> Response<body::Body> {
        let payload = self.to_payload();
        let json = serde_json::to_string(&payload).unwrap();
        let body = body::Body::new(json);
        Response::builder()
            .status(http::StatusCode::OK)
            .header("Content-Type", "application/json")
            .body(body)
            .unwrap()
    }
}
