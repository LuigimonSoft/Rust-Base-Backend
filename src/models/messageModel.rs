use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessageModel {
  pub id: usize,
  pub content: String
}

#[derive(Debug, Deserialize)]
pub struct CreateMessageModelDto{
  pub content:String
}

#[derive(Debug, Serialize)]
pub struct MessageResponseDto {
  pub id: usize,
  pub content: String
}