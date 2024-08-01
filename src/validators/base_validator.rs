use warp::{Filter, Rejection};
use crate::models::messageModel::CreateMessageModelDto;
use crate::errors::error_codes::ErrorCodes;
use crate::middleware::validator::Rule;

pub fn validate_create_message() -> impl Filter<Extract = (CreateMessageModelDto,), Error = Rejection> + Clone {
    warp::body::json()
        .and_then(|body: CreateMessageModelDto| async move {
          match Rule::new(body.content.as_ref())
                .not_null()
                .with_error_code(ErrorCodes::NotNull)
                .not_empty()
                .with_error_code(ErrorCodes::NotEmpty)
                .max_length(32)
                .with_error_code(ErrorCodes::MaxSize)
                .validate() {
                    Ok(_) => Ok(body),
                    Err(err) => Err(err),
                }
        })
}