use std::sync::Arc;

use achat_provider::ProviderRegistry;

use crate::tools::ToolRegistry;

/// The agent runtime orchestrates LLM calls, tool execution, and memory.
pub struct AgentRuntime {
    pub provider_registry: Arc<ProviderRegistry>,
    pub tool_registry: ToolRegistry,
}

impl AgentRuntime {
    pub fn new(provider_registry: Arc<ProviderRegistry>, tool_registry: ToolRegistry) -> Self {
        Self {
            provider_registry,
            tool_registry,
        }
    }
}
