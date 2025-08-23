use std::sync::Arc;
use warp::Reply;

use crate::controllers::auth_controller::{generate_token, protected_endpoint};
use crate::repositories::token_repository::InMemoryTokenRepository;
use crate::services::auth_service::{AuthService, AuthServiceImpl};

#[tokio::test]
async fn handler_generate_token() {
    let repo = InMemoryTokenRepository::new();
    let service = AuthServiceImpl::new(repo);
    let reply = generate_token(Arc::new(service))
        .await
        .unwrap()
        .into_response();
    assert_eq!(reply.status(), 200);
}

#[tokio::test]
async fn handler_protected_endpoint() {
    let reply = protected_endpoint().await.unwrap().into_response();
    assert_eq!(reply.status(), 200);
}
