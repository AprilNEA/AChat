use std::collections::HashMap;
use std::sync::Arc;

use achat_core::traits::ChatProvider;

/// Registry that maps model identifiers (e.g. "openai/gpt-4") to provider instances.
pub struct ProviderRegistry {
    providers: HashMap<String, Arc<dyn ChatProvider>>,
}

impl ProviderRegistry {
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
        }
    }

    pub fn register(&mut self, model_id: impl Into<String>, provider: Arc<dyn ChatProvider>) {
        self.providers.insert(model_id.into(), provider);
    }

    pub fn get(&self, model_id: &str) -> Option<Arc<dyn ChatProvider>> {
        self.providers.get(model_id).cloned()
    }

    pub fn list_models(&self) -> Vec<&str> {
        self.providers.keys().map(|s| s.as_str()).collect()
    }
}

impl Default for ProviderRegistry {
    fn default() -> Self {
        Self::new()
    }
}
