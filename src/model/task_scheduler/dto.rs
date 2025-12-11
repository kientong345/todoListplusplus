use chrono::{DateTime, Duration, Utc};
use serde::Deserialize;

use crate::model::task_scheduler::TaskScheduleParams;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskScheduleDto {
    pub expires_at: String,
    pub cycle_time: Option<String>, // in seconds
}

impl TaskScheduleDto {
    pub fn bind(self, task_id: i32) -> TaskScheduleParams {
        let expires_at: DateTime<Utc> = DateTime::parse_from_rfc3339(&self.expires_at)
            .unwrap()
            .into();

        let reschedule_time = self.cycle_time.and_then(|cycle_seconds_str| {
            // Parse cycle_time string to i64 (seconds)
            let cycle_seconds: i64 = cycle_seconds_str.parse().ok()?;

            // Calculate reschedule time by adding duration to expires_at
            Some(expires_at + Duration::seconds(cycle_seconds))
        });

        TaskScheduleParams {
            task_id,
            expires_at,
            reschedule_time,
        }
    }
}
