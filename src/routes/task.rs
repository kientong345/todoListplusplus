use axum::{middleware::from_fn_with_state, routing::get, Router};

use crate::{app::AppState, controller::task, middleware::auth::auth_middleware};

pub fn create_route(state: AppState) -> Router {
    Router::new()
        .route(
            "/api/categories/{:category_id}/tasks",
            get(task::get_page).post(task::create),
        )
        .route(
            "/api/categories/{:category_id}/tasks/{:task_id}",
            get(task::find_by_id)
                .delete(task::delete)
                .patch(task::update),
        )
        .layer(from_fn_with_state(state.clone(), auth_middleware))
        .with_state(state)
}
