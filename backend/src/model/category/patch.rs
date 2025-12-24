use sqlx::PgConnection;
use uuid::Uuid;

use crate::model::{
    category::{CategoryDatabase, CategoryUpdateParams},
    error::ModelError,
};

impl CategoryDatabase {
    pub async fn update(
        params: &CategoryUpdateParams,
        connection: &mut PgConnection,
    ) -> Result<CategoryDatabase, ModelError> {
        if let Some(name) = &params.name {
            Self::update_name(params.id, name, connection).await?;
        }
        if let Some(image_url) = &params.image_url {
            Self::update_image_url(params.id, image_url, connection).await?;
        }
        if let Some(description) = &params.description {
            Self::update_description(params.id, description, connection).await?;
        }
        let category = Self::get_by_id(params.id, connection).await?;
        Ok(category)
    }

    async fn update_name(
        category_id: Uuid,
        name: &str,
        connection: &mut PgConnection,
    ) -> Result<(), ModelError> {
        sqlx::query!(
            r#"UPDATE categories
            SET cat_name = $2
            WHERE cat_id = $1"#,
            category_id,
            name,
        )
        .execute(connection)
        .await?;
        Ok(())
    }

    async fn update_image_url(
        category_id: Uuid,
        image_url: &str,
        connection: &mut PgConnection,
    ) -> Result<(), ModelError> {
        sqlx::query!(
            r#"UPDATE categories
            SET cat_image_url = $2
            WHERE cat_id = $1"#,
            category_id,
            image_url,
        )
        .execute(connection)
        .await?;
        Ok(())
    }

    async fn update_description(
        category_id: Uuid,
        description: &str,
        connection: &mut PgConnection,
    ) -> Result<(), ModelError> {
        sqlx::query!(
            r#"UPDATE categories
            SET cat_description = $2
            WHERE cat_id = $1"#,
            category_id,
            description,
        )
        .execute(connection)
        .await?;
        Ok(())
    }
}
