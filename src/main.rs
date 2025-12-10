use std::{net::SocketAddr, sync::Arc};

use todo_list::{
    app::{self, AppState},
    config::Configuration,
    database::persistent::PrimaryDatabase,
    service::auth::AuthService,
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

    let app_state = AppState {
        db,
        config,
        auth_service,
    };

    // Create app
    let app = app::create_app(app_state).await;

    // Serve app
    axum::serve(listener, app)
        .await
        .expect("cannot serving app");
}
