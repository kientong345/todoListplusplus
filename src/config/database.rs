use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct DatabaseConfig {
    pub database_url: String,
}

impl DatabaseConfig {
    pub fn get() -> DatabaseConfig {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");

        DatabaseConfig { database_url }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct CacheConfig {
    pub cache_url: String,
}

impl CacheConfig {
    pub fn get() -> CacheConfig {
        let cache_url = match std::env::var("CACHE_URL") {
            Ok(url) => url,
            Err(_) => {
                println!("CACHE_URL is not set, using default value");
                "".to_string()
            }
        };

        CacheConfig { cache_url }
    }
}
