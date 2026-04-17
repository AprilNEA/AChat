use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub id: String,
    pub name: String,
    pub provider_type: ProviderType,
    pub base_url: Option<String>,
    pub api_key: Option<String>,
    pub is_enabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderType {
    OpenAi,
    Anthropic,
    Ollama,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: String,
    pub provider_id: String,
    pub name: String,
    pub display_name: Option<String>,
    pub supports_tools: bool,
    pub supports_vision: bool,
}
