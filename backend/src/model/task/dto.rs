use std::str::FromStr;

use serde::{Deserialize, Serialize};
use sqlx::PgConnection;

use crate::{
    model::{
        error::ModelError,
        task::{
            TaskCreateParams, TaskDatabase, TaskDetail, TaskMinimal, TaskSearchParams, TaskSortBy,
            TaskStatus, TaskUpdateParams,
        },
    },
    utils::{datetime_to_string, pg_interval_to_string, string_to_datetime, string_to_pg_interval},
};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskMinimalDto {
    pub id: i32,
    pub title: String,
    pub status: String,
    pub expires_at: Option<String>,
    pub cycle_time: Option<String>,
}

impl From<TaskMinimal> for TaskMinimalDto {
    fn from(value: TaskMinimal) -> Self {
        TaskMinimalDto {
            id: value.id,
            title: value.title,
            status: value.status.to_string(),
            expires_at: value.expires_at.map(|dt| datetime_to_string(dt)),
            cycle_time: value.cycle_time.map(|d| pg_interval_to_string(d)),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskDetailDto {
    pub id: i32,
    pub category_id: i32,
    pub category_name: String,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub user_comment: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub expires_at: Option<String>,
    pub cycle_time: Option<String>,
    pub pre_notify_time: Option<String>,
}

impl From<TaskDetail> for TaskDetailDto {
    fn from(value: TaskDetail) -> Self {
        TaskDetailDto {
            id: value.id,
            category_id: value.category_id,
            category_name: value.category_name,
            title: value.title,
            description: value.description,
            status: value.status.to_string(),
            user_comment: value.user_comment,
            created_at: value.created_at.map(|dt| datetime_to_string(dt)),
            updated_at: value.updated_at.map(|dt| datetime_to_string(dt)),
            expires_at: value.expires_at.map(|dt| datetime_to_string(dt)),
            cycle_time: value.cycle_time.map(|d| pg_interval_to_string(d)),
            pre_notify_time: value.pre_notify_time.map(|d| pg_interval_to_string(d)),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskCreateDto {
    pub title: String,
    pub description: Option<String>,
    pub expires_at: Option<String>,
    pub cycle_time: Option<String>,
    pub pre_notify_time: Option<String>,
}

impl TaskCreateDto {
    pub fn bind(self, category_id: i32) -> TaskCreateParams {
        TaskCreateParams {
            category_id,
            title: self.title.clone(),
            description: self.description.clone(),
            expires_at: self.expires_at.clone().map(|s| string_to_datetime(&s)),
            cycle_time: self.cycle_time.clone().map(|s| string_to_pg_interval(&s)),
            pre_notify_time: self
                .pre_notify_time
                .clone()
                .map(|s| string_to_pg_interval(&s)),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskUpdateDto {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub user_comment: Option<String>,
}

impl TaskUpdateDto {
    pub fn bind(self, task_id: i32) -> TaskUpdateParams {
        TaskUpdateParams {
            id: task_id,
            title: self.title.clone(),
            description: self.description.clone(),
            status: self
                .status
                .clone()
                .map(|s| TaskStatus::from_str(&s).unwrap()),
            user_comment: self.user_comment.clone(),
            expires_at: None,
            cycle_time: None,
            pre_notify_time: None,
            next_version_id: None,
        }
    }
}

impl TaskUpdateParams {
    pub async fn validate(
        self,
        user_id: i32,
        connection: &mut PgConnection,
    ) -> Result<TaskUpdateParams, ModelError> {
        let owner_id = TaskDatabase::get_owner_id(self.id, connection).await?;

        if owner_id != user_id {
            return Err(ModelError::PermissionDenied(
                "cannot update other user's task".to_string(),
            ));
        }
        Ok(self.into())
    }
}

pub struct TaskScheduleDto {
    pub expires_at: Option<String>,
    pub cycle_time: Option<String>,
    pub pre_notify_time: Option<String>,
}

impl TaskScheduleDto {
    pub fn bind(self, task_id: i32) -> TaskUpdateParams {
        TaskUpdateParams {
            id: task_id,
            title: None,
            description: None,
            status: None,
            user_comment: None,
            expires_at: self.expires_at.clone().map(|s| string_to_datetime(&s)),
            cycle_time: self.cycle_time.clone().map(|s| string_to_pg_interval(&s)),
            pre_notify_time: self
                .pre_notify_time
                .clone()
                .map(|s| string_to_pg_interval(&s)),
            next_version_id: None,
        }
    }
}

pub struct TaskNextVersionDto {
    pub next_version_id: i32,
}

impl TaskNextVersionDto {
    pub fn bind(self, task_id: i32) -> TaskUpdateParams {
        TaskUpdateParams {
            id: task_id,
            title: None,
            description: None,
            status: None,
            user_comment: None,
            expires_at: None,
            cycle_time: None,
            pre_notify_time: None,
            next_version_id: Some(self.next_version_id),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TaskDeleteDto(i32);

impl From<i32> for TaskDeleteDto {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl TaskDeleteDto {
    pub async fn validate(
        self,
        user_id: i32,
        connection: &mut PgConnection,
    ) -> Result<i32, ModelError> {
        let owner_id = TaskDatabase::get_owner_id(self.0, connection).await?;

        if owner_id != user_id {
            return Err(ModelError::PermissionDenied(
                "cannot delete other user's task".to_string(),
            ));
        }
        Ok(self.0)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskSearchDto {
    pub title_pattern: Option<String>,
    pub status: Option<Vec<String>>,
    pub page: i32,
    pub page_size: i32,
    pub sort_by: String, // 'latest' | 'new-update' | 'deadline'
}

impl TaskSearchDto {
    pub fn bind(self, category_id: i32) -> TaskSearchParams {
        TaskSearchParams {
            category_id,
            title_pattern: self.title_pattern.clone(),
            status: self.status.clone().map(|s| {
                s.into_iter()
                    .map(|s| TaskStatus::from_str(&s).unwrap())
                    .collect()
            }),
            page: self.page,
            page_size: self.page_size,
            sort_by: TaskSortBy::from_str(&self.sort_by).unwrap_or(TaskSortBy::CreateTime),
        }
    }
}
