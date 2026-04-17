use async_trait::async_trait;

use crate::types::*;
use crate::Result;

#[async_trait]
pub trait ConversationStore: Send + Sync {
    async fn create_conversation(&self, title: &str, owner_id: UserId) -> Result<Conversation>;
    async fn get_conversation(&self, id: ConversationId) -> Result<Option<Conversation>>;
    async fn list_conversations(&self, owner_id: UserId, limit: u32, offset: u32) -> Result<Vec<Conversation>>;
    async fn update_conversation_title(&self, id: ConversationId, title: &str) -> Result<()>;
    async fn delete_conversation(&self, id: ConversationId) -> Result<()>;
}

#[async_trait]
pub trait MessageStore: Send + Sync {
    async fn create_message(&self, conversation_id: ConversationId, role: Role, content: MessageContent) -> Result<Message>;
    async fn get_messages(&self, conversation_id: ConversationId) -> Result<Vec<Message>>;
    async fn get_message(&self, id: MessageId) -> Result<Option<Message>>;
    async fn delete_message(&self, id: MessageId) -> Result<()>;
}
