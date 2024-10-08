use crate::server::run_server;
use tokio::task;
use tokio::time::{sleep, Duration};

struct TestServer {
    shutdown: Option<tokio::sync::oneshot::Sender<()>>,
    base_url: String,
}

impl TestServer {
    async fn new() -> Self {
        dotenv::dotenv().ok();
        let (shutdown, base_url) = run_server().await;

        let _keep_alive = task::spawn(async move {
            sleep(Duration::from_secs(360)).await;
        });
        TestServer {
            shutdown: Some(shutdown),
            base_url,
        }
    }
}

impl Drop for TestServer {
    fn drop(&mut self) {
        if let Some(shutdown) = self.shutdown.take() {
            let _ = shutdown.send(());
        }
    }
}

#[cfg(test)]
mod test {
    use super::TestServer;
    use crate::config;
    use crate::errors::error_codes::ErrorCodes;
    use dotenv::dotenv;
    use once_cell::sync::Lazy;
    use reqwest;
    use serde_json::Value;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    static SERVER: Lazy<Mutex<Option<Arc<TestServer>>>> = Lazy::new(|| Mutex::new(None));

    async fn initialize_server() -> Arc<TestServer> {
        let mut server_lock = SERVER.lock().await;

        if server_lock.is_none() {
            let server = TestServer::new().await;
            *server_lock = Some(Arc::new(server));
        }

        Arc::clone(server_lock.as_ref().unwrap())
    }

    fn build_address(endpoint: &str) -> String {
      dotenv().ok();
      let config = Arc::new(config::Config::from_env());
      format!("http://localhost:{}/{}/{}", config.port, config.api_base, endpoint)
    }

    #[tokio::test]
    async fn test_get_messages_valid() {
        let server = initialize_server().await;

        let address = build_address("messages");
        let client = reqwest::Client::new();
        let response = client.get(address).send().await.unwrap();

        assert_eq!(response.status(), 200);

        let body: Value = response.json().await.unwrap();

        assert!(body.is_array());
    }

    #[tokio::test]
    async fn test_create_message_valid() {
        let server = initialize_server().await;
        let text_expected = "Hello, world!";

        dotenv().ok();
        let config = Arc::new(config::Config::from_env());
        let address = build_address("messages");
        let client = reqwest::Client::new();
        let response = client
            .post(address)
            .json(&serde_json::json!({
              "content": text_expected
            }))
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), 201);

        let body: Value = response.json().await.unwrap();

        assert_eq!(body["content"], text_expected);
    }

    #[tokio::test]
    async fn test_search_message_valid() {
        let server = initialize_server().await;

        let text_expected = "Text to search";

        dotenv().ok();
        let config = Arc::new(config::Config::from_env());
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
            .get(format!("{}/{}", address.clone(), "Text"))
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), 200);

        let body: Value = response.json().await.unwrap();

        assert_eq!(body[0]["content"], text_expected);
    }

    #[tokio::test]
    async fn test_create_message_invalid_null() {
        let server = initialize_server().await;

        dotenv().ok();
        let config = Arc::new(config::Config::from_env());
        let address = build_address("messages");
        let client = reqwest::Client::new();

        // null test
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
    }

    #[tokio::test]
    async fn test_create_message_invalid_empty() {
        let server = initialize_server().await;

        dotenv().ok();
        let config = Arc::new(config::Config::from_env());
        let address = build_address("messages");
        let client = reqwest::Client::new();

        // Empty test
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
    }

    #[tokio::test]
    async fn test_create_message_invalid_maxsize() {
        let server = initialize_server().await;

        dotenv().ok();
        let config = Arc::new(config::Config::from_env());
        let address = build_address("messages");
        let client = reqwest::Client::new();

        // More than 32 characters test
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
    }
}
