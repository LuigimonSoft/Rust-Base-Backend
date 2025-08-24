use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[async_trait]
pub trait CredentialRepository: Send + Sync {
    async fn validate_user(&self, username: &str, password: &str) -> bool;
    async fn validate_client(&self, client_id: &str, client_secret: &str) -> bool;
}

pub struct InMemoryCredentialRepository {
    users: Arc<Mutex<HashMap<String, String>>>,
    clients: Arc<Mutex<HashMap<String, String>>>,
}

impl InMemoryCredentialRepository {
    pub fn new() -> Self {
        let mut users = HashMap::new();
        users.insert("admin".to_string(), "password".to_string());
        let mut clients = HashMap::new();
        clients.insert("client".to_string(), "secret".to_string());
        Self {
            users: Arc::new(Mutex::new(users)),
            clients: Arc::new(Mutex::new(clients)),
        }
    }
}

#[async_trait]
impl CredentialRepository for InMemoryCredentialRepository {
    async fn validate_user(&self, username: &str, password: &str) -> bool {
        self.users
            .lock()
            .unwrap()
            .get(username)
            .map(|p| p == password)
            .unwrap_or(false)
    }

    async fn validate_client(&self, client_id: &str, client_secret: &str) -> bool {
        self.clients
            .lock()
            .unwrap()
            .get(client_id)
            .map(|s| s == client_secret)
            .unwrap_or(false)
    }
}
