use std::future::Future;

use axum::{
    body, http, response::{ IntoResponse, Response }
};
use serde;
use serde_json;

use crate::def::res::Result;

#[allow(unused)]
#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct Page<T: serde::Serialize> {
    pub page_num: u64,
    pub page_size: u64,
    pub total: u64,
    pub list: Vec<T>,
}

#[allow(unused)]
pub enum PageOrNot<T: serde::Serialize> {
    Page(Page<T>),
    Not(Vec<T>),
}
#[allow(unused)]
impl<T: serde::Serialize> serde::Serialize for PageOrNot<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        match self {
            PageOrNot::Page(page)=> {
                page.serialize(serializer)
            },
            PageOrNot::Not(vec) => {
                vec.serialize(serializer)
            },
        }
    }
}

#[allow(unused)]
#[derive(Debug, serde::Serialize)]
pub struct Payload<T: serde::Serialize> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
    pub trace: Option<String>,

}
#[allow(unused)]
impl<T: serde::Serialize> Payload<T> {
    pub fn data(data: T) -> Self {
        Payload {
            code: 0,
            message: "ok".to_string(),
            data: Some(data),
            trace: None
        }
    }
    pub fn empty() -> Self {
        Payload {
            code: 0,
            message: "ok".to_string(),
            data: None,
            trace: None
        }
    }
}

#[allow(unused)]
#[derive(Debug)]
pub struct ApplicationJson<T: serde::Serialize> {
    pub payload: T
}
impl<T: serde::Serialize> ApplicationJson<Payload<T>> {
    pub fn payload(data: T) -> Self {
        let payload = Payload::data(data);
        ApplicationJson { payload }
    }
    pub fn from_result(res: Result<T>) -> ApplicationJson<Payload<T>> {
        match res {
            Ok(data) => {
                let payload = Payload::data(data);
                ApplicationJson { payload }
            },
            Err(err) => {
                let mut payload = Payload::<T>::empty();
                payload.code = err.code.0;
                payload.message = err.message;
                payload.trace = err.trace;
                ApplicationJson { payload }
            }
        }
    }
}
impl<T: serde::Serialize> IntoResponse for ApplicationJson<T> {
    fn into_response(self) -> Response<body::Body> {
        let json = serde_json::to_string(&self.payload).unwrap();
        let body = body::Body::new(json);
        Response::builder()
            .status(http::StatusCode::OK)
            .header("Content-Type", "application/json")
            .body(body)
            .unwrap()
    }
}

#[allow(unused)]
pub async fn application_json_handler<F, T: serde::Serialize>(future: F) -> Response
where  
    F: Future<Output = Result<T>>,  
{  
    let res = future.await;
    match res {
        Ok(data) => {
            let payload = Payload::data(data);
            ApplicationJson { payload }.into_response()
        },
        Err(err) => {
            let mut payload = Payload::<String>::empty();
            payload.code = err.code.0;
            payload.message = err.message;
            payload.data = err.trace;
            ApplicationJson { payload }.into_response()
        }
    }
}

