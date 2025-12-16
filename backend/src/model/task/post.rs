use sqlx::PgConnection;

use crate::model::{
    error::ModelError,
    task::{TaskCreateParams, TaskDatabase, TaskStatus},
};

impl TaskDatabase {
    pub async fn create_from(
        params: &TaskCreateParams,
        connection: &mut PgConnection,
    ) -> Result<TaskDatabase, ModelError> {
        let task = sqlx::query_as!(
            TaskDatabase,
            r#"INSERT INTO tasks (
                tsk_cat_id, tsk_title, tsk_description, tsk_status, tsk_usr_comment,
                tsk_expires_at, tsk_cycle_time, tsk_pre_notify_time
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
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
                tsk_cycle_time AS cycle_time,
                tsk_pre_notify_time AS pre_notify_time,
                tsk_next_version_id AS next_version_id"#,
            params.category_id,
            params.title,
            params.description,
            params.status.clone() as TaskStatus,
            params.user_comment,
            params.expires_at,
            params.cycle_time,
            params.pre_notify_time,
        )
        .fetch_one(connection)
        .await?;
        Ok(task)
    }
}
