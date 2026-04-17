use std::sync::Arc;

use achat_agent::AgentRuntime;
use achat_provider::ProviderRegistry;

/// Shared application state for the Axum server.
#[allow(dead_code)]
pub struct AppState {
    // pub db: sqlx::PgPool,
    pub agent_runtime: Arc<AgentRuntime>,
    pub provider_registry: Arc<ProviderRegistry>,
}
