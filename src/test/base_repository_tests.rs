#![allow(dead_code, unused_imports, unused_variables)]
#[cfg(test)]
mod tests {
    use crate::repositories::base_repository::InMemoryBaseRepository;
    use crate::repositories::base_repository::BaseRepository;

    #[tokio::test] 
    async fn test_add_message() {
        let repository = InMemoryBaseRepository::new();
        let content = String::from("Hello, world!");

        let message = repository.add_message(content.clone()).await;

        assert_eq!(message.content, content);
        assert_eq!(message.id, 1);

        let messages = repository.get_messages().await;

        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].content, content);
    }

    #[tokio::test]
    async fn test_get_messages() {
        let repo = InMemoryBaseRepository::new();

        repo.add_message(String::from("Message 1")).await;
        repo.add_message(String::from("Message 2")).await;

        let messages = repo.get_messages().await;

        assert_eq!(messages.len(), 2);
        assert_eq!(messages[0].content, "Message 1");
        assert_eq!(messages[1].content, "Message 2");
    }

    #[tokio::test]
    async fn test_search_messages() {
        let repo = InMemoryBaseRepository::new();
        repo.add_message(String::from("Hello, world!")).await;
        repo.add_message(String::from("Hello, Rust!")).await;
        repo.add_message(String::from("Goodbye, world!")).await;

        let results = repo.search_messages("Hello").await;
        assert_eq!(results.len(), 2);
        assert!(results.iter().any(|m| m.content == "Hello, world!"));
        assert!(results.iter().any(|m| m.content == "Hello, Rust!"));
    }
}
