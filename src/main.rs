use anyhow::Result;
use rust_claw::core::config::Settings;
use rust_claw::interfaces::endpoints::create_router;
use rust_claw::state::AppState;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let settings = Settings::from_env()?;
    init_logging(&settings.log_level);

    let addr = settings.socket_addr()?;
    let state = AppState::new(settings)?;

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = create_router()
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    tracing::info!("rust_claw backend listening on http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// 初始化日志：优先读取 `RUST_LOG`，否则回退到配置中的日志级别。
fn init_logging(log_level: &str) {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(log_level.to_lowercase()));
    tracing_subscriber::fmt().with_env_filter(filter).init();
}
