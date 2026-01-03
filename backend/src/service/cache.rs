use moka::future::Cache as MokaCache;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::json;

use crate::service::error::ServiceError;

pub const DEFAULT_TTL_SECONDS: u64 = 999;

pub struct LocalCache {
    inner: MokaCache<String, serde_json::Value>,
}

impl LocalCache {
    pub fn new() -> Self {
        Self {
            inner: MokaCache::builder().max_capacity(100).build(),
        }
    }

    pub async fn get<T: DeserializeOwned + Send>(
        &self,
        key: &str,
    ) -> Result<Option<T>, ServiceError> {
        if let Some(json_str) = self.inner.get(key).await {
            let val = serde_json::from_str(&json_str.to_string())
                .map_err(|e| ServiceError::CacheError(e.to_string()))?;
            Ok(Some(val))
        } else {
            Ok(None)
        }
    }

    pub async fn set<T: Serialize + Send + Sync>(
        &self,
        key: &str,
        value: &T,
        _ttl_seconds: u64,
    ) -> Result<(), ServiceError> {
        let json_str = json!(value);
        self.inner.insert(key.to_string(), json_str).await;
        Ok(())
    }

    pub async fn delete(&self, key: &str) -> Result<(), ServiceError> {
        self.inner.invalidate(key).await;
        Ok(())
    }

    pub async fn delete_prefix(&self, key_prefix: &str) -> Result<(), ServiceError> {
        for (key, _val) in self.inner.iter() {
            if key.starts_with(key_prefix) {
                self.delete(&key).await?;
            }
        }
        Ok(())
    }

    pub async fn clear(&self) -> Result<(), ServiceError> {
        self.inner.invalidate_all();
        Ok(())
    }
}
