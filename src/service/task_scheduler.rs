use std::{collections::HashMap, sync::Arc};

use chrono::{DateTime, Duration, Utc};
use sqlx::PgConnection;
use tokio::sync::RwLock;

use crate::service::{email_client::EmailClient, error::ServiceError};

#[derive(Debug, Clone)]
struct ScheduledInfo {
    cycle_time: Option<Duration>,
    next_expire_time: Option<DateTime<Utc>>,
    notify_time: Option<DateTime<Utc>>,
}

pub struct SchedulerService {
    scheduled_taskmap: Arc<RwLock<HashMap<i32, ScheduledInfo>>>,

    runtime: tokio::runtime::Runtime,
    email_client: Arc<EmailClient>,
}

impl SchedulerService {
    pub fn new(runtime: tokio::runtime::Runtime, email_client: Arc<EmailClient>) -> Self {
        let scheduled_taskmap = Arc::new(RwLock::new(HashMap::new()));
        Self {
            scheduled_taskmap,
            runtime,
            email_client,
        }
    }

    pub async fn start(&self) -> Result<(), ServiceError> {
        Ok(())
    }

    pub async fn sync_with_db(
        &mut self,
        connection: &mut PgConnection,
    ) -> Result<(), ServiceError> {
        Ok(())
    }

    pub async fn set_expire_datetime(
        &mut self,
        task_id: i32,
        datetime: DateTime<Utc>,
    ) -> Result<(), ServiceError> {
        Ok(())
    }

    pub async fn set_task_notification(
        &mut self,
        task_id: i32,
        email: &str,
        time_before_expiration: Duration,
    ) -> Result<(), ServiceError> {
        Ok(())
    }

    pub async fn set_task_cycle(
        &mut self,
        task_id: i32,
        cycle_time: Duration,
        next_expires_time: DateTime<Utc>,
    ) -> Result<(), ServiceError> {
        Ok(())
    }
}
