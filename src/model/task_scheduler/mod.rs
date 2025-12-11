use chrono::{DateTime, Utc};

pub mod dto;
pub mod schedule;

#[derive(Debug, Clone)]
pub struct TaskScheduleParams {
    pub task_id: i32,
    pub expires_at: DateTime<Utc>,
    pub reschedule_time: Option<DateTime<Utc>>,
}
