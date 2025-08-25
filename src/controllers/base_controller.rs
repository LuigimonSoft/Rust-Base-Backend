use warp::reply::with_status;

use crate::models::message_model::{CreateMessageModelDto, MessageResponseDto};
use crate::services::base_service::BaseService;
use std::sync::Arc;
#[allow(unused_imports)]
use crate::models::error_response::ErrorResponse;

#[utoipa::path(
    get,
    path = "/api/v1/messages",
    tag = "Get all messages",
    responses(
        (status = 200, body = Vec<MessageResponseDto>),
        (status = 400, description="Bad request", body = ErrorResponse),
        (status = 500, body = ErrorResponse)
    )
)]
pub async fn handle_get_messages<S: BaseService + Send + Sync>(
    service: Arc<S>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let messages = service.get_messages().await;
    let response: Vec<MessageResponseDto> = messages
        .into_iter()
        .map(|m| MessageResponseDto {
            id: m.id,
            content: m.content,
        })
        .collect();
    Ok(warp::reply::json(&response))
}

#[utoipa::path(
    post,
    path = "/api/v1/messages",
    tag = "Create a message",
    responses(
        (status = 201, body = MessageResponseDto),
        (status = 400, description="Bad request", body = ErrorResponse),
        (status = 500, body = ErrorResponse)
    ),
    request_body(content = CreateMessageModelDto, description = "Message to create", content_type = "application/json")
    
)]
pub async fn handle_create_message<S: BaseService + Send + Sync>(
    dto: CreateMessageModelDto,
    service: Arc<S>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let message = service.create_message(dto).await;
    let response = MessageResponseDto {
        id: message.id,
        content: message.content,
    };
    Ok(with_status(warp::reply::json(&response), warp::http::StatusCode::CREATED))
}

#[utoipa::path(
    get,
    path = "/api/v1/messages/{message}",
    tag = "Search messages",
    responses(
        (status = 200, body = Vec<MessageResponseDto>),
        (status = 400, description="Bad request", body = ErrorResponse),
        (status = 500, body = ErrorResponse)
    ),
    params(
        ("message"= String, description = "Query to search for")
    )
)]
pub async fn handle_search_messages<S: BaseService + Send + Sync>(
    message: String,
    service: Arc<S>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let messages = service.search_messages(&message).await;
    let response: Vec<MessageResponseDto> = messages
        .into_iter()
        .map(|m| MessageResponseDto {
            id: m.id,
            content: m.content,
        })
        .collect();
    Ok(warp::reply::json(&response))
}