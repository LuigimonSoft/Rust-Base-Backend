use mockall::automock;
use crate::models::messageModel::{CreateMessageModelDto, MessageModel, MessageResponseDto};

#[automock]
pub trait BaseService: Send + Sync {
  fn get_messages(&self) -> Vec<MessageModel>;
  fn create_message(&self, dto: CreateMessageModelDto) -> MessageModel;
  fn search_messages(&self, query: &str) -> Vec<MessageModel>;
}

#[cfg(test)]
mod tests {
  use mockall::predicate::*;
  use super::*;

  #[tokio::test]
  async fn test_get_messages() {
    let message1:String = "Message 1".to_string();
    let message2:String = "Message 2".to_string();

    let message1_expected = message1.clone();
    let message2_expected = message2.clone();

    let mut mock = MockBaseService::new();

    mock.expect_get_messages()
        .times(1)
        .returning(move || {
            vec![
                MessageModel { id: 1, content: message1.clone() },
                MessageModel { id: 2, content: message2.clone() }
            ]
        });

    let messages = mock.get_messages();
    assert_eq!(messages.len(), 2);
    assert_eq!(messages[0].content, message1_expected);
    assert_eq!(messages[1].content, message2_expected);
  }

  #[tokio::test]
  async fn test_create_message() {
    let message: String = "Hello, world!".to_string();
    let message_expected = message.clone();
    let dto = CreateMessageModelDto { content: Some("Hello, world!".to_string()) };

    let mut mock = MockBaseService::new();

     mock
        .expect_create_message()
        .with(mockall::predicate::eq(dto.clone()))
        .times(1)
        .returning(move |_| {
            MessageModel { id: 1, content: message.clone() }
        });

    let message = mock.create_message(CreateMessageModelDto { content: Some("Hello, world!".to_string()) });
    
    assert_eq!(message.id, 1);
    assert_eq!(message.content, message_expected);
  }

  #[tokio::test]
  async fn test_search_messages() {
    let message1:String = "Message 1".to_string();
    let message2:String = "Message 2".to_string();

    let message1_expected = message1.clone();
    let message2_expected = message2.clone();

    let mut mock = MockBaseService::new();

    mock.expect_search_messages()
        .with(eq("Message"))
        .times(1)
        .returning(move |_| {
            vec![
                MessageModel { id: 1, content: message1.clone() },
                MessageModel { id: 2, content: message2.clone() }
            ]
        });

    let messages = mock.search_messages("Message");
    assert_eq!(messages.len(), 2);
    assert_eq!(messages[0].content, message1_expected);
    assert_eq!(messages[1].content, message2_expected);
  }
}