use axum::{
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct Reply<T>
    where
        T: Serialize,
{
    pub code: i32,
    pub err: bool,
    pub msg: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

pub enum Status<T>
    where
        T: Serialize,
{
    OK(Option<T>),
    Err(i32, String),
}

impl<T> Status<T>
    where
        T: Serialize,
{
    pub fn to_reply(self) -> Reply<T> {
        let mut resp = Reply {
            code: 0,
            err: false,
            msg: String::from("OK"),
            data: None,
        };

        match self {
            Status::OK(data) => {
                resp.data = data;
            }
            Status::Err(code, msg) => {
                resp.code = code;
                resp.err = true;
                resp.msg = msg;
            }
        }

        resp
    }
}


pub struct ApiOK<T>(pub Option<T>)
    where
        T: Serialize;

impl<T> IntoResponse for ApiOK<T>
    where
        T: Serialize,
{
    fn into_response(self) -> Response {
        let ApiOK(data) = self;
        let status = Status::OK(data);

        Json(status.to_reply()).into_response()
    }
}

#[allow(dead_code)]
pub enum ApiErr {
    Error(i32, String),
    ErrSystem(Option<String>),
}

use ApiErr::*;

impl IntoResponse for ApiErr {
    fn into_response(self) -> Response {
        let status = match self {
            Error(code, msg) => Status::<()>::Err(code, msg),
            ErrSystem(msg) => {
                let code = 50000;

                match msg {
                    Some(v) => Status::<()>::Err(code, v),
                    None => Status::<()>::Err(code, String::from("Internal error")),
                }
            }
        };

        Json(status.to_reply()).into_response()
    }
}

pub type Result<T> = std::result::Result<T, ApiErr>;
