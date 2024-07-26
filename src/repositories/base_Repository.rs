use async_trait::async_trait;
use crate::models::messageModel::MessageModel;
use std::sync::{Arc, Mutex};

#[async_trait]
pub trait BaseRepository: Send + Sync {
  async fn get_messages(&self) -> Vec<MessageModel>;
  async fn add_message(&self, content: String) -> MessageModel;
  async fn search_messages(&self, query: &str) -> Vec<MessageModel>;
}

pub struct InMemoryBaseRepository {
  messages: Arc<Mutex<Vec<MessageModel>>>
}

impl InMemoryBaseRepository {
  pub fn new() -> Self {
    Self {
      messages: Arc::new(Mutex::new(Vec::new()))
    }
  }
}

#[async_trait]
impl BaseRepository for InMemoryBaseRepository {
  async fn get_messages(&self) -> Vec<MessageModel> {
    self.messages.lock().unwrap().clone()
  }

  async fn add_message(&self, content: String) -> MessageModel {
    let mut messages = self.messages.lock().unwrap();
    let id = messages.len() + 1;
    let message = MessageModel { id, content };

    messages.push(message.clone());
    message
  }

  async fn search_messages(&self, query: &str) -> Vec<MessageModel> {
    self.messages
        .lock()
        .unwrap()
        .iter()
        .filter(|m| m.content.contains(query))
        .cloned()
        .collect()
  }
} 

