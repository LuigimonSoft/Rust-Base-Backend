pub mod error_codes;

use std::convert::Infallible;
use thiserror::Error;
use warp::{http::StatusCode, reject::Reject, Rejection, Reply};
use serde::Serialize;
use crate::errors::error_codes::ERROR_CODES;
use crate::errors::error_codes::ErrorCodes;
use crate::models::error_response::ErrorResponse;
use crate::models::error_response::ValidationProblem;

#[derive(Error, Debug)]
pub enum ApiError {
  #[error("Message not found")]
  NotFound,
  #[error("Invalid input: {0}")]
  BadRequest(String,u16),
  #[error("Internal server error")]
  InternalServerError,
  #[error("custom")]
  ErrorCode(ErrorCodes),
  #[error("Multiple validation errors")]
  MultipleErrors(Option<Vec<ErrorCodes>>, Option<String>, Option<String>)
}

impl Reject for ApiError {}



pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
  let dict = ERROR_CODES.read().unwrap();
  let errors: ErrorResponse = if err.is_not_found() {
    ErrorResponse { title: "Not Found".to_string(), status: StatusCode::NOT_FOUND.as_u16(), instance: None, details: None}
  } else if let Some(e) = err.find::<ApiError>() {
    match  e {
      ApiError::NotFound => ErrorResponse { title: e.to_string(), status: StatusCode::NOT_FOUND.as_u16(), instance: None, details: None},
      ApiError::BadRequest(details, code) => ErrorResponse { title: "Bad request".to_string(), status: StatusCode::BAD_REQUEST.as_u16(), instance: None, details: Some(vec![ValidationProblem {field: None, message: details.clone(), error_code: code.clone()}])},
      ApiError::InternalServerError => ErrorResponse { title: "Internal server error".to_string(), status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(), instance: None, details: Some(vec![ValidationProblem {field: None, message: e.to_string(), error_code: 0}])},
      ApiError::ErrorCode(code) => {
        if let Some(errorcode) = dict.get(code){
          ErrorResponse { title: errorcode.message.clone(), status: errorcode.status_code.as_u16(), instance: None, details: Some(vec![ValidationProblem {field: None, message: errorcode.message.clone(), error_code: errorcode.code}])}//(errorcode.status_code, errorcode.code, errorcode.message.to_string())
        } else {
          ErrorResponse { title: "Internal server error".to_string(), status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(), instance: None, details: Some(vec![ValidationProblem {field: None, message: e.to_string(), error_code: 0}])}
        }  
      },
      ApiError::MultipleErrors(errors, field, instance) => {
        let mut validation_problems: Option<Vec<ValidationProblem>> = Some(vec![]);
        let mut status_code: u16 = 0;
        if let Some(error_list) = errors {
          for code in error_list {
            if let Some(errorcode) = dict.get(code){
              status_code = errorcode.status_code.as_u16();
              if let Some(ref mut problems) = validation_problems {
                problems.push(ValidationProblem {field: field.clone(), message: errorcode.message.clone(), error_code: errorcode.code});
              }
            } 
          }
        }
        ErrorResponse { title: e.to_string(), status: status_code, instance: instance.clone(), details: validation_problems}
      }
    }
   } else {
      ErrorResponse { title: "Internal server error".to_string(), status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(), instance: None, details: None}       
   };

   let json = warp::reply::json(&errors);
   let res_status_code: StatusCode;
   match StatusCode::from_u16(errors.status) {
    Ok(status_code)=> {res_status_code = status_code},
    Err(_)=> {res_status_code = StatusCode::INTERNAL_SERVER_ERROR}
   }

   Ok(warp::reply::with_status(json, res_status_code))
}