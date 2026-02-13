use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use xtrace::{run_server, ServerConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,tower_http=info,sqlx=warn".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = ServerConfig {
        database_url: std::env::var("DATABASE_URL")
            .map_err(|_| anyhow::anyhow!("missing env DATABASE_URL"))?,
        api_bearer_token: std::env::var("API_BEARER_TOKEN")
            .map_err(|_| anyhow::anyhow!("missing env API_BEARER_TOKEN"))?,
        bind_addr: std::env::var("BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:8742".to_string()),
        default_project_id: std::env::var("DEFAULT_PROJECT_ID")
            .unwrap_or_else(|_| "default".to_string()),
        langfuse_public_key: std::env::var("XTRACE_PUBLIC_KEY")
            .ok()
            .or_else(|| std::env::var("LANGFUSE_PUBLIC_KEY").ok()),
        langfuse_secret_key: std::env::var("XTRACE_SECRET_KEY")
            .ok()
            .or_else(|| std::env::var("LANGFUSE_SECRET_KEY").ok()),
    };

    run_server(config).await
}
