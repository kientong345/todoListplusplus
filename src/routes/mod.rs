use axum::{routing::get, Router};

pub mod auth;
pub mod category;
pub mod task;
pub mod user;

pub fn create_default_route() -> Router {
    Router::new().route("/", get(|| async { "Hello from todoList++!" }))
}
