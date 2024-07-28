pub mod error_codes;

use std::convert::Infallible;
use thiserror::Error;
use tokio::sync::broadcast::error;
use warp::{http::StatusCode, reject::Reject, Rejection, Reply};
use serde::Serialize;
use crate::models::messageModel::CreateMessageModelDto;
use crate::errors::error_codes::ERROR_CODES;
use crate::errors::error_codes::ErrorCodes;

#[derive(Error, Debug)]
pub enum ApiError {
  #[error("Message not found")]
  NotFound,
  #[error("Invalid input: {0}")]
  BadRequest(String,u16),
  #[error("Internal server error")]
  InternalServerError,
  #[error("custom")]
  ErrorCode(ErrorCodes)
}

impl Reject for ApiError {}

#[derive(Serialize)]
pub struct ErrorResponse {
  pub code:u16,
  pub message: String,
  pub details: Option<Vec<String>>
}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
  let dict = ERROR_CODES.read().unwrap();
  let (status_code, code, message) = if err.is_not_found() {
    (StatusCode::NOT_FOUND, 404, "Not Found".to_string())
  } else if let Some(e) = err.find::<ApiError>() {
    match  e {
      ApiError::NotFound => (StatusCode::NOT_FOUND, 404, e.to_string()),
      ApiError::BadRequest(details, code) => (StatusCode::BAD_REQUEST, code.clone(), details.clone()),
      ApiError::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, 500, e.to_string()),
      ApiError::ErrorCode(code) => {
        if let Some(errorcode) = dict.get(code){
          (errorcode.status_code, errorcode.code, errorcode.message.to_string())
        } else {
          (StatusCode::INTERNAL_SERVER_ERROR, 500, e.to_string())
        }
        
      }
    }
   } else {
      (StatusCode::INTERNAL_SERVER_ERROR, 500, "Internal Server Error".to_string())        
   };

   let json = warp::reply::json(&ErrorResponse {
    code,
    message: message,
    details: None
   });

   Ok(warp::reply::with_status(json, status_code))
}