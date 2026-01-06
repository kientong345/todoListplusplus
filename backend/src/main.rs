use std::{net::SocketAddr, sync::Arc};

use todo_list_plusplus::{
    app::{self, AppState},
    config::Configuration,
    infrastructures::{cache::LocalCache, persistent::PrimaryDatabase},
    service::{auth::AuthService, task_scheduler::SchedulerService},
};
use tokio::net::TcpListener;
use tower_governor::{
    governor::GovernorConfigBuilder, key_extractor::SmartIpKeyExtractor, GovernorLayer,
};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    // Load configuration
    let config = Arc::new(Configuration::get());

    // Start server
    let address = SocketAddr::from(([0, 0, 0, 0], config.app_config.port));

    let listener = TcpListener::bind(address)
        .await
        .expect("cannot bind address");

    // Initialize application state
    let db = PrimaryDatabase::init(&config.db_config).await;

    let auth_service = AuthService::new(config.auth_config.clone());

    // let email_client = Arc::new(MessageClient::new());

    let cache = Arc::new(LocalCache::new());

    let scheduler_service = Arc::new(
        SchedulerService::init(db.clone(), cache.clone())
            .await
            .expect("cannot fetch scheduled tasks from database"),
    );

    scheduler_service
        .clone()
        .start()
        .await
        .expect("cannot start scheduler service");

    let app_state = AppState {
        db,
        config,
        cache,
        auth_service,
        scheduler_service,
        // email_client,
    };

    // test-mode
    let cors_layer = CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_methods(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any);

    // rate limit
    let governor_conf = GovernorConfigBuilder::default()
        .per_second(5)
        .burst_size(20)
        .key_extractor(SmartIpKeyExtractor)
        .finish()
        .expect("cannot create governor config");

    let governor_layer = GovernorLayer::new(governor_conf);

    // Create app
    let app = app::create_app(app_state)
        .await
        .layer(governor_layer)
        .layer(cors_layer);

    // Serve app
    axum::serve(listener, app)
        .await
        .expect("cannot serving app");
}
