use std::str::FromStr;

use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::model::error::ModelError;

pub mod delete;
pub mod dto;
pub mod get;
pub mod paginate;
pub mod patch;
pub mod post;

#[derive(Debug, Clone, FromRow)]
pub struct CategoryDatabase {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub image_url: Option<String>,
    pub description: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, FromRow)]
pub struct CategoryMinimal {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub image_url: Option<String>,
    pub description: Option<String>,
    pub progress: f64,
    pub task_count: i64,
}

#[derive(Debug, Clone, FromRow)]
pub struct CategoryDetail {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub image_url: Option<String>,
    pub description: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub progress: f64,
    pub task_count: i64,
    pub opened_task_count: i64,
    pub canceled_task_count: i64,
    pub done_task_count: i64,
}

#[derive(Debug, Clone)]
pub struct CategoryCreateParams {
    pub user_id: Uuid,
    pub name: String,
    pub image_url: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CategoryUpdateParams {
    pub id: Uuid,
    pub name: Option<String>,
    pub image_url: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub enum CategorySortBy {
    UpdateTime,
    TaskCount,
    Progress,
}

impl FromStr for CategorySortBy {
    type Err = ModelError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "new-update" => Ok(CategorySortBy::UpdateTime),
            "task-count" => Ok(CategorySortBy::TaskCount),
            "progress" => Ok(CategorySortBy::Progress),
            _ => Err(ModelError::BadRequest(format!("Invalid sort by: {}", s))),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CategorySearchParams {
    pub user_id: Uuid,
    pub name_pattern: Option<String>,
    pub page: i32,
    pub page_size: i32,
    pub sort_by: CategorySortBy, // default: desc order
}
