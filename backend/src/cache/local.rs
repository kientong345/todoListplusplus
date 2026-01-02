use async_trait::async_trait;
use moka::future::Cache;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::json;

use crate::cache::Caching;

pub struct LocalCache {
    inner: Cache<String, serde_json::Value>,
}

impl LocalCache {
    pub fn new() -> Self {
        Self {
            inner: Cache::builder().max_capacity(100).build(),
        }
    }
}

#[async_trait]
impl Caching for LocalCache {
    async fn get<T: DeserializeOwned + Send>(&self, key: &str) -> Result<Option<T>, String> {
        if let Some(json_str) = self.inner.get(key).await {
            let val = serde_json::from_str(&json_str.to_string()).map_err(|e| e.to_string())?;
            println!("use cache for: {key}");
            Ok(Some(val))
        } else {
            Ok(None)
        }
    }

    async fn set<T: Serialize + Send + Sync>(
        &self,
        key: &str,
        value: &T,
        _ttl_seconds: u64,
    ) -> Result<(), String> {
        let json_str = json!(value);
        self.inner.insert(key.to_string(), json_str).await;
        Ok(())
    }

    async fn delete(&self, key: &str) -> Result<(), String> {
        self.inner.invalidate(key).await;
        Ok(())
    }

    async fn delete_prefix(&self, key_prefix: &str) -> Result<(), String> {
        for (key, _val) in self.inner.iter() {
            if key.starts_with(key_prefix) {
                self.delete(&key).await?;
            }
        }
        Ok(())
    }

    async fn clear(&self) -> Result<(), String> {
        self.inner.invalidate_all();
        Ok(())
    }
}
