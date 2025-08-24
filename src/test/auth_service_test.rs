use crate::models::auth_request::AuthRequestDto;
use crate::repositories::credentials_repository::InMemoryCredentialRepository;
use crate::repositories::token_repository::InMemoryTokenRepository;
use crate::services::auth_service::{AuthService, AuthServiceImpl};

#[tokio::test]
async fn generate_and_validate_token() {
    let token_repo = InMemoryTokenRepository::new();
    let cred_repo = InMemoryCredentialRepository::new();
    let service = AuthServiceImpl::new(token_repo, cred_repo);
    let request = AuthRequestDto::User {
        username: "admin".to_string(),
        password: "password".to_string(),
    };
    let token = service.generate_token(request).await.unwrap().token;
    assert!(service.validate_token(&token).await);
}

#[tokio::test]
async fn reject_invalid_credentials() {
    let token_repo = InMemoryTokenRepository::new();
    let cred_repo = InMemoryCredentialRepository::new();
    let service = AuthServiceImpl::new(token_repo, cred_repo);
    let request = AuthRequestDto::User {
        username: "admin".to_string(),
        password: "wrong".to_string(),
    };
    assert!(service.generate_token(request).await.is_err());
}
