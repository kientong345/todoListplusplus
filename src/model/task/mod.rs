use std::str::FromStr;

use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::prelude::FromRow;

use crate::model::error::ModelError;

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
    pub id: i32,
    pub category_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub user_comment: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub reschedule_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct TaskMinimal {
    pub id: i32,
    pub title: String,
    pub status: TaskStatus,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, FromRow)]
pub struct TaskDetail {
    pub id: i32,
    pub category_id: i32,
    pub category_name: String,
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub user_comment: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub reschedule_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct TaskCreateParams {
    pub category_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub user_comment: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub reschedule_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct TaskUpdateParams {
    pub id: i32,
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<TaskStatus>,
    pub user_comment: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub reschedule_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub enum TaskSortBy {
    CreateTime,
    UpdateTime,
    ExpiredTime,
    RescheduleTime,
}

impl FromStr for TaskSortBy {
    type Err = ModelError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "create_time" => Ok(TaskSortBy::CreateTime),
            "update_time" => Ok(TaskSortBy::UpdateTime),
            "expired_time" => Ok(TaskSortBy::ExpiredTime),
            "reschedule_time" => Ok(TaskSortBy::RescheduleTime),
            _ => Err(ModelError::BadRequest(format!("Invalid TaskSortBy: {}", s))),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TaskSearchParams {
    pub category_id: i32,
    pub title_pattern: Option<String>,
    pub status: Option<Vec<TaskStatus>>,
    pub page: i32,
    pub page_size: i32,
    pub sort_by: TaskSortBy, // default: desc order
}
