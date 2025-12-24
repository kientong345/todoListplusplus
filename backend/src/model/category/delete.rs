use crate::model::{category::CategoryDatabase, error::ModelError};
use sqlx::PgConnection;
use uuid::Uuid;

impl CategoryDatabase {
    pub async fn delete_by_id(
        category_id: Uuid,
        connection: &mut PgConnection,
    ) -> Result<(), ModelError> {
        sqlx::query!(r#"DELETE FROM categories WHERE cat_id = $1"#, category_id)
            .execute(connection)
            .await?;
        Ok(())
    }
}
