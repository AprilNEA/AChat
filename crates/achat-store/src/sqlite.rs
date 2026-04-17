//! SQLite storage implementation for desktop/local use.
//!
//! Uses rusqlite with WAL mode for concurrent reads.
//! Wrapped in `spawn_blocking` for async compatibility.

#[cfg(feature = "sqlite")]
pub struct SqliteStore {
    // conn: rusqlite::Connection,
}

#[cfg(feature = "sqlite")]
impl SqliteStore {
    /// Open or create a SQLite database at the given path.
    pub fn open(_path: &str) -> achat_core::Result<Self> {
        // TODO: Initialize connection with WAL mode, create tables
        Ok(Self {})
    }
}
