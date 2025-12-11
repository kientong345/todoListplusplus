use axum::{routing::post, Router};

use crate::{
    app::AppState,
    controller::auth::{
        handle_google_login, handle_login, handle_logout, handle_refresh, handle_register,
    },
};

pub fn create_route(state: AppState) -> Router {
    Router::new()
        .route("/api/auth/register", post(handle_register))
        .route("/api/auth/login", post(handle_login))
        .route("/api/auth/google-login", post(handle_google_login))
        .route("/api/auth/logout", post(handle_logout))
        .route("/api/auth/refresh", post(handle_refresh))
        .with_state(state)
}
