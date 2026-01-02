use async_trait::async_trait;
use moka::future::Cache;
use std::time::Duration;

use crate::cache::CacheInterface;

pub struct LocalCache {
    cache: Cache<String, serde_json::Value>,
}

impl LocalCache {
    pub fn new() -> Self {
        Self {
            cache: Cache::new(100),
        }
    }
}

#[async_trait]
impl CacheInterface for LocalCache {
    type Error = String;

    async fn get_value(&self, key: &str) -> Result<Option<serde_json::Value>, Self::Error> {
        Ok(self.cache.get(key).await)
    }

    async fn set_value(
        &self,
        key: &str,
        value: serde_json::Value,
        _ttl: Option<Duration>,
    ) -> Result<(), Self::Error> {
        self.cache.insert(key.to_string(), value).await;
        Ok(())
    }

    async fn delete(&self, key: &str) -> Result<(), Self::Error> {
        self.cache.remove(key).await;
        Ok(())
    }

    async fn exists(&self, key: &str) -> Result<bool, Self::Error> {
        Ok(self.cache.contains_key(key))
    }
}
