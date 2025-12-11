use sqlx::PgConnection;

use crate::model::{error::ModelError, task::TaskDatabase, task_scheduler::TaskScheduleParams};

impl TaskDatabase {
    pub async fn schedule(
        params: &TaskScheduleParams,
        connection: &mut PgConnection,
    ) -> Result<TaskDatabase, ModelError> {
        let task_id = sqlx::query_scalar!(
            "UPDATE tasks SET tsk_expires_at = $1, tsk_reschedule_at = $2 WHERE tsk_id = $3 RETURNING tsk_id",
            params.expires_at,
            params.reschedule_time,
            params.task_id,
        )
        .fetch_one(&mut *connection)
        .await?;

        Ok(TaskDatabase::get_by_id(task_id, connection).await?)
    }
}
