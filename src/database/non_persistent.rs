use deadpool_redis::{Config, Connection, CreatePoolError, Pool, PoolError, Runtime};

use crate::config::database::CacheConfig;

#[derive(Clone)]
pub struct SecondaryDatabase {
    connection_pool: Pool,
}

impl SecondaryDatabase {
    pub fn init(config: &CacheConfig) -> Result<Self, CreatePoolError> {
        let connection_pool =
            Config::from_url(config.cache_url.as_str()).create_pool(Some(Runtime::Tokio1))?;

        Ok(Self { connection_pool })
    }

    pub async fn get_connection(&self) -> Result<Connection, PoolError> {
        self.connection_pool.get().await
    }
}
