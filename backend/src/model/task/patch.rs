use chrono::{DateTime, Utc};
use sqlx::{postgres::types::PgInterval, PgConnection};
use uuid::Uuid;

use crate::model::{
    error::ModelError,
    task::{TaskDatabase, TaskStatus, TaskUpdateParams},
};

impl TaskDatabase {
    pub async fn update(
        params: &TaskUpdateParams,
        connection: &mut PgConnection,
    ) -> Result<TaskDatabase, ModelError> {
        if let Some(title) = &params.title {
            Self::update_title(params.id, title, connection).await?;
        }
        if let Some(description) = &params.description {
            Self::update_description(params.id, description, connection).await?;
        }
        if let Some(status) = &params.status {
            Self::update_status(params.id, status.clone(), connection).await?;
        }
        if let Some(user_comment) = &params.user_comment {
            Self::update_user_comment(params.id, user_comment, connection).await?;
        }
        if let Some(expires_at) = &params.expires_at {
            Self::update_expires_at(params.id, expires_at.clone(), connection).await?;
        }
        if let Some(cycle_time) = &params.cycle_time {
            Self::update_cycle_time(params.id, cycle_time.clone(), connection).await?;
        }
        if let Some(pre_notify_time) = &params.pre_notify_time {
            Self::update_pre_notify_time(params.id, pre_notify_time.clone(), connection).await?;
        }
        if let Some(next_version_id) = &params.next_version_id {
            Self::update_next_version_id(params.id, next_version_id.clone(), connection).await?;
        }
        let task = Self::get_by_id(params.id, connection).await?;
        Ok(task)
    }

    async fn update_title(
        task_id: Uuid,
        title: &str,
        connection: &mut PgConnection,
    ) -> Result<(), ModelError> {
        sqlx::query!(
            r#"UPDATE tasks
            SET tsk_title = $2
            WHERE tsk_id = $1"#,
            task_id,
            title,
        )
        .execute(connection)
        .await?;
        Ok(())
    }

    async fn update_description(
        task_id: Uuid,
        description: &str,
        connection: &mut PgConnection,
    ) -> Result<(), ModelError> {
        sqlx::query!(
            r#"UPDATE tasks
            SET tsk_description = $2
            WHERE tsk_id = $1"#,
            task_id,
            description,
        )
        .execute(connection)
        .await?;
        Ok(())
    }

    async fn update_status(
        task_id: Uuid,
        status: TaskStatus,
        connection: &mut PgConnection,
    ) -> Result<(), ModelError> {
        sqlx::query!(
            r#"UPDATE tasks
            SET tsk_status = $2
            WHERE tsk_id = $1"#,
            task_id,
            status as TaskStatus,
        )
        .execute(connection)
        .await?;
        Ok(())
    }

    async fn update_user_comment(
        task_id: Uuid,
        user_comment: &str,
        connection: &mut PgConnection,
    ) -> Result<(), ModelError> {
        sqlx::query!(
            r#"UPDATE tasks
            SET tsk_usr_comment = $2
            WHERE tsk_id = $1"#,
            task_id,
            user_comment,
        )
        .execute(connection)
        .await?;
        Ok(())
    }

    async fn update_expires_at(
        task_id: Uuid,
        expires_at: DateTime<Utc>,
        connection: &mut PgConnection,
    ) -> Result<(), ModelError> {
        sqlx::query!(
            r#"UPDATE tasks
            SET tsk_expires_at = $2
            WHERE tsk_id = $1"#,
            task_id,
            expires_at,
        )
        .execute(connection)
        .await?;
        Ok(())
    }

    async fn update_cycle_time(
        task_id: Uuid,
        cycle_time: PgInterval,
        connection: &mut PgConnection,
    ) -> Result<(), ModelError> {
        sqlx::query!(
            r#"UPDATE tasks
            SET tsk_cycle_time = $2
            WHERE tsk_id = $1"#,
            task_id,
            cycle_time,
        )
        .execute(connection)
        .await?;
        Ok(())
    }

    async fn update_pre_notify_time(
        task_id: Uuid,
        pre_notify_time: PgInterval,
        connection: &mut PgConnection,
    ) -> Result<(), ModelError> {
        sqlx::query!(
            r#"UPDATE tasks
            SET tsk_pre_notify_time = $2
            WHERE tsk_id = $1"#,
            task_id,
            pre_notify_time,
        )
        .execute(connection)
        .await?;
        Ok(())
    }

    async fn update_next_version_id(
        task_id: Uuid,
        next_version_id: Uuid,
        connection: &mut PgConnection,
    ) -> Result<(), ModelError> {
        sqlx::query!(
            r#"UPDATE tasks
            SET tsk_next_version_id = $2
            WHERE tsk_id = $1"#,
            task_id,
            next_version_id,
        )
        .execute(connection)
        .await?;
        Ok(())
    }
}
