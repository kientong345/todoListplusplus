use axum::{middleware::from_fn_with_state, routing::get, Router};

use crate::{app::AppState, controller::category, middleware::auth::auth_middleware};

pub fn create_route(state: AppState) -> Router {
    Router::new()
        .route(
            "/api/categories",
            get(category::get_page).post(category::create),
        )
        .route(
            "/api/categories/{:id}",
            get(category::find_by_id)
                .delete(category::delete)
                .patch(category::update),
        )
        .layer(from_fn_with_state(state.clone(), auth_middleware))
        .with_state(state)
}
