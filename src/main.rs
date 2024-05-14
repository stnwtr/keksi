use axum::{Router, serve};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    let router = Router::new()
        .route_service("/", ServeDir::new("html"));

    serve(TcpListener::bind("127.0.0.1:8080").await?, router).await?;

    Ok(())
}