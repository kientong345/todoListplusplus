use std::str::FromStr;

use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{postgres::types::PgInterval, prelude::FromRow};
use uuid::Uuid;

use crate::{
    model::error::ModelError,
    utils::{get_end_of_day, get_end_of_month, get_end_of_week, RecurrenceType},
};

pub mod delete;
pub mod dto;
pub mod get;
pub mod paginate;
pub mod patch;
pub mod post;

#[derive(Debug, Clone, sqlx::Type, Serialize)]
#[sqlx(type_name = "task_status", rename_all = "lowercase")]
pub enum TaskStatus {
    Open,
    Cancel,
    Done,
}

impl ToString for TaskStatus {
    fn to_string(&self) -> String {
        match self {
            TaskStatus::Open => "open".to_string(),
            TaskStatus::Cancel => "cancel".to_string(),
            TaskStatus::Done => "done".to_string(),
        }
    }
}

impl FromStr for TaskStatus {
    type Err = ModelError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "open" => Ok(TaskStatus::Open),
            "cancel" => Ok(TaskStatus::Cancel),
            "done" => Ok(TaskStatus::Done),
            _ => Err(ModelError::BadRequest(format!("Invalid TaskStatus: {}", s))),
        }
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct TaskDatabase {
    pub id: Uuid,
    pub category_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub user_comment: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub cycle_time: Option<PgInterval>,
    pub notify_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, FromRow)]
pub struct TaskMinimal {
    pub id: Uuid,
    pub title: String,
    pub status: TaskStatus,
    pub expires_at: Option<DateTime<Utc>>,
    pub cycle_time: Option<PgInterval>,
}

#[derive(Debug, Clone, FromRow)]
pub struct TaskDetail {
    pub id: Uuid,
    pub category_id: Uuid,
    pub category_name: String,
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub user_comment: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub cycle_time: Option<PgInterval>,
    pub notify_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct TaskCreateParams {
    pub category_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub cycle_time: Option<PgInterval>,
    pub notify_time: Option<DateTime<Utc>>,
}

impl TaskCreateParams {
    pub fn align_expiration(mut self, gmt: &str) -> Self {
        self.expires_at = if let Some(cycle_time) = &self.cycle_time {
            match RecurrenceType::try_from(*cycle_time) {
                Ok(RecurrenceType::Daily) => Some(get_end_of_day(Utc::now(), gmt)),
                Ok(RecurrenceType::Weekly) => Some(get_end_of_week(Utc::now(), gmt)),
                Ok(RecurrenceType::Monthly) => Some(get_end_of_month(Utc::now(), gmt)),
                _ => Some(get_end_of_day(Utc::now(), gmt)),
            }
        } else {
            self.expires_at
        };
        self
    }
}

#[derive(Debug, Clone)]
pub struct TaskUpdateParams {
    pub id: Uuid,
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<TaskStatus>,
    pub user_comment: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub cycle_time: Option<PgInterval>,
    pub notify_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub enum TaskSortBy {
    CreateTime,
    UpdateTime,
    ExpireTime,
}

impl FromStr for TaskSortBy {
    type Err = ModelError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "latest" => Ok(TaskSortBy::CreateTime),
            "new-update" => Ok(TaskSortBy::UpdateTime),
            "deadline" => Ok(TaskSortBy::ExpireTime),
            _ => Err(ModelError::BadRequest(format!("Invalid TaskSortBy: {}", s))),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TaskSearchParams {
    pub category_id: Uuid,
    pub title_pattern: Option<String>,
    pub status: Option<Vec<TaskStatus>>,
    pub page: i32,
    pub page_size: i32,
    pub sort_by: TaskSortBy, // default: desc order
}
