use serde::Serialize;

#[derive(Debug, Serialize, utoipa::ToSchema, utoipa::ToResponse)]
pub struct TokenResponseDto {
    pub token: String,
}
