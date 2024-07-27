use std::convert::Infallible;
use thiserror::Error;
use warp::{http::StatusCode, reject::Reject, Rejection, Reply};
use serde::Serialize;
use crate::models::messageModel::CreateMessageModelDto;

#[derive(Error, Debug)]
pub enum ApiError {
  #[error("Message not found")]
  NotFound,
  #[error("Invalid input: {0}")]
  BadRequest(String,u16),
  #[error("Internal server error")]
  InternalServerError
}

impl Reject for ApiError {}

#[derive(Serialize)]
pub struct ErrorResponse {
  pub code:u16,
  pub message: String,
  pub details: Option<Vec<String>>
}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
  let (status_code, code, message) = if err.is_not_found() {
    (StatusCode::NOT_FOUND, 404, "Not Found".to_string())
  } else if let Some(e) = err.find::<ApiError>() {
    match  e {
      ApiError::NotFound => (StatusCode::NOT_FOUND, 404, e.to_string()),
      ApiError::BadRequest(details, code) => (StatusCode::BAD_REQUEST, code.clone(), details.clone()),
      ApiError::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, 500, e.to_string())
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