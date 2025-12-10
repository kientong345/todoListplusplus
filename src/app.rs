use std::sync::Arc;

use axum::Router;

use crate::{
    config::Configuration, database::persistent::PrimaryDatabase, service::auth::AuthService,
};

#[derive(Clone)]
pub struct AppState {
    pub db: PrimaryDatabase,
    pub config: Arc<Configuration>,
    pub auth_service: AuthService,
}

pub async fn create_app(state: AppState) -> Router {
    Router::new()
    // // Admin routes
    // .merge(routes::admin::create_admin_route(state.clone()))
    // // auth routes
    // .merge(routes::auth::create_route(state.clone()))
    // // user routes
    // .merge(routes::user::create_route(state.clone()))
    // .merge(routes::user::create_auth_route(state.clone()))
    // .merge(routes::user::create_admin_route(state.clone()))
    // // category routes
    // .merge(routes::category::create_route(state.clone()))
    // .merge(routes::category::create_admin_route(state.clone()))
    // // quiz routes
    // .merge(routes::quiz::create_route(state.clone()))
    // .merge(routes::quiz::create_auth_route(state.clone()))
    // .merge(routes::quiz::create_owner_route(state.clone()))
    // // default routes
    // .merge(routes::create_default_route())
}
