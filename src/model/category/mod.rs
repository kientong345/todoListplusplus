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

#[derive(Debug, Clone, FromRow)]
pub struct CategoryDatabase {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub image_url: Option<String>,
    pub description: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct CategoryMinimal {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub image_url: Option<String>,
    pub description: Option<String>,
    pub progress: f64,
    pub task_count: i64,
}

#[derive(Debug, Clone, FromRow)]
pub struct CategoryDetail {
    pub id: i32,
    pub user_id: i32,
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
    pub user_id: i32,
    pub name: String,
    pub image_url: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CategoryUpdateParams {
    pub id: i32,
    pub name: Option<String>,
    pub image_url: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub enum CategorySortBy {
    CreateTime,
    UpdateTime,
    TaskCount,
    Progress,
}

impl FromStr for CategorySortBy {
    type Err = ModelError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "create_time" => Ok(CategorySortBy::CreateTime),
            "update_time" => Ok(CategorySortBy::UpdateTime),
            "task_count" => Ok(CategorySortBy::TaskCount),
            "progress" => Ok(CategorySortBy::Progress),
            _ => Err(ModelError::BadRequest(format!("Invalid sort by: {}", s))),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CategorySearchParams {
    pub user_id: i32,
    pub name_pattern: Option<String>,
    pub page: i32,
    pub page_size: i32,
    pub sort_by: CategorySortBy, // default: desc order
}
