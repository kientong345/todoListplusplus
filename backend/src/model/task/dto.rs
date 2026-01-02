use std::str::FromStr;

use serde::{Deserialize, Serialize};
use sqlx::PgConnection;
use utoipa::ToSchema;
use uuid::Uuid;

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

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TaskMinimalDto {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub id: String,

    #[schema(example = "My Task")]
    pub title: String,

    #[schema(example = "new_update")]
    pub status: String,

    #[schema(example = "2022-01-01T00:00:00.000Z")]
    pub expires_at: Option<String>,

    #[schema(example = "1 day")]
    pub cycle_time: Option<String>,
}

impl From<TaskMinimal> for TaskMinimalDto {
    fn from(value: TaskMinimal) -> Self {
        TaskMinimalDto {
            id: value.id.to_string(),
            title: value.title,
            status: value.status.to_string(),
            expires_at: value.expires_at.map(|dt| datetime_to_string(dt)),
            cycle_time: value.cycle_time.map(|d| pg_interval_to_string(d)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TaskDetailDto {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub id: String,

    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub category_id: String,

    #[schema(example = "My Category")]
    pub category_name: String,

    #[schema(example = "My Task")]
    pub title: String,

    #[schema(example = "This is a task description")]
    pub description: Option<String>,

    #[schema(example = "new_update")]
    pub status: String,

    #[schema(example = "My User Comment")]
    pub user_comment: Option<String>,

    #[schema(example = "2022-01-01T00:00:00.000Z")]
    pub created_at: Option<String>,

    #[schema(example = "2022-01-01T00:00:00.000Z")]
    pub updated_at: Option<String>,

    #[schema(example = "2022-01-01T00:00:00.000Z")]
    pub expires_at: Option<String>,

    #[schema(example = "1 day")]
    pub cycle_time: Option<String>,

    #[schema(example = "2022-01-01T00:00:00.000Z")]
    pub notify_time: Option<String>,
}

impl From<TaskDetail> for TaskDetailDto {
    fn from(value: TaskDetail) -> Self {
        TaskDetailDto {
            id: value.id.to_string(),
            category_id: value.category_id.to_string(),
            category_name: value.category_name,
            title: value.title,
            description: value.description,
            status: value.status.to_string(),
            user_comment: value.user_comment,
            created_at: value.created_at.map(|dt| datetime_to_string(dt)),
            updated_at: value.updated_at.map(|dt| datetime_to_string(dt)),
            expires_at: value.expires_at.map(|dt| datetime_to_string(dt)),
            cycle_time: value.cycle_time.map(|d| pg_interval_to_string(d)),
            notify_time: value.notify_time.map(|d| datetime_to_string(d)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TaskCreateDto {
    #[schema(example = "My Task")]
    pub title: String,

    #[schema(example = "This is a task description")]
    pub description: Option<String>,

    #[schema(example = "2022-01-01T00:00:00.000Z")]
    pub expires_at: Option<String>,

    #[schema(example = "1 day")]
    pub cycle_time: Option<String>,

    #[schema(example = "2022-01-01T00:00:00.000Z")]
    pub notify_time: Option<String>,
}

impl TaskCreateDto {
    pub fn bind(self, category_id: String) -> TaskCreateParams {
        TaskCreateParams {
            category_id: Uuid::from_str(&category_id).unwrap(),
            title: self.title.clone(),
            description: self.description.clone(),
            expires_at: self.expires_at.clone().map(|s| string_to_datetime(&s)),
            cycle_time: self.cycle_time.clone().map(|s| string_to_pg_interval(&s)),
            notify_time: self.notify_time.clone().map(|s| string_to_datetime(&s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TaskUpdateDto {
    #[schema(example = "My Task")]
    pub title: Option<String>,

    #[schema(example = "This is a task description")]
    pub description: Option<String>,

    #[schema(example = "new_update")]
    pub status: Option<String>,

    #[schema(example = "My User Comment")]
    pub user_comment: Option<String>,
}

impl TaskUpdateDto {
    pub fn bind(self, task_id: String) -> TaskUpdateParams {
        TaskUpdateParams {
            id: Uuid::from_str(&task_id).unwrap(),
            title: self.title.clone(),
            description: self.description.clone(),
            status: self
                .status
                .clone()
                .map(|s| TaskStatus::from_str(&s).unwrap()),
            user_comment: self.user_comment.clone(),
            expires_at: None,
            cycle_time: None,
            notify_time: None,
        }
    }
}

impl TaskUpdateParams {
    pub async fn validate(
        self,
        user_id: String,
        connection: &mut PgConnection,
    ) -> Result<TaskUpdateParams, ModelError> {
        let owner_id = TaskDatabase::get_owner_id(self.id, connection).await?;

        if owner_id.to_string() != user_id {
            return Err(ModelError::PermissionDenied(
                "cannot update other user's task".to_string(),
            ));
        }
        Ok(self.into())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TaskScheduleDto {
    #[schema(example = "2022-01-01T00:00:00.000Z")]
    pub expires_at: Option<String>,

    #[schema(example = "1 day")]
    pub cycle_time: Option<String>,

    #[schema(example = "2022-01-01T00:00:00.000Z")]
    pub notify_time: Option<String>,
}

impl TaskScheduleDto {
    pub fn bind(self, task_id: String) -> TaskUpdateParams {
        let task_id = Uuid::from_str(&task_id).unwrap();
        TaskUpdateParams {
            id: task_id,
            title: None,
            description: None,
            status: None,
            user_comment: None,
            expires_at: self.expires_at.clone().map(|s| string_to_datetime(&s)),
            cycle_time: self.cycle_time.clone().map(|s| string_to_pg_interval(&s)),
            notify_time: self.notify_time.clone().map(|s| string_to_datetime(&s)),
        }
    }
}

#[derive(Debug, Clone, ToSchema)]
pub struct TaskDeleteDto(String);

impl From<String> for TaskDeleteDto {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl TaskDeleteDto {
    pub async fn validate(
        self,
        user_id: String,
        connection: &mut PgConnection,
    ) -> Result<Uuid, ModelError> {
        let task_id = Uuid::from_str(&self.0).unwrap();
        let owner_id = TaskDatabase::get_owner_id(task_id, connection).await?;

        if owner_id.to_string() != user_id {
            return Err(ModelError::PermissionDenied(
                "cannot delete other user's task".to_string(),
            ));
        }
        Ok(task_id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TaskSearchDto {
    #[schema(example = "My Task")]
    pub title_pattern: Option<String>,

    #[schema(example = "new_update")]
    pub status: Option<Vec<String>>,

    #[schema(example = "1")]
    pub page: i32,

    #[schema(example = "10")]
    pub page_size: i32,

    #[schema(example = "latest")]
    pub sort_by: String, // 'latest' | 'new-update' | 'deadline'
}

impl TaskSearchDto {
    pub fn bind(self, category_id: String) -> TaskSearchParams {
        TaskSearchParams {
            category_id: Uuid::from_str(&category_id).unwrap(),
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
