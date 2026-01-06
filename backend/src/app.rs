use std::sync::Arc;

use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    config::Configuration,
    infrastructures::cache::LocalCache,
    infrastructures::persistent::PrimaryDatabase,
    openapi::ApiDoc,
    routes,
    service::{auth::AuthService, task_scheduler::SchedulerService},
};

#[derive(Clone)]
pub struct AppState {
    pub db: PrimaryDatabase,
    pub config: Arc<Configuration>,
    pub cache: Arc<LocalCache>,
    pub auth_service: AuthService,
    pub scheduler_service: Arc<SchedulerService>,
    // pub email_client: Arc<MessageClient>,
}

pub async fn create_app(state: AppState) -> Router {
    Router::new()
        // auth routes
        .merge(routes::auth::create_route(state.clone()))
        // user routes
        .merge(routes::user::create_auth_route(state.clone()))
        // category routes
        .merge(routes::category::create_auth_route(state.clone()))
        .merge(routes::category::create_ownership_route(state.clone()))
        // task routes
        .merge(routes::task::create_auth_route(state.clone()))
        .merge(routes::task::create_ownership_route(state.clone()))
        // default routes
        .merge(routes::create_default_route())
        // health check routes
        .merge(routes::create_health_check_route())
        // openapi routes
        .merge(SwaggerUi::new("/swagger/api-docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
}
