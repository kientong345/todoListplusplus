use sqlx::PgConnection;

use crate::model::{
    error::ModelError,
    task::{TaskDatabase, TaskStatus, TaskUpdateParams},
};

impl TaskDatabase {
    pub async fn update_by_id(
        params: TaskUpdateParams,
        connection: &mut PgConnection,
    ) -> Result<TaskDatabase, ModelError> {
        let task = sqlx::query_as!(
            TaskDatabase,
            r#"UPDATE tasks SET tsk_id = $1,
            tsk_title = $2,
            tsk_description = $3,
            tsk_status = $4,
            tsk_usr_comment = $5,
            tsk_expires_at = $6,
            tsk_reschedule_at = $7
            RETURNING
                tsk_id AS id,
                tsk_cat_id AS "category_id!",
                tsk_title AS title,
                tsk_description AS description,
                tsk_status AS "status: _",
                tsk_usr_comment AS user_comment,
                tsk_created_at AS created_at,
                tsk_updated_at AS updated_at,
                tsk_expires_at AS expires_at,
                tsk_reschedule_at AS reschedule_at"#,
            params.id,
            params.title,
            params.description,
            params.status as Option<TaskStatus>,
            params.user_comment,
            params.expires_at,
            params.reschedule_at
        )
        .fetch_one(connection)
        .await?;
        Ok(task)
    }
}
