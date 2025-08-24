use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, ToSchema)]
#[serde(tag = "grant_type", rename_all = "snake_case")]
pub enum AuthRequestDto {
    User { username: String, password: String },
    Client { client_id: String, client_secret: String },
}
