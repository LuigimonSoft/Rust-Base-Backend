use async_trait::async_trait;
use base64::{engine::general_purpose, Engine as _};
use chrono::{Duration, Utc};
use hex;
use rand::RngCore;
use sha2::{Digest, Sha256};

use crate::errors::ApiError;
use crate::models::{auth_request::AuthRequestDto, token_model::TokenResponseDto};
use crate::repositories::{credentials_repository::CredentialRepository, token_repository::TokenRepository};

#[async_trait]
pub trait AuthService: Send + Sync {
    async fn generate_token(&self, request: AuthRequestDto) -> Result<TokenResponseDto, ApiError>;
    async fn validate_token(&self, token: &str) -> bool;
}

pub struct AuthServiceImpl<R: TokenRepository, C: CredentialRepository> {
    token_repository: R,
    credential_repository: C,
    ttl_minutes: i64,
}

impl<R: TokenRepository, C: CredentialRepository> AuthServiceImpl<R, C> {
    pub fn new(token_repository: R, credential_repository: C) -> Self {
        Self {
            token_repository,
            credential_repository,
            ttl_minutes: 60,
        }
    }
}

#[async_trait]
impl<R: TokenRepository + Send + Sync, C: CredentialRepository + Send + Sync> AuthService
    for AuthServiceImpl<R, C>
{
    async fn generate_token(
        &self,
        request: AuthRequestDto,
    ) -> Result<TokenResponseDto, ApiError> {
        let valid = match request {
            AuthRequestDto::User { username, password } => {
                self.credential_repository
                    .validate_user(&username, &password)
                    .await
            }
            AuthRequestDto::Client {
                client_id,
                client_secret,
            } => {
                self.credential_repository
                    .validate_client(&client_id, &client_secret)
                    .await
            }
        };

        if !valid {
            return Err(ApiError::Unauthorized);
        }

        let mut bytes = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut bytes);
        let token = general_purpose::URL_SAFE_NO_PAD.encode(&bytes);
        let hashed = Sha256::digest(token.as_bytes());
        let hashed_hex = hex::encode(hashed);
        let expires_at = Utc::now() + Duration::minutes(self.ttl_minutes);
        self.token_repository
            .store_token(hashed_hex, expires_at)
            .await;
        Ok(TokenResponseDto { token })
    }

    async fn validate_token(&self, token: &str) -> bool {
        let hashed = Sha256::digest(token.as_bytes());
        let hashed_hex = hex::encode(hashed);
        self.token_repository.is_valid(&hashed_hex).await
    }
}
