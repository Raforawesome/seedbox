use axum::{Router, routing::get};
use tracing::Level;

const ADDR: &str = "0.0.0.0:31667";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tracing_sub = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .with_line_number(true)
        .with_thread_ids(false)
        .with_target(true)
        .finish();
    tracing::subscriber::set_global_default(tracing_sub)?;

    let app = Router::new().route("/", get(async || "Seedbox is up!"));
    let listener = tokio::net::TcpListener::bind(ADDR).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
