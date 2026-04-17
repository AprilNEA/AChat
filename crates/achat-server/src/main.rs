use tracing_subscriber::EnvFilter;

mod app;
mod state;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let app = app::create_router();

    let addr = "0.0.0.0:3000";
    tracing::info!("AChat server listening on {addr}");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
