use serde::Serialize;

#[derive(Serialize, utoipa::ToResponse,utoipa::ToSchema)]
pub struct ErrorResponse {
  pub title:String,
  pub status:u16,
  pub instance: Option<String>,
  pub details: Option<Vec<ValidationProblem>>
}

#[derive(Debug, Serialize, utoipa::ToResponse, utoipa::ToSchema)]
pub struct ValidationProblem {
   pub field: Option<String>,
   pub message: String,
   pub error_code: u16,
}