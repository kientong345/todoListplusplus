use sqlx::PgConnection;

use crate::model::{
    category::{CategoryCreateParams, CategoryDatabase},
    error::ModelError,
};

impl CategoryDatabase {
    pub async fn create_from(
        params: &CategoryCreateParams,
        connection: &mut PgConnection,
    ) -> Result<CategoryDatabase, ModelError> {
        let category = sqlx::query_as!(
            CategoryDatabase,
            r#"INSERT INTO categories (cat_usr_id, cat_name, cat_image_url, cat_description)
            VALUES ($1, $2, $3, $4)
            RETURNING
                cat_id AS id, cat_usr_id AS "user_id!", cat_name AS name,
                cat_image_url AS image_url, cat_description AS description,
                cat_created_at AS created_at, cat_updated_at AS updated_at"#,
            params.user_id,
            params.name,
            params.image_url,
            params.description,
        )
        .fetch_one(connection)
        .await?;
        Ok(category)
    }
}
