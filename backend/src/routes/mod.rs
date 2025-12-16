use axum::{routing::get, Router};

pub mod auth;
pub mod category;
pub mod task;
pub mod user;

pub const API_PREFIX: &str = "/api/v1";

pub fn create_default_route() -> Router {
    Router::new().route("/", get(|| async { "Hello from todoList++!" }))
}
