use warp::{Filter, Rejection};
use crate::models::messageModel::CreateMessageModelDto;
use crate::errors::error_codes::ErrorCodes;
use crate::middleware::validator::Rule;

pub fn validate_create_message(path:Option<String>) -> impl Filter<Extract = (CreateMessageModelDto,), Error = Rejection> + Clone {
    let path = warp::any().map(move || path.clone());
    warp::body::json()
        .and(path)
        .and_then(|body: CreateMessageModelDto, path: Option<String>| async move {
          let content_validation = match Rule::new(body.content.as_ref(),Some("content".to_string()), path)
                .not_null()
                .with_error_code(ErrorCodes::NotNull)
                .not_empty()
                .with_error_code(ErrorCodes::NotEmpty)
                .max_length(32)
                .with_error_code(ErrorCodes::MaxSize)
                .validate() {
                    Ok(_) => Ok(body),
                    Err(err) => Err(err)
                };

                content_validation
                    .map(|body| body)
                    
        })
}