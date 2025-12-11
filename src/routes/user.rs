use axum::{middleware::from_fn_with_state, routing::get, Router};

use crate::{
    app::AppState,
    controller::user::{get_me, update_me},
    middleware::auth::auth_middleware,
};

pub fn create_auth_route(state: AppState) -> Router {
    Router::new()
        .route("/api/users/me", get(get_me).patch(update_me))
        .layer(from_fn_with_state(state.clone(), auth_middleware))
        .with_state(state)
}
