use axum::routing::{get, post};
use axum::{Json, Router};
use tower_http::services::ServeDir;

use crate::controller;

pub fn static_files() -> Router {
    Router::new().route_service("/", ServeDir::new("html"))
}

pub fn health() -> Router {
    Router::new()
        .route("/healthy", get(|| async { Json("healthy") }))
        .route("/started", get(|| async { Json("started") }))
        .route("/ready", get(|| async { Json("ready") }))
        .route("/live", get(|| async { Json("live") }))
        .route("/ping", get(|| async { Json("pong") }))
}

pub fn cookie() -> Router {
    Router::new()
        .route("/cookies", post(controller::create_cookie))
        .route("/cookies", get(controller::read_cookies))
}
