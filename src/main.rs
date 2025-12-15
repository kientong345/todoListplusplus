use std::{net::SocketAddr, sync::Arc};

use todo_list::{
    app::{self, AppState},
    config::Configuration,
    database::persistent::PrimaryDatabase,
    service::{auth::AuthService, email_client::EmailClient, task_scheduler::SchedulerService},
    utils::get_runtime,
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    // Load configuration
    let config = Arc::new(Configuration::get());

    // Start server
    #[cfg(feature = "local")]
    let address = SocketAddr::from(([127, 0, 0, 1], config.app_config.port));

    #[cfg(not(feature = "local"))]
    let address = SocketAddr::from(([0, 0, 0, 0], config.app_config.port));

    let listener = TcpListener::bind(address)
        .await
        .expect("cannot bind address");

    // Initialize application state
    let db = PrimaryDatabase::init(&config.db_config).await;

    let auth_service = AuthService::new(config.auth_config.clone());

    let scheduler_runtime = get_runtime(
        config.app_config.scheduler_threads as usize,
        "todoListplusplus-tasks-scheduler",
    )
    .expect("cannot build todoListplusplus-tasks-scheduler runtime");

    let email_client = Arc::new(EmailClient::new());

    let scheduler_service = Arc::new(SchedulerService::new(
        scheduler_runtime,
        email_client.clone(),
    ));

    let app_state = AppState {
        db,
        config,
        auth_service,
        scheduler_service,
        email_client,
    };

    // Create app
    let app = app::create_app(app_state).await;

    // Serve app
    axum::serve(listener, app)
        .await
        .expect("cannot serving app");
}
