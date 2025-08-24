use warp::{http::StatusCode, reply::with_status};

use crate::models::error_response::ErrorResponse;

#[utoipa::path(
    get,
    path = "/api/v1/protected",
    tag = "Protected",
    security(("api_key" = [])),
    responses(
        (status = 200, body = String),
        (status = 401, description = "Unauthorized", body = ErrorResponse)
    )
)]
pub async fn protected_endpoint() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(with_status(
        warp::reply::json(&serde_json::json!({"message": "Top secret"})),
        StatusCode::OK,
    ))
}
