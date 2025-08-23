use crate::config;
use crate::server::run_server;
use dotenv::dotenv;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::oneshot;
use tokio::time::{sleep, Duration};

async fn spawn_server() -> (oneshot::Sender<()>, String) {
    std::env::set_var("PORT", "0");
    let (shutdown, base) = run_server().await;
    // give the server time to start
    sleep(Duration::from_millis(100)).await;
    (shutdown, base)
}

fn build_address(base: &str, endpoint: &str) -> String {
    dotenv().ok();
    let config = Arc::new(config::Config::from_env());
    format!("{}/{}/{}", base, config.api_base, endpoint)
}

#[tokio::test]
async fn test_login_valid_credentials() {
    let (shutdown, base) = spawn_server().await;
    let address = build_address(&base, "auth/login");
    let client = reqwest::Client::new();
    
    let response = client
        .post(address)
        .json(&serde_json::json!({
            "username": "admin",
            "password": "password123"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);

    let body: Value = response.json().await.unwrap();
    assert!(body["access_token"].is_string());
    assert_eq!(body["token_type"], "Bearer");
    assert_eq!(body["expires_in"], 86400);

    let _ = shutdown.send(());
}

#[tokio::test]
async fn test_login_invalid_credentials() {
    let (shutdown, base) = spawn_server().await;
    let address = build_address(&base, "auth/login");
    let client = reqwest::Client::new();
    
    let response = client
        .post(address)
        .json(&serde_json::json!({
            "username": "admin",
            "password": "wrongpassword"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 401);

    let body: Value = response.json().await.unwrap();
    assert_eq!(body["title"], "Unauthorized");
    assert_eq!(body["detail"], "Invalid username or password");

    let _ = shutdown.send(());
}

#[tokio::test]
async fn test_protected_endpoint_with_valid_token() {
    let (shutdown, base) = spawn_server().await;
    let client = reqwest::Client::new();
    
    // First, get a token
    let login_address = build_address(&base, "auth/login");
    let login_response = client
        .post(login_address)
        .json(&serde_json::json!({
            "username": "admin",
            "password": "password123"
        }))
        .send()
        .await
        .unwrap();

    let login_body: Value = login_response.json().await.unwrap();
    let token = login_body["access_token"].as_str().unwrap();

    // Then, access the protected endpoint
    let protected_address = build_address(&base, "test/protected");
    let protected_response = client
        .get(protected_address)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();

    assert_eq!(protected_response.status(), 200);

    let body: Value = protected_response.json().await.unwrap();
    assert_eq!(body["message"], "This is a protected endpoint");
    assert_eq!(body["user"], "admin");
    assert!(body["timestamp"].is_string());

    let _ = shutdown.send(());
}

#[tokio::test]
async fn test_protected_endpoint_without_token() {
    let (shutdown, base) = spawn_server().await;
    let client = reqwest::Client::new();
    
    let protected_address = build_address(&base, "test/protected");
    let response = client
        .get(protected_address)
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 401);

    let _ = shutdown.send(());
}

#[tokio::test]
async fn test_protected_endpoint_with_invalid_token() {
    let (shutdown, base) = spawn_server().await;
    let client = reqwest::Client::new();
    
    let protected_address = build_address(&base, "test/protected");
    let response = client
        .get(protected_address)
        .header("Authorization", "Bearer invalid-token")
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 401);

    let _ = shutdown.send(());
}