use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::UserId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ConversationId(pub Uuid);

impl ConversationId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: ConversationId,
    pub title: String,
    pub owner_id: UserId,
    pub team_id: Option<String>,
    pub model_id: Option<String>,
    pub is_shared: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub hlc_timestamp: i64,
}
