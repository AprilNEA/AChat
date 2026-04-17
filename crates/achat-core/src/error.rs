use thiserror::Error;

#[derive(Debug, Error)]
pub enum AchatError {
    #[error("Store error: {0}")]
    Store(String),

    #[error("Provider error: {0}")]
    Provider(String),

    #[error("Agent error: {0}")]
    Agent(String),

    #[error("Sync error: {0}")]
    Sync(String),

    #[error("Auth error: {0}")]
    Auth(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
