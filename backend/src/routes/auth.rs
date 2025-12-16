use axum::{routing::post, Router};

use crate::{
    app::AppState,
    controller::auth::{
        handle_google_login, handle_login, handle_logout, handle_refresh, handle_register,
    },
    routes::API_PREFIX,
};

pub fn create_route(state: AppState) -> Router {
    Router::new()
        .route(
            &format!("{}/auth/register", API_PREFIX),
            post(handle_register),
        )
        .route(&format!("{}/auth/login", API_PREFIX), post(handle_login))
        .route(
            &format!("{}/auth/google-login", API_PREFIX),
            post(handle_google_login),
        )
        .route(&format!("{}/auth/logout", API_PREFIX), post(handle_logout))
        .route(
            &format!("{}/auth/refresh", API_PREFIX),
            post(handle_refresh),
        )
        .with_state(state)
}
