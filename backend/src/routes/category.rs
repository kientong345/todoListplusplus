use axum::{middleware::from_fn_with_state, routing::get, Router};

use crate::{
    app::AppState,
    controller::category,
    middleware::{auth::auth_middleware, ownership::category_ownership_check},
    routes::API_PREFIX,
};

pub fn create_auth_route(state: AppState) -> Router {
    Router::new()
        .route(
            &format!("{}/categories", API_PREFIX),
            get(category::get_page).post(category::create),
        )
        .layer(from_fn_with_state(state.clone(), auth_middleware))
        .with_state(state)
}

pub fn create_ownership_route(state: AppState) -> Router {
    Router::new()
        .route(
            &format!("{}/categories/{{:category_id}}", API_PREFIX),
            get(category::find_by_id)
                .delete(category::delete)
                .patch(category::update),
        )
        .layer(from_fn_with_state(state.clone(), category_ownership_check))
        .layer(from_fn_with_state(state.clone(), auth_middleware))
        .with_state(state)
}
