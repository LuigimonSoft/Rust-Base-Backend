use async_trait::async_trait;
use crate::models::messageModel::{MessageModel, CreateMessageModelDto};
use crate::repositories::base_Repository::BaseRepository;

#[async_trait]
pub trait BaseService: Send + Sync {
  async fn get_messages(&self) -> Vec<MessageModel>;
  async fn create_message(&self, dto: CreateMessageModelDto) -> MessageModel;
  async fn search_messages(&self, query: &str) -> Vec<MessageModel>;
}

pub struct BaseServiceImpl<R: BaseRepository> {
  repository: R
}

impl<R: BaseRepository> BaseServiceImpl<R> {
  pub fn new(repository: R) -> Self {
    Self { repository }
  }
}

#[async_trait]
impl<R: BaseRepository + Send + Sync> BaseService for BaseServiceImpl<R> {
  async fn get_messages(&self) -> Vec<MessageModel> {
    self.repository.get_messages().await
  }

  async fn create_message(&self, dto:CreateMessageModelDto) -> MessageModel {
    self.repository.add_message(dto.content).await
  }

  async fn search_messages(&self, query: &str) -> Vec<MessageModel> {
    self.repository.search_messages(query).await
  }
}