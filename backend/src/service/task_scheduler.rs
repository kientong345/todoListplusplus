use std::{collections::HashMap, sync::Arc};

use chrono::{DateTime, Duration, Utc};
use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    Mutex, RwLock,
};
use uuid::Uuid;

use crate::{
    database::persistent::PrimaryDatabase,
    model::task::TaskDatabase,
    service::{error::ServiceError, message_client::MessageClient},
    utils::{pg_interval_to_time, sleep_until_dt},
};

const MINIMUM_CYCLE_S: i64 = 10;

#[derive(Debug, Clone)]
struct ScheduledInfo {
    expires_at: DateTime<Utc>,
    cycle_time: Option<Duration>,
    notify_time: Option<DateTime<Utc>>,
}

impl TryFrom<TaskDatabase> for ScheduledInfo {
    type Error = String;
    fn try_from(task: TaskDatabase) -> Result<Self, Self::Error> {
        let cycle_time = if let Some(interval) = task.cycle_time {
            Some(pg_interval_to_time(interval))
        } else {
            None
        };
        let notify_time = if let Some(time) = task.notify_time {
            Some(time)
        } else {
            None
        };
        let expires_at = task.expires_at.ok_or("Task expires at is not set")?;
        Ok(Self {
            expires_at,
            cycle_time,
            notify_time,
        })
    }
}

#[derive(Debug, Clone)]
enum ScheduleEventType {
    Expiration,
    Notification,
}

#[derive(Debug, Clone)]
struct ScheduleEvent {
    task_id: Uuid,
    time: DateTime<Utc>,
    r#type: ScheduleEventType,
}

#[derive(Debug, Clone)]
pub enum UpdateEventType {
    UpdateExpiration(DateTime<Utc>), // action unavailable yet
    Unschedule, // scheduled_tasks must have expire_time - action unavailable yet
    UpdateCycleTime(Duration), // action unavailable yet
    Uncycle,    // action unavailable yet
    UpdateNotifyTime(DateTime<Utc>), // action unavailable yet
    UnNotify,   // action unavailable yet
    NewScheduledTask {
        expires_at: DateTime<Utc>,
        cycle_time: Option<Duration>,
        notify_time: Option<DateTime<Utc>>,
    },
}

#[derive(Debug, Clone)]
pub struct UpdateEvent {
    pub task_id: Uuid,
    pub r#type: UpdateEventType,
}

pub struct SchedulerService {
    db: PrimaryDatabase,
    scheduled_taskmap: Arc<RwLock<HashMap<Uuid, ScheduledInfo>>>,

    tx: Sender<UpdateEvent>,
    rx: Mutex<Receiver<UpdateEvent>>,
    // message_client: Arc<MessageClient>,
}

impl SchedulerService {
    pub async fn init(
        db: PrimaryDatabase,
        // message_client: Arc<MessageClient>,
    ) -> Result<Self, ServiceError> {
        let scheduled_taskmap = Arc::new(RwLock::new(HashMap::new()));
        let (tx, rx) = mpsc::channel(999);
        let mut scheduler_service = Self {
            db,
            scheduled_taskmap,
            tx,
            rx: Mutex::new(rx),
            // message_client,
        };
        scheduler_service.load_scheduled_tasks().await?;
        Ok(scheduler_service)
    }

    async fn load_scheduled_tasks(&mut self) -> Result<(), ServiceError> {
        let mut transaction = self.db.start_transaction().await?;
        let tasks = TaskDatabase::get_scheduled_tasks(&mut transaction).await?;
        transaction.commit().await?;

        for task in tasks {
            if let Ok(scheduled_info) = ScheduledInfo::try_from(task.clone()) {
                self.scheduled_taskmap
                    .write()
                    .await
                    .insert(task.id, scheduled_info);
            }
        }
        Ok(())
    }

    pub async fn start(self: Arc<Self>) -> Result<(), ServiceError> {
        tokio::spawn(async move { self.clone().schedule_loop().await });
        Ok(())
    }

    pub async fn trigger_schedule_update_event(
        &self,
        update_params: UpdateEvent,
    ) -> Result<(), ServiceError> {
        Ok(self
            .tx
            .send(update_params)
            .await
            .expect("cannot trigger schedule update"))
    }

