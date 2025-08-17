use crate::server::run_server;
use tokio::time::{sleep, Duration};
use tokio::sync::oneshot;
use crate::config;
use crate::errors::error_codes::ErrorCodes;
use dotenv::dotenv;
use serde_json::Value;
use std::sync::Arc;

async fn spawn_server() -> oneshot::Sender<()> {
    let (shutdown, _base) = run_server().await;
    // give the server time to start
    sleep(Duration::from_millis(100)).await;
    shutdown
}

fn build_address(endpoint: &str) -> String {
    dotenv().ok();
    let config = Arc::new(config::Config::from_env());
    format!("http://localhost:{}/{}/{}", config.port, config.api_base, endpoint)
}

fn build_static_address(path: &str) -> String {
    dotenv().ok();
    let config = Arc::new(config::Config::from_env());
    format!("http://localhost:{}/{}", config.port, path)
}

#[tokio::test]
async fn test_get_messages_valid() {
    let shutdown = spawn_server().await;

    let address = build_address("messages");
    let client = reqwest::Client::new();
    let response = client.get(address).send().await.unwrap();

    assert_eq!(response.status(), 200);

    let body: Value = response.json().await.unwrap();
    assert!(body.is_array());

    let _ = shutdown.send(());
}

#[tokio::test]
async fn test_create_message_valid() {
    let shutdown = spawn_server().await;
    let text_expected = "Hello, world!";

    let address = build_address("messages");
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
    let shutdown = spawn_server().await;
    let text_expected = "Text to search";

    let address = build_address("messages");
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
    let shutdown = spawn_server().await;

    let address = build_address("messages");
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
    let shutdown = spawn_server().await;

    let address = build_address("messages");
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
    assert_eq!(body["details"][0]["error_code"], ErrorCodes::NotEmpty as u16);

    let _ = shutdown.send(());
}

#[tokio::test]
async fn test_create_message_invalid_maxsize() {
    let shutdown = spawn_server().await;

    let address = build_address("messages");
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
    let shutdown = spawn_server().await;

    let client = reqwest::Client::new();

    let address = build_static_address("index.html");
    let response = client.get(address).send().await.unwrap();
    assert_eq!(response.status(), 200);
    let body = response.text().await.unwrap();
    assert!(body.contains("Static file served"));

    let _ = shutdown.send(());
}
