use warp::{Filter, Rejection};
use crate::models::messageModel::CreateMessageModelDto;
use crate::errors::error_codes::ErrorCodes;
use crate::middleware::validator::Rule;

pub fn validate_create_message() -> impl Filter<Extract = (CreateMessageModelDto,), Error = Rejection> + Clone {
    warp::body::json()
        .and_then(|body: CreateMessageModelDto| async move {
          Rule::new(body.content.as_ref())
            .with_error_code(ErrorCodes::NotNull).not_null().validate()
            .and_then(|_| Rule::new(body.content.as_ref()).with_error_code(ErrorCodes::NotEmpty).not_empty().validate())
            .and_then(|_| Rule::new(body.content.as_ref()).with_error_code(ErrorCodes::MaxSize).max_length(32).validate())
            .map(|_| body)
            .map_err(|err| warp::reject::custom(err))


            /*match body.content {
                Some(ref s) if s.is_empty() => Err(warp::reject::custom(ApiError::ErrorCode(ErrorCodes::NotEmpty))),
                Some(ref s) if s.len() > 32 => Err(warp::reject::custom(ApiError::ErrorCode(ErrorCodes::MaxSize))),
                Some(ref s) => Ok(body),
                None => Err(warp::reject::custom(ApiError::ErrorCode(ErrorCodes::NotNull)))
            } */
        })
}