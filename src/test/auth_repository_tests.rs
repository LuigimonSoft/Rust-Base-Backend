#![allow(dead_code, unused_imports, unused_variables)]

use crate::repositories::credentials_repository::{
    CredentialRepository, InMemoryCredentialRepository,
};
use crate::repositories::token_repository::{InMemoryTokenRepository, TokenRepository};
use chrono::{Duration, Utc};
use hex;
use sha2::{Digest, Sha256};

#[tokio::test]
async fn store_and_validate_token() {
    let repo = InMemoryTokenRepository::new();
    let token = "test";
    let hashed = Sha256::digest(token.as_bytes());
    let hashed_hex = hex::encode(hashed);
    repo.store_token(hashed_hex.clone(), Utc::now() + Duration::minutes(5))
        .await;
    assert!(repo.is_valid(&hashed_hex).await);
}

#[tokio::test]
async fn validate_user_and_client_credentials() {
    let repo = InMemoryCredentialRepository::new();
    assert!(repo.validate_user("admin", "password").await);
    assert!(!repo.validate_user("admin", "wrong").await);
    assert!(repo.validate_client("client", "secret").await);
    assert!(!repo.validate_client("client", "nope").await);
}
