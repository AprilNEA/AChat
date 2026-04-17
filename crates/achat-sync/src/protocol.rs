use serde::{Deserialize, Serialize};

/// A single change in the sync changelog.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Change {
    pub table_name: String,
    pub row_id: String,
    pub operation: ChangeOperation,
    pub changed_columns: Option<Vec<String>>,
    pub data: serde_json::Value,
    pub hlc_timestamp: i64,
    pub device_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ChangeOperation {
    Insert,
    Update,
    Delete,
}

/// Request to pull changes from the server.
#[derive(Debug, Serialize, Deserialize)]
pub struct PullRequest {
    pub device_id: String,
    pub last_pull_hlc: i64,
}

/// Response containing changes from the server.
#[derive(Debug, Serialize, Deserialize)]
pub struct PullResponse {
    pub changes: Vec<Change>,
    pub server_hlc: i64,
}

/// Request to push local changes to the server.
#[derive(Debug, Serialize, Deserialize)]
pub struct PushRequest {
    pub device_id: String,
    pub changes: Vec<Change>,
}

/// Response after pushing changes.
#[derive(Debug, Serialize, Deserialize)]
pub struct PushResponse {
    pub applied: usize,
    pub conflicts: Vec<Change>,
}
