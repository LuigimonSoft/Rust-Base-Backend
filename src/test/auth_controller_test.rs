use std::sync::Arc;
use warp::Reply;

use crate::controllers::auth_controller::{generate_token, protected_endpoint};
use crate::models::auth_request::AuthRequestDto;
use crate::repositories::{
    credentials_repository::InMemoryCredentialRepository,
    token_repository::InMemoryTokenRepository,
};
use crate::services::auth_service::{AuthService, AuthServiceImpl};

#[tokio::test]
async fn handler_generate_token() {
    let token_repo = InMemoryTokenRepository::new();
    let cred_repo = InMemoryCredentialRepository::new();
    let service = AuthServiceImpl::new(token_repo, cred_repo);
    let request = AuthRequestDto::Client {
        client_id: "client".to_string(),
        client_secret: "secret".to_string(),
    };
    let reply = generate_token(Arc::new(service), request)
        .await
        .unwrap()
        .into_response();
    assert_eq!(reply.status(), 200);
}

#[tokio::test]
async fn handler_generate_token_invalid_credentials() {
    let token_repo = InMemoryTokenRepository::new();
    let cred_repo = InMemoryCredentialRepository::new();
    let service = AuthServiceImpl::new(token_repo, cred_repo);
    let request = AuthRequestDto::Client {
        client_id: "client".to_string(),
        client_secret: "wrong".to_string(),
    };
    let result = generate_token(Arc::new(service), request).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn handler_protected_endpoint() {
    let reply = protected_endpoint().await.unwrap().into_response();
    assert_eq!(reply.status(), 200);
}
