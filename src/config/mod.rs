use serde::{Deserialize, Serialize};

use crate::config::{
    app::AppConfig,
    auth::AuthConfig,
    database::{CacheConfig, DatabaseConfig},
    oauth::OAuthConfig,
    scheduler::SchedulerConfig,
};

pub mod app;
pub mod auth;
pub mod database;
pub mod oauth;
pub mod scheduler;

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Configuration {
    pub app_config: AppConfig,
    pub db_config: DatabaseConfig,
    pub cache_config: CacheConfig,
    pub auth_config: AuthConfig,
    pub oauth_config: OAuthConfig,
    pub scheduler_config: SchedulerConfig,
}

impl Configuration {
    pub fn get() -> Configuration {
        Configuration {
            app_config: AppConfig::get(),
            db_config: DatabaseConfig::get(),
            cache_config: CacheConfig::get(),
            auth_config: AuthConfig::get(),
            oauth_config: OAuthConfig::get(),
            scheduler_config: SchedulerConfig::get(),
        }
    }
}
