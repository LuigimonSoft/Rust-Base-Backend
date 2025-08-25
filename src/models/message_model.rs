use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessageModel {
  pub id: usize,
  pub content: String
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, utoipa::ToSchema)]
pub struct CreateMessageModelDto{
  pub content:Option<String>
}

#[derive(Debug, Serialize, utoipa::ToSchema, utoipa::ToResponse)]
pub struct MessageResponseDto {
  pub id: usize,
  pub content: String
}