use serde::{Deserialize, Serialize};

/// Events emitted by the agent runtime during execution.
/// These map to SSE events (cloud) or Tauri events (desktop).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AgentEvent {
    Thinking,
    StreamChunk { content: String },
    ToolCallStart { name: String, arguments: serde_json::Value },
    ToolCallEnd { name: String, result: String },
    Completed { message_id: String },
    Error { message: String },
}
