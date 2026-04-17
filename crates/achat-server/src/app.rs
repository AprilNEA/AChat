use axum::{Router, routing::get};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

pub fn create_router() -> Router {
    Router::new()
        .route("/health", get(health))
        // .nest("/api/v1/chat", routes::chat::router())
        // .nest("/api/v1/agent", routes::agent::router())
        // .nest("/api/v1/sync", routes::sync::router())
        // .nest("/api/v1/compute", routes::compute::router())
        // .nest("/api/v1/admin", routes::admin::router())
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
}

async fn health() -> &'static str {
    "ok"
}
