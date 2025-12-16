use axum::{middleware::from_fn_with_state, routing::get, Router};

use crate::{
    app::AppState,
    controller::user::{get_me, update_me},
    middleware::auth::auth_middleware,
    routes::API_PREFIX,
};

pub fn create_auth_route(state: AppState) -> Router {
    Router::new()
        .route(
            &format!(r#"{}/users/me"#, API_PREFIX),
            get(get_me).patch(update_me),
        )
        .layer(from_fn_with_state(state.clone(), auth_middleware))
        .with_state(state)
}
