use axum::{middleware::from_fn_with_state, routing::get, Router};

use crate::{
    app::AppState, controller::category, middleware::auth::auth_middleware, routes::API_PREFIX,
};

pub fn create_route(state: AppState) -> Router {
    Router::new()
        .route(
            &format!("{}/categories", API_PREFIX),
            get(category::get_page).post(category::create),
        )
        .route(
            &format!("{}/categories/{{:id}}", API_PREFIX),
            get(category::find_by_id)
                .delete(category::delete)
                .patch(category::update),
        )
        .layer(from_fn_with_state(state.clone(), auth_middleware))
        .with_state(state)
}
