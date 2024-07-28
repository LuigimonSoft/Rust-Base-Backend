use warp::{Filter, Rejection};
use crate::errors::ApiError;
use crate::models::messageModel::CreateMessageModelDto;
use crate::errors::error_codes::ErrorCodes;

pub fn validate_create_message() -> impl Filter<Extract = (CreateMessageModelDto,), Error = Rejection> + Clone {
    warp::body::json()
        .and_then(|body: CreateMessageModelDto| async move {
            match body.content {
                Some(ref s) if s.is_empty() => Err(warp::reject::custom(ApiError::ErrorCode(ErrorCodes::NotEmpty))),
                Some(ref s) if s.len() > 32 => Err(warp::reject::custom(ApiError::ErrorCode(ErrorCodes::MaxSize))),
                Some(ref s) => Ok(body),
                None => Err(warp::reject::custom(ApiError::ErrorCode(ErrorCodes::NotNull)))
            }
        })
}