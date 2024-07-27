use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessageModel {
  pub id: usize,
  pub content: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateMessageModelDto{
  pub content:Option<String>
}

#[derive(Debug, Serialize)]
pub struct MessageResponseDto {
  pub id: usize,
  pub content: String
}