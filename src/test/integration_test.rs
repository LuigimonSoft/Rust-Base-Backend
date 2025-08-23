use crate::config;
use crate::errors::error_codes::ErrorCodes;
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

fn build_static_address(base: &str, path: &str) -> String {
    format!("{}/{}", base, path)
}

#[tokio::test]
async fn test_get_messages_valid() {
    let (shutdown, base) = spawn_server().await;

    let address = build_address(&base, "messages");
    let client = reqwest::Client::new();
    let response = client.get(address).send().await.unwrap();

    assert_eq!(response.status(), 200);

    let body: Value = response.json().await.unwrap();
    assert!(body.is_array());

    let _ = shutdown.send(());
}

#[tokio::test]
async fn test_create_message_valid() {
    let (shutdown, base) = spawn_server().await;
    let text_expected = "Hello, world!";
    let address = build_address(&base, "messages");
    let client = reqwest::Client::new();
    let response = client
        .post(address.clone())
        .json(&serde_json::json!({
            "content": text_expected
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 201);

    let body: Value = response.json().await.unwrap();
    assert_eq!(body["content"], text_expected);

    let _ = shutdown.send(());
}

#[tokio::test]
async fn test_search_message_valid() {
    let (shutdown, base) = spawn_server().await;
    let text_expected = "Text to search";
    let address = build_address(&base, "messages");
    let client = reqwest::Client::new();
    let response = client
        .post(address.clone())
        .json(&serde_json::json!({
            "content": text_expected
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 201);

    let response = client
        .get(format!("{}/{}", address, "Text"))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);

    let body: Value = response.json().await.unwrap();
    assert_eq!(body[0]["content"], text_expected);

    let _ = shutdown.send(());
}

#[tokio::test]
async fn test_create_message_invalid_null() {
    let (shutdown, base) = spawn_server().await;

    let address = build_address(&base, "messages");
    let client = reqwest::Client::new();

    let response = client
        .post(address.clone())
        .json(&serde_json::json!({
            "content": null
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), 400);
    let body: Value = response.json().await.unwrap();
    assert_eq!(body["details"][0]["error_code"], ErrorCodes::NotNull as u16);

    let _ = shutdown.send(());
}

#[tokio::test]
async fn test_create_message_invalid_empty() {
    let (shutdown, base) = spawn_server().await;

    let address = build_address(&base, "messages");
    let client = reqwest::Client::new();

    let response = client
        .post(address.clone())
        .json(&serde_json::json!({
            "content": ""
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), 400);
    let body: Value = response.json().await.unwrap();
    assert_eq!(
        body["details"][0]["error_code"],
        ErrorCodes::NotEmpty as u16
    );

    let _ = shutdown.send(());
}

#[tokio::test]
async fn test_create_message_invalid_maxsize() {
    let (shutdown, base) = spawn_server().await;

    let address = build_address(&base, "messages");
    let client = reqwest::Client::new();

    let response = client
        .post(address.clone())
        .json(&serde_json::json!({
            "content": "This is a very long text that is more than 32 characters"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), 400);
    let body: Value = response.json().await.unwrap();
    assert_eq!(body["details"][0]["error_code"], ErrorCodes::MaxSize as u16);

    let _ = shutdown.send(());
}

#[tokio::test]
async fn test_static_file_serving() {
    let (shutdown, base) = spawn_server().await;

    let client = reqwest::Client::new();

    let address = build_static_address(&base, "index.html");
    let response = client.get(address).send().await.unwrap();
    assert_eq!(response.status(), 200);
    let body = response.text().await.unwrap();
    assert!(body.contains("Static file served"));

    let _ = shutdown.send(());
}

#[tokio::test]
async fn test_auth_token_and_protected() {
    let (shutdown, base) = spawn_server().await;
    let client = reqwest::Client::new();

    let token_addr = build_address(&base, "auth/token");
    let token_resp = client.post(token_addr).send().await.unwrap();
    assert_eq!(token_resp.status(), 200);
    let body: Value = token_resp.json().await.unwrap();
    let token = body["token"].as_str().unwrap();

    let protected_addr = build_address(&base, "protected");
    let unauth = client.get(protected_addr.clone()).send().await.unwrap();
    assert_eq!(unauth.status(), 401);

    let auth = client
        .get(protected_addr)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .unwrap();
    assert_eq!(auth.status(), 200);

    let _ = shutdown.send(());
}
