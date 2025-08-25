use std::sync::Arc;

#[allow(unused_imports)]
use crate::models::{
    auth_request::AuthRequestDto, error_response::ErrorResponse, token_model::TokenResponseDto,
};
use crate::services::auth_service::AuthService;

#[utoipa::path(
    post,
    path = "/api/v1/auth/token",
    tag = "Authentication",
    request_body(
        content = AuthRequestDto,
        description = "User/password or client credentials used to request a token",
        content_type = "application/json"
    ),
    responses(
        (status = 200, description = "Token generated", body = TokenResponseDto),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
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
