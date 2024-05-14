use axum::{Json, Router};
use axum::routing::get;
use tower_http::services::ServeDir;

use crate::state::KeksiState;

pub fn static_files() -> Router<KeksiState> {
    Router::new()
        .route_service("/", ServeDir::new("html"))
}

pub fn health() -> Router<KeksiState> {
    Router::new()
        .route("/healthy", get(|| async { Json("healthy") }))
        .route("/started", get(|| async { Json("started") }))
        .route("/ready", get(|| async { Json("ready") }))
        .route("/live", get(|| async { Json("live") }))
        .route("/ping", get(|| async { Json("pong") }))
}
