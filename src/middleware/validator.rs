use warp::{Filter, Rejection};
use crate::errors::ApiError;
use crate::models::messageModel::CreateMessageModelDto;

pub fn validate_create_message() -> impl Filter<Extract = (CreateMessageModelDto,), Error = Rejection> + Clone {
    warp::body::json()
        .and_then(|body: CreateMessageModelDto| async move {
            if body.content.is_empty() {
                Err(warp::reject::custom(ApiError::BadRequest("Content must not be empty".to_string())))
            } else {
                Ok(body)
            }
        })
}