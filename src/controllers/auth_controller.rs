use warp::reply::with_status;
use crate::models::auth_model::{LoginRequest, TokenResponse, Claims};
use crate::middleware::auth_middleware::create_jwt;
use crate::models::error_response::ErrorResponse;

const VALID_USERNAME: &str = "admin";
const VALID_PASSWORD: &str = "password123";

#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    tag = "Authentication",
    responses(
        (status = 200, body = TokenResponse),
        (status = 401, description="Unauthorized", body = ErrorResponse),
        (status = 400, description="Bad request", body = ErrorResponse),
        (status = 500, body = ErrorResponse)
    ),
    request_body(content = LoginRequest, description = "Login credentials", content_type = "application/json")
)]
pub async fn handle_login(
    login_request: LoginRequest,
) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    // Simple credential validation (in production, use proper authentication)
    if login_request.username == VALID_USERNAME && login_request.password == VALID_PASSWORD {
        match create_jwt(&login_request.username) {
            Ok(token) => {
                let response = TokenResponse {
                    access_token: token,
                    token_type: "Bearer".to_string(),
                    expires_in: 86400, // 24 hours in seconds
                };
                Ok(Box::new(warp::reply::json(&response)))
            }
            Err(_) => {
                let error_response = ErrorResponse {
                    instance: Some("/api/v1/auth/login".to_string()),
                    title: "Internal Server Error".to_string(),
                    detail: "Failed to generate token".to_string(),
                    status: 500,
                    details: None,
                };
                Ok(Box::new(with_status(
                    warp::reply::json(&error_response),
                    warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                )))
            }
        }
    } else {
        let error_response = ErrorResponse {
            instance: Some("/api/v1/auth/login".to_string()),
            title: "Unauthorized".to_string(),
            detail: "Invalid username or password".to_string(),
            status: 401,
            details: None,
        };
        Ok(Box::new(with_status(
            warp::reply::json(&error_response),
            warp::http::StatusCode::UNAUTHORIZED,
        )))
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/test/protected",
    tag = "Test",
    responses(
        (status = 200, description = "Success", body = String),
        (status = 401, description="Unauthorized", body = ErrorResponse),
        (status = 500, body = ErrorResponse)
    ),
    security(
        ("Bearer" = [])
    )
)]
pub async fn handle_protected_test(
    claims: Claims,
) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let response = serde_json::json!({
        "message": "This is a protected endpoint",
        "user": claims.sub,
        "timestamp": chrono::Utc::now().to_rfc3339()
    });
    Ok(Box::new(warp::reply::json(&response)))
}