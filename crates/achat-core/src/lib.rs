pub mod error;
pub mod traits;
pub mod types;

pub use error::AchatError;
pub type Result<T> = std::result::Result<T, AchatError>;
