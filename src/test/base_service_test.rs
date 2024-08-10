use mockall::automock;
use crate::models::messageModel::MessageModel;



#[automock]
pub trait BaseRepository {
    fn add_message(&self, content: String) -> MessageModel;
    fn get_messages(&self) -> Vec<MessageModel>;
    fn search_messages(&self, query: &str) -> Vec<MessageModel>;
}

#[cfg(test)]
mod tests {
  use mockall::predicate::*;
  use super::*;

  #[tokio::test]
  async fn test_add_message() {
    let message_expected: String = "Hello, world!".to_string();

    let mut mock = MockBaseRepository::new();

    mock.expect_add_message()
        .with(eq("Hello, world!".to_string()))
        .times(1)
        .returning(|message_expected| {
            MessageModel { id: 1, content: message_expected.clone() }
        });

    let message = mock.add_message("Hello, world!".to_string());
    assert_eq!(message.id, 1);
    assert_eq!(message.content, message_expected);
  }

  #[tokio::test]
  async fn test_get_messages() {
    let message1:String = "Message 1".to_string();
    let message2:String = "Message 2".to_string();

   let message1_expected = message1.clone();
   let message2_expected = message2.clone();
 

    let mut mock = MockBaseRepository::new();

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
  async fn test_search_messages() {
    let message1:String = "Hello, world!".to_string();
    let message2:String = "Hello, Rust!".to_string();

    let message1_expected = message1.clone();
    let message2_expected = message2.clone();

    let mut mock = MockBaseRepository::new();

    mock.expect_search_messages()
        .with(eq("Hello"))
        .times(1)
        .returning(move |_| {
            vec![
                MessageModel { id: 1, content: message1.clone() },
                MessageModel { id: 2, content: message2.clone() }
            ]
        });

    let results = mock.search_messages("Hello");
    assert_eq!(results.len(), 2);
    assert!(results.iter().any(|m| m.content == message1_expected));
    assert!(results.iter().any(|m| m.content == message2_expected));
  }
}