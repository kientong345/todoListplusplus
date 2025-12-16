use axum::{middleware::from_fn_with_state, routing::get, Router};

use crate::{
    app::AppState, controller::task, middleware::auth::auth_middleware, routes::API_PREFIX,
};

pub fn create_route(state: AppState) -> Router {
    Router::new()
        .route(
            &format!(r#"{}/categories/{{:category_id}}/tasks"#, API_PREFIX),
            get(task::get_page).post(task::create),
        )
        .route(
            &format!(
                r#"{}/categories/{{:category_id}}/tasks/{{:task_id}}"#,
                API_PREFIX
            ),
            get(task::find_by_id)
                .delete(task::delete)
                .patch(task::update),
        )
        .layer(from_fn_with_state(state.clone(), auth_middleware))
        .with_state(state)
}
