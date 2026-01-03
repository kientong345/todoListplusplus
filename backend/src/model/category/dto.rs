use std::str::FromStr;

use serde::{Deserialize, Serialize};
use sqlx::PgConnection;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    model::{
        category::{
            CategoryCreateParams, CategoryDatabase, CategoryDetail, CategoryMinimal,
            CategorySearchParams, CategorySortBy, CategoryUpdateParams,
        },
        error::ModelError,
    },
    utils::datetime_to_string,
};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CategoryDetailDto {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub id: String,

    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub user_id: String,

    #[schema(example = "My Category")]
    pub name: String,

    #[schema(example = "https://example.com/image.jpg")]
    pub image_url: Option<String>,

    #[schema(example = "This is a category description")]
    pub description: Option<String>,

    #[schema(example = "2022-01-01T00:00:00.000Z")]
    pub created_at: Option<String>,

    #[schema(example = "2022-01-01T00:00:00.000Z")]
    pub updated_at: Option<String>,

    #[schema(example = 10)]
    pub task_count: i64,

    #[schema(example = 5)]
    pub opened_task_count: i64,

    #[schema(example = 2)]
    pub canceled_task_count: i64,

    #[schema(example = 3)]
    pub done_task_count: i64,

    #[schema(example = 50.0)]
    pub progress: f64,
}

impl From<CategoryDetail> for CategoryDetailDto {
    fn from(value: CategoryDetail) -> Self {
        Self {
            id: value.id.to_string(),
            user_id: value.user_id.to_string(),
            name: value.name,
            image_url: value.image_url,
            description: value.description,
            created_at: value.created_at.map(|dt| datetime_to_string(dt)),
            updated_at: value.updated_at.map(|dt| datetime_to_string(dt)),
            task_count: value.task_count,
            opened_task_count: value.opened_task_count,
            canceled_task_count: value.canceled_task_count,
            done_task_count: value.done_task_count,
            progress: value.progress,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CategoryMinimalDto {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub id: String,

    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub user_id: String,

    #[schema(example = "My Category")]
    pub name: String,

    #[schema(example = "https://example.com/image.jpg")]
    pub image_url: Option<String>,

    #[schema(example = "This is a category description")]
    pub description: Option<String>,

    #[schema(example = 10)]
    pub task_count: i64,
}

impl From<CategoryMinimal> for CategoryMinimalDto {
    fn from(value: CategoryMinimal) -> Self {
        Self {
            id: value.id.to_string(),
            user_id: value.user_id.to_string(),
            name: value.name,
            image_url: value.image_url,
            description: value.description,
            task_count: value.task_count,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CategoryCreateDto {
    #[schema(example = "My Category")]
    pub name: String,

    #[schema(example = "https://example.com/image.jpg")]
    pub image_url: Option<String>,

    #[schema(example = "This is a category description")]
    pub description: Option<String>,
}

impl CategoryCreateDto {
    pub fn bind(self, user_id: String) -> CategoryCreateParams {
        let user_id = Uuid::from_str(&user_id).unwrap();
        CategoryCreateParams {
            user_id,
            name: self.name.clone(),
            image_url: self.image_url.clone(),
            description: self.description.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CategoryUpdateDto {
    #[schema(example = "My Category")]
    pub name: Option<String>,

    #[schema(example = "https://example.com/image.jpg")]
    pub image_url: Option<String>,

    #[schema(example = "This is a category description")]
    pub description: Option<String>,
}

impl CategoryUpdateDto {
    pub fn bind(self, id: String) -> CategoryUpdateParams {
        let id = Uuid::from_str(&id).unwrap();
        CategoryUpdateParams {
            id,
            name: self.name,
            image_url: self.image_url,
            description: self.description,
        }
    }
}

impl CategoryUpdateParams {
    pub async fn validate(
        self,
        user_id: String,
        connection: &mut PgConnection,
    ) -> Result<CategoryUpdateParams, ModelError> {
        let user_id = Uuid::from_str(&user_id).unwrap();
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
pub struct CategoryDeleteDto(String);

impl From<String> for CategoryDeleteDto {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl CategoryDeleteDto {
    pub async fn validate(
        self,
        user_id: String,
        connection: &mut PgConnection,
    ) -> Result<Uuid, ModelError> {
        let uid = Uuid::from_str(&self.0).unwrap();
        let category = CategoryDatabase::get_by_id(uid, connection).await?;

        if category.user_id.to_string() != user_id {
            return Err(ModelError::PermissionDenied(
                "cannot delete other user's category".to_string(),
            ));
        }
        Ok(uid)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CategorySearchDto {
    #[schema(example = "My Category")]
    pub name_pattern: Option<String>,

    #[schema(example = 1)]
    pub page: i32,

    #[schema(example = 10)]
    pub page_size: i32,

    #[schema(example = "new_update")]
    pub sort_by: String, // new_update | task_count | progress
}

impl CategorySearchDto {
    pub fn bind(self, user_id: String) -> CategorySearchParams {
        let user_id = Uuid::from_str(&user_id).unwrap();
        CategorySearchParams {
            user_id,
            name_pattern: self.name_pattern.clone(),
            page: self.page,
            page_size: self.page_size,
            sort_by: CategorySortBy::from_str(&self.sort_by).unwrap_or(CategorySortBy::UpdateTime),
        }
    }
}
