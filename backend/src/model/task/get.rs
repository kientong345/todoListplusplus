use sqlx::PgConnection;
use uuid::Uuid;

use crate::model::{
    error::ModelError,
    task::{TaskDatabase, TaskDetail},
};

impl TaskDatabase {
    pub async fn get_by_id(
        task_id: Uuid,
        connection: &mut PgConnection,
    ) -> Result<TaskDatabase, ModelError> {
        let task = sqlx::query_as!(
            TaskDatabase,
            r#"SELECT
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
            tsk_next_version_id AS next_version_id
            FROM tasks
            WHERE tsk_id = $1 AND tsk_next_version_id IS NULL"#,
            task_id
        )
        .fetch_one(connection)
        .await?;
        Ok(task)
    }

    pub async fn get_owner_id(
        task_id: Uuid,
        connection: &mut PgConnection,
    ) -> Result<Uuid, ModelError> {
        let owner_id = sqlx::query_scalar!(
            r#"SELECT cat_usr_id
            FROM categories
            JOIN tasks ON cat_id = tsk_cat_id
            WHERE tsk_id = $1 AND tsk_next_version_id IS NULL"#,
            task_id
        )
        .fetch_one(connection)
        .await?;
        Ok(owner_id)
    }
}

impl TaskDetail {
    pub async fn get_by_id(
        task_id: Uuid,
        connection: &mut PgConnection,
    ) -> Result<TaskDetail, ModelError> {
        let task = sqlx::query_as!(
            TaskDetail,
            r#"SELECT
            tsk_id AS id,
            tsk_cat_id AS "category_id!",
            cat_name AS "category_name!",
            tsk_title AS title,
            tsk_description AS description,
            tsk_status AS "status: _",
            tsk_usr_comment AS user_comment,
            tsk_created_at AS created_at,
            tsk_updated_at AS updated_at,
            tsk_expires_at AS expires_at,
            tsk_cycle_time AS cycle_time,
            tsk_pre_notify_time AS pre_notify_time
            FROM tasks
            JOIN categories ON tsk_cat_id = cat_id
            WHERE tsk_id = $1 AND tsk_next_version_id IS NULL"#,
            task_id
        )
        .fetch_one(connection)
        .await?;
        Ok(task)
    }
}
