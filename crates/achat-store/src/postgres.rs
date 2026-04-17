//! PostgreSQL storage implementation for cloud server.
//!
//! Uses sqlx with connection pooling and compile-time query checking.

#[cfg(feature = "postgres")]
pub struct PgStore {
    // pool: sqlx::PgPool,
}

#[cfg(feature = "postgres")]
impl PgStore {
    /// Create a new PostgreSQL store from a connection pool.
    pub fn new() -> Self {
        // TODO: Accept PgPool, run migrations
        Self {}
    }
}
