use sqlx::PgConnection;

use crate::model::{category::CategoryDatabase, error::ModelError};

impl CategoryDatabase {
    pub async fn delete_by_id(
        category_id: i32,
        connection: &mut PgConnection,
    ) -> Result<(), ModelError> {
        sqlx::query!(r#"DELETE FROM categories WHERE cat_id = $1"#, category_id)
            .execute(connection)
            .await?;
        Ok(())
    }
}