    async fn get_next_event(&self) -> Option<ScheduleEvent> {
        let scheduled_taskmap = self.scheduled_taskmap.read().await;
        let mut next_event = ScheduleEvent {
            task_id: Uuid::nil(),
            time: DateTime::<Utc>::MAX_UTC,
            r#type: ScheduleEventType::Expiration,
        };
        scheduled_taskmap.iter().for_each(|(task_id, sch)| {
            if sch.notify_time.is_some() && sch.notify_time.unwrap() < sch.expires_at {
                if sch.notify_time.unwrap() < next_event.time {
                    next_event.task_id = *task_id;
                    next_event.time = sch.notify_time.unwrap();
                    next_event.r#type = ScheduleEventType::Notification;
                }
            } else {
                if sch.expires_at < next_event.time {
                    next_event.task_id = *task_id;
                    next_event.time = sch.expires_at;
                    next_event.r#type = ScheduleEventType::Expiration;
                }
            };
        });
        Some(next_event)
    }

    async fn handle_schedule_event(
        self: Arc<Self>,
        event: ScheduleEvent,
    ) -> Result<(), ServiceError> {
        match event.r#type {
            ScheduleEventType::Expiration => {
                let mut scheduled_taskmap = self.scheduled_taskmap.write().await;
                let schedule_info = scheduled_taskmap.get_mut(&event.task_id).unwrap();

                if schedule_info.cycle_time.is_none() {
                    self.clone()
                        .scheduled_taskmap
                        .write()
                        .await
                        .remove(&event.task_id);
                } else {
                    let mut transaction = self.db.start_transaction().await?;
                    TaskDatabase::spawn_new_link(event.task_id, &mut *transaction).await?;
                    transaction.commit().await?;
                    schedule_info.expires_at = Utc::now() + schedule_info.cycle_time.unwrap();
                }
            }
            ScheduleEventType::Notification => {
                // feature unavailable
            }
        }
        Ok(())
    }

    async fn handle_update_event(
        self: Arc<Self>,
        event: UpdateEvent,
    ) -> Result<Option<ScheduledInfo>, ServiceError> {
        let new_update = match event.r#type {
            UpdateEventType::UpdateCycleTime(new_cycle_time) => {
                // event unavailable
                None
            }
            UpdateEventType::UpdateExpiration(new_expire_time) => {
                // event unavailable
                None
            }
            UpdateEventType::UpdateNotifyTime(new_notify_time) => {
                // event unavailable
                None
            }
            UpdateEventType::UnNotify => {
                // event unavailable
                None
            }
            UpdateEventType::Uncycle => {
                // event unavailable
                None
            }
            UpdateEventType::Unschedule => {
                // event unavailable
                None
            }
            UpdateEventType::NewScheduledTask {
                expires_at,
                cycle_time,
                notify_time,
            } => {
                let mut scheduled_taskmap = self.scheduled_taskmap.write().await;
                scheduled_taskmap.insert(
                    event.task_id,
                    ScheduledInfo {
                        expires_at,
                        cycle_time,
                        notify_time,
                    },
                )
            }
        };
        Ok(new_update)
    }

    async fn schedule_loop(self: Arc<Self>) -> Result<(), ServiceError> {
        let mut next_wakeup = Utc::now();
        let mut rx = self.rx.lock().await;

        loop {
            tokio::select! {
                _ = sleep_until_dt(next_wakeup) => {
                    next_wakeup = loop {
                        if let Some(next_event) = self.clone().get_next_event().await {
                            if next_event.time > Utc::now() {
                                break std::cmp::min(next_event.time, Utc::now() + Duration::seconds(MINIMUM_CYCLE_S));
                            }
                            self.clone().handle_schedule_event(next_event).await?;
                        } else {
                            break Utc::now() + Duration::seconds(MINIMUM_CYCLE_S);
                        }
                    }
                }
                Some(update_params) = rx.recv() => {
                    if let Some(new_update) = self.clone().handle_update_event(update_params).await? {
                        let nearest_event = if new_update.notify_time.is_some() && new_update.notify_time.unwrap() < new_update.expires_at {
                            new_update.notify_time.unwrap()
                        } else {
                            new_update.expires_at
                        };
                        if nearest_event < next_wakeup {
                            next_wakeup = nearest_event;
                        }
                    }
                }
            }
        }
    }
}
