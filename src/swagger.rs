use utoipa::OpenApi;
use crate::models::messageModel::{CreateMessageModelDto, MessageResponseDto};
use crate::models::error_response::{ErrorResponse, ValidationProblem};
use utoipauto::utoipauto;

use warp::{
    http::Uri,
    hyper::{Response, StatusCode},
    path::{FullPath, Tail}, Rejection, Reply,
};
use std::str::FromStr;
use std::sync::Arc;
use utoipa_swagger_ui::Config;

#[utoipauto(
    paths = "./src/controllers"
)]
#[derive(OpenApi)]
#[openapi(
    info(
        title = "Rust Base Backend API ",
        version = "1.0.0",
        description = "This is a simple Rust Base Backend API",
    ),
    components(
        schemas(
            CreateMessageModelDto,
            MessageResponseDto,
            ErrorResponse,
            ValidationProblem
        )
    )
)]
pub struct ApiDoc;


pub async fn serve_swagger(
    full_path: FullPath,
    tail: Tail,
    config: Arc<Config<'static>>,
) -> Result<Box<dyn Reply + 'static>, Rejection> {
    let api_base_path = std::env::var("API_BASE").expect("API_BASE must be set");

    if full_path.as_str() == format!("/{}/swagger-ui", api_base_path) {
        return Ok(Box::new(warp::redirect::found(Uri::from_str(
            format!("/{}/swagger-ui", api_base_path.clone()).as_str()
        ).unwrap())));
    }

    let path = tail.as_str();
    match utoipa_swagger_ui::serve(path, config) {
        Ok(file) => {
            if let Some(file) = file {
                Ok(Box::new(
                    Response::builder()
                        .header("Content-Type", file.content_type)
                        .body(file.bytes),
                ))
            } else {
                Ok(Box::new(StatusCode::NOT_FOUND))
            }
        }
        Err(error) => Ok(Box::new(
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(error.to_string()),
        )),
    }
}
