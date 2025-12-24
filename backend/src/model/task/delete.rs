use sqlx::PgConnection;
use uuid::Uuid;

use crate::model::{error::ModelError, task::TaskDatabase};

impl TaskDatabase {
    pub async fn delete_by_id(
        task_id: Uuid,
        connection: &mut PgConnection,
    ) -> Result<(), ModelError> {
        sqlx::query!(r#"DELETE FROM tasks WHERE tsk_id = $1"#, task_id)
            .execute(connection)
            .await?;
        Ok(())
    }
}
