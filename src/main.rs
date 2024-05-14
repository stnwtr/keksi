use axum::{serve, Router};
use time::OffsetDateTime;
use tokio::net::TcpListener;
use tokio::signal;
use tower_http::cors::CorsLayer;

mod controller;
mod entity;
mod router;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("0.0.0.0:8080").await?;

    println!(
        "[{}] Listening on http://0.0.0.0:8080/",
        OffsetDateTime::now_utc()
    );

    let router = Router::new()
        .merge(router::static_files())
        .nest("/api/v1", router::health())
        .nest("/api/v1", router::cookie())
        .layer(CorsLayer::permissive());

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
