use axum::{serve, Router};
use tokio::net::TcpListener;
use tokio::signal;
use tower_http::cors::CorsLayer;

use crate::state::KeksiState;

mod controller;
mod entity;
mod router;
mod state;

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let key = std::env::var("KEKSI_KEY")?;
    let hostname = std::env::var("KEKSI_ADDRESS")?;
    let port = std::env::var("KEKSI_PORT")?.parse()?;

    let state = KeksiState::new(key);

    let router = Router::new()
        .merge(router::static_files())
        .nest("/api/v1", router::health())
        .nest("/api/v1", router::cookie())
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = TcpListener::bind((hostname, port)).await?;

    serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
