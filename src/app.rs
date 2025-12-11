use std::sync::Arc;

use axum::Router;

use crate::{
    config::Configuration,
    database::persistent::PrimaryDatabase,
    routes,
    service::{auth::AuthService, task_scheduler::SchedulerService},
};

#[derive(Clone)]
pub struct AppState {
    pub db: PrimaryDatabase,
    pub config: Arc<Configuration>,
    pub auth_service: AuthService,
    pub scheduler_service: SchedulerService,
}

pub async fn create_app(state: AppState) -> Router {
    Router::new()
        // auth routes
        .merge(routes::auth::create_route(state.clone()))
        // user routes
        .merge(routes::user::create_auth_route(state.clone()))
        // category routes
        .merge(routes::category::create_route(state.clone()))
        // default routes
        .merge(routes::create_default_route())
}
