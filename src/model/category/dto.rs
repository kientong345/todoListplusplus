use std::str::FromStr;

use serde::{Deserialize, Serialize};
use sqlx::PgConnection;

use crate::model::{
    category::{
        CategoryCreateParams, CategoryDatabase, CategoryDetail, CategoryMinimal,
        CategorySearchParams, CategorySortBy, CategoryUpdateParams,
    },
    error::ModelError,
};

#[derive(Debug, Clone, Serialize)]
pub struct CategoryDetailDto {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub image_url: Option<String>,
    pub description: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub task_count: i64,
    pub opened_task_count: i64,
    pub canceled_task_count: i64,
    pub done_task_count: i64,
    pub progress: f64,
}

impl From<CategoryDetail> for CategoryDetailDto {
    fn from(value: CategoryDetail) -> Self {
        Self {
            id: value.id,
            user_id: value.user_id,
            name: value.name,
            image_url: value.image_url,
            description: value.description,
            created_at: value.created_at.map(|dt| dt.to_rfc3339()),
            updated_at: value.updated_at.map(|dt| dt.to_rfc3339()),
            task_count: value.task_count,
            opened_task_count: value.opened_task_count,
            canceled_task_count: value.canceled_task_count,
            done_task_count: value.done_task_count,
            progress: value.progress,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct CategoryMinimalDto {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub image_url: Option<String>,
    pub description: Option<String>,
    pub task_count: i64,
}

impl From<CategoryMinimal> for CategoryMinimalDto {
    fn from(value: CategoryMinimal) -> Self {
        Self {
            id: value.id,
            user_id: value.user_id,
            name: value.name,
            image_url: value.image_url,
            description: value.description,
            task_count: value.task_count,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CategoryCreateDto {
    pub name: String,
    pub image_url: Option<String>,
    pub description: Option<String>,
}

impl CategoryCreateDto {
    pub fn bind(self, user_id: i32) -> CategoryCreateParams {
        CategoryCreateParams {
            user_id,
            name: self.name.clone(),
            image_url: self.image_url.clone(),
            description: self.description.clone(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CategoryUpdateDto {
    pub id: i32,
    pub name: Option<String>,
    pub image_url: Option<String>,
    pub description: Option<String>,
}

impl Into<CategoryUpdateParams> for CategoryUpdateDto {
    fn into(self) -> CategoryUpdateParams {
        CategoryUpdateParams {
            id: self.id,
            name: self.name,
            image_url: self.image_url,
            description: self.description,
        }
    }
}

impl CategoryUpdateDto {
    pub async fn validate(
        self,
        user_id: i32,
        connection: &mut PgConnection,
    ) -> Result<CategoryUpdateParams, ModelError> {
        let category = CategoryDatabase::get_by_id(self.id, connection).await?;

        if category.user_id != user_id {
            return Err(ModelError::PermissionDenied(
                "cannot modify other user's category".to_string(),
            ));
        }

        Ok(self.into())
    }
}

#[derive(Debug, Clone)]
pub struct CategoryDeleteDto(i32);

impl From<i32> for CategoryDeleteDto {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl CategoryDeleteDto {
    pub async fn validate(
        self,
        user_id: i32,
        connection: &mut PgConnection,
    ) -> Result<i32, ModelError> {
        let category = CategoryDatabase::get_by_id(self.0, connection).await?;

        if category.user_id != user_id {
            return Err(ModelError::PermissionDenied(
                "cannot delete other user's category".to_string(),
            ));
        }
        Ok(self.0)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CategorySearchDto {
    pub name_pattern: Option<String>,
    pub page: i32,
    pub page_size: i32,
    pub sort_by: String,
}

impl CategorySearchDto {
    pub fn bind(self, user_id: i32) -> CategorySearchParams {
        CategorySearchParams {
            user_id,
            name_pattern: self.name_pattern.clone(),
            page: self.page,
            page_size: self.page_size,
            sort_by: CategorySortBy::from_str(&self.sort_by).unwrap_or(CategorySortBy::CreateTime),
        }
    }
}
