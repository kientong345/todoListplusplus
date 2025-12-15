use std::{collections::HashMap, sync::Arc};

use chrono::{DateTime, Duration, Utc};
use tokio::sync::RwLock;

use crate::{
    database::persistent::PrimaryDatabase,
    service::{email_client::EmailClient, error::ServiceError},
};

#[derive(Debug, Clone)]
struct ScheduledInfo {
    expires_at: Option<DateTime<Utc>>,
    cycle_time: Option<Duration>,
    pre_notify_time: Option<Duration>,
}

pub struct SchedulerService {
    db: PrimaryDatabase,
    scheduled_taskmap: Arc<RwLock<HashMap<i32, ScheduledInfo>>>,

    runtime: tokio::runtime::Runtime,
    email_client: Arc<EmailClient>,
}

impl SchedulerService {
    pub fn init(
        db: PrimaryDatabase,
        runtime: tokio::runtime::Runtime,
        email_client: Arc<EmailClient>,
    ) -> Self {
        let scheduled_taskmap = Arc::new(RwLock::new(HashMap::new()));
        Self {
            db,
            scheduled_taskmap,
            runtime,
            email_client,
        }
    }

    pub async fn with_scheduled_tasks(mut self) -> Result<Self, ServiceError> {
        Ok(self)
    }

    pub async fn start(&self) -> Result<(), ServiceError> {
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
