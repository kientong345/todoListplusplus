use sqlx::PgConnection;

use crate::model::{
    category::{CategoryDatabase, CategoryDetail},
    error::ModelError,
};

impl CategoryDatabase {
    pub async fn get_by_id(
        category_id: i32,
        connection: &mut PgConnection,
    ) -> Result<CategoryDatabase, ModelError> {
        let category = sqlx::query_as!(
            CategoryDatabase,
            r#"SELECT 
                cat_id AS id, cat_usr_id AS "user_id!", cat_name AS name,
                cat_image_url AS image_url, cat_description AS description,
                cat_created_at AS created_at, cat_updated_at AS updated_at
            FROM categories WHERE cat_id = $1"#,
            category_id
        )
        .fetch_one(connection)
        .await?;
        Ok(category)
    }
}

impl CategoryDetail {
    pub async fn get_by_id(
        category_id: i32,
        connection: &mut PgConnection,
    ) -> Result<CategoryDetail, ModelError> {
        let category = sqlx::query_as!(
            CategoryDetail,
            r#"SELECT 
                id AS "id!", user_id AS "user_id!", name AS "name!",
                image_url AS "image_url!", description AS "description!",
                created_at AS "created_at!", updated_at AS "updated_at!",
                progress AS "progress!: f64",
                task_count AS "task_count!",
                opened_task_count AS "opened_task_count!",
                canceled_task_count AS "canceled_task_count!",
                done_task_count AS "done_task_count!"
            FROM categories_with_tasks_count WHERE id = $1"#,
            category_id
        )
        .fetch_one(connection)
        .await?;
        Ok(category)
    }
}
