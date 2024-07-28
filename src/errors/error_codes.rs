use std::collections::HashMap;
use std::sync::RwLock;
use lazy_static::lazy_static;
use warp::http::StatusCode;

#[derive(Debug)]
pub struct Errorcode {
  pub code: u16,
  pub status_code: StatusCode,
  pub message: String
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum ErrorCodes {
    NotNull = 1001,
    NotEmpty = 1002,
    MaxSize = 1003
}

lazy_static! {
    pub static ref ERROR_CODES: RwLock<HashMap<ErrorCodes, Errorcode>> = {
        let mut m = HashMap::new();

        m.insert(ErrorCodes::NotNull, Errorcode {
            code: ErrorCodes::NotNull as u16,
            status_code: StatusCode::BAD_REQUEST,
            message: String::from("Content must not be null"),
        });

        m.insert(ErrorCodes::NotEmpty, Errorcode {
            code: ErrorCodes::NotEmpty as u16,
            status_code: StatusCode::BAD_REQUEST,
            message: String::from("Content must not be empty"),
        });

        m.insert(ErrorCodes::MaxSize, Errorcode {
            code: ErrorCodes::MaxSize as u16,
            status_code: StatusCode::BAD_REQUEST,
            message: String::from("Content must be maximum size 32"),
        });

        RwLock::new(m)
    };
}