use axum::{middleware::from_fn_with_state, routing::get, Router};

use crate::{
    app::AppState,
    controller::task,
    middleware::{
        auth::auth_middleware,
        ownership::{category_ownership_check, task_ownership_check},
    },
    routes::API_PREFIX,
};

pub fn create_auth_route(state: AppState) -> Router {
    Router::new()
        .route(
            &format!(r#"{}/categories/{{:category_id}}/tasks"#, API_PREFIX),
            get(task::get_page).post(task::create),
        )
        .layer(from_fn_with_state(state.clone(), category_ownership_check))
        .layer(from_fn_with_state(state.clone(), auth_middleware))
        .with_state(state)
}

pub fn create_ownership_route(state: AppState) -> Router {
    Router::new()
        .route(
            &format!(
                r#"{}/categories/{{:category_id}}/tasks/{{:task_id}}"#,
                API_PREFIX
            ),
            get(task::find_by_id)
                .delete(task::delete)
                .patch(task::update),
        )
        .layer(from_fn_with_state(state.clone(), task_ownership_check))
        .layer(from_fn_with_state(state.clone(), auth_middleware))
        .with_state(state)
}
