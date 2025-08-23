use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::sync::{Arc, Mutex};

struct TokenEntry {
    hashed: String,
    expires_at: DateTime<Utc>,
}

#[async_trait]
pub trait TokenRepository: Send + Sync {
    async fn store_token(&self, hashed_token: String, expires_at: DateTime<Utc>);
    async fn is_valid(&self, hashed_token: &str) -> bool;
}

pub struct InMemoryTokenRepository {
    tokens: Arc<Mutex<Vec<TokenEntry>>>,
}

impl InMemoryTokenRepository {
    pub fn new() -> Self {
        Self {
            tokens: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[async_trait]
impl TokenRepository for InMemoryTokenRepository {
    async fn store_token(&self, hashed_token: String, expires_at: DateTime<Utc>) {
        let mut tokens = self.tokens.lock().unwrap();
        tokens.push(TokenEntry {
            hashed: hashed_token,
            expires_at,
        });
    }

    async fn is_valid(&self, hashed_token: &str) -> bool {
        let mut tokens = self.tokens.lock().unwrap();
        let now = Utc::now();
        tokens.retain(|t| t.expires_at > now);
        tokens.iter().any(|t| t.hashed == hashed_token)
    }
}
