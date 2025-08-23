use crate::repositories::token_repository::InMemoryTokenRepository;
use crate::services::auth_service::{AuthService, AuthServiceImpl};

#[tokio::test]
async fn generate_and_validate_token() {
    let repo = InMemoryTokenRepository::new();
    let service = AuthServiceImpl::new(repo);
    let token = service.generate_token().await.token;
    assert!(service.validate_token(&token).await);
}
