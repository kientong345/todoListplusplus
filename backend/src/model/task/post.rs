use sqlx::PgConnection;
use uuid::Uuid;

use crate::{
    model::{
        error::ModelError,
        task::{TaskCreateParams, TaskDatabase},
    },
    utils::pg_interval_to_time,
};

impl TaskDatabase {
    pub async fn create_from(
        params: &TaskCreateParams,
        connection: &mut PgConnection,
    ) -> Result<TaskDatabase, ModelError> {
        let task_id = sqlx::query_scalar!(
            r#"INSERT INTO task_chains (
                chain_cat_id, chain_title, chain_description, chain_cycle_time
            )
            VALUES ($1, $2, $3, $4)
            RETURNING chain_id"#,
            params.category_id,
            params.title,
            params.description,
            params.cycle_time,
        )
        .fetch_one(&mut *connection)
        .await?;

        sqlx::query!(
            r#"INSERT INTO task_chain_links (
                link_chain_id, link_expires_at, link_notify_time
            )
            VALUES ($1, $2, $3)"#,
            task_id,
            params.expires_at,
            params.notify_time,
        )
        .execute(&mut *connection)
        .await?;

        let task = TaskDatabase::get_by_id(task_id, &mut *connection).await?;
        Ok(task)
    }

    pub async fn spawn_new_link(
        task_id: Uuid,
        connection: &mut PgConnection,
    ) -> Result<TaskDatabase, ModelError> {
        let task = TaskDatabase::get_by_id(task_id, &mut *connection).await?;
        let prev_link_id = sqlx::query_scalar!(
            r#"UPDATE task_chain_links
            SET link_is_latest = FALSE
            WHERE link_chain_id = $1 AND link_is_latest = TRUE
            RETURNING link_id"#,
            task_id,
        )
        .fetch_one(&mut *connection)
        .await?;

        let next_expires_at = if let Some(curr_expires_at) = task.expires_at {
            Some(curr_expires_at + pg_interval_to_time(task.cycle_time.unwrap_or_default()))
        } else {
            None
        };

        sqlx::query!(
            r#"INSERT INTO task_chain_links (
                link_chain_id, link_expires_at, link_notify_time, link_prev_id
            )
            VALUES ($1, $2, $3, $4)"#,
            task_id,
            next_expires_at,
            task.notify_time,
            prev_link_id,
        )
        .execute(&mut *connection)
        .await?;

        Ok(task)
    }
}
