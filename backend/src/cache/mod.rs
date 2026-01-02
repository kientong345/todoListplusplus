pub mod local;
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

pub const DEFAULT_TTL_SECONDS: u64 = 999;

#[async_trait]
pub trait Caching: Send + Sync {
    async fn get<T: DeserializeOwned + Send>(&self, key: &str) -> Result<Option<T>, String>;

    async fn set<T: Serialize + Send + Sync>(
        &self,
        key: &str,
        value: &T,
        ttl_seconds: u64,
    ) -> Result<(), String>;

    async fn delete(&self, key: &str) -> Result<(), String>;

    async fn delete_prefix(&self, prefix: &str) -> Result<(), String>;

    async fn clear(&self) -> Result<(), String>;
}
