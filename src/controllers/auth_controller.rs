use std::sync::Arc;
use warp::{http::StatusCode, reply::with_status};

use crate::models::auth_request::AuthRequestDto;
use crate::services::auth_service::AuthService;

#[utoipa::path(
    post,
    path = "/api/v1/auth/token",
    tag = "Authentication",
    request_body = crate::models::auth_request::AuthRequestDto,
    responses(
        (status = 200, body = crate::models::token_model::TokenResponseDto),
        (status = 401, body = crate::models::error_response::ErrorResponse),
        (status = 500, body = crate::models::error_response::ErrorResponse)
    )
)]
pub async fn generate_token<S: AuthService + Send + Sync>(
    service: Arc<S>,
    request: AuthRequestDto,
) -> Result<impl warp::Reply, warp::Rejection> {
    match service.generate_token(request).await {
        Ok(token) => Ok(warp::reply::json(&token)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/protected",
    tag = "Protected",
    security(("api_key" = [])),
    responses(
        (status = 200, body = String),
        (status = 401, description = "Unauthorized", body = crate::models::error_response::ErrorResponse)
    )
)]
pub async fn protected_endpoint() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(with_status(
        warp::reply::json(&serde_json::json!({"message": "Top secret"})),
        StatusCode::OK,
    ))
}
