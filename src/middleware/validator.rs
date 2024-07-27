use warp::{Filter, Rejection};
use crate::errors::ApiError;
use crate::models::messageModel::CreateMessageModelDto;

pub fn validate_create_message() -> impl Filter<Extract = (CreateMessageModelDto,), Error = Rejection> + Clone {
    warp::body::json()
        .and_then(|body: CreateMessageModelDto| async move {
            match body.content {
                Some(ref s) if s.is_empty() => Err(warp::reject::custom(ApiError::BadRequest("Content must not be empty".to_string(), 1002))),
                Some(ref s) if s.len() > 32 => Err(warp::reject::custom(ApiError::BadRequest("Content must be maximum size 32 ".to_string(), 1003))),
                Some(ref s) => Ok(body),
                None => Err(warp::reject::custom(ApiError::BadRequest("Content must not be null".to_string(), 1001)))
            }
        })
}