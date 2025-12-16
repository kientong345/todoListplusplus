use sqlx::PgConnection;

use crate::model::{
    category::{CategoryDatabase, CategoryUpdateParams},
    error::ModelError,
};

impl CategoryDatabase {
    pub async fn update(
        params: &CategoryUpdateParams,
        connection: &mut PgConnection,
    ) -> Result<CategoryDatabase, ModelError> {
        let category = sqlx::query_as!(
            CategoryDatabase,
            r#"UPDATE categories
            SET cat_name = $2, cat_image_url = $3, cat_description = $4
            WHERE cat_id = $1
            RETURNING
                cat_id AS id, cat_usr_id AS "user_id!", cat_name AS name,
                cat_image_url AS image_url, cat_description AS description,
                cat_created_at AS created_at, cat_updated_at AS updated_at"#,
            params.id,
            params.name,
            params.image_url,
            params.description,
        )
        .fetch_one(connection)
        .await?;
        Ok(category)
    }
}
