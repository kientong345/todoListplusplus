pub mod local;
use async_trait::async_trait;
use std::time::Duration;

#[async_trait]
pub trait CacheInterface: Send + Sync {
    type Error: Send + Sync + From<String>;

    async fn get_value(&self, key: &str) -> Result<Option<serde_json::Value>, Self::Error>;

    async fn set_value(
        &self,
        key: &str,
        value: serde_json::Value,
        ttl: Option<Duration>,
    ) -> Result<(), Self::Error>;

    async fn delete(&self, key: &str) -> Result<(), Self::Error>;

    async fn exists(&self, key: &str) -> Result<bool, Self::Error>;
}
