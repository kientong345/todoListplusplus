use crate::model::{
    error::ModelError,
    pagination::{Page, Paginate},
    task::{TaskMinimal, TaskSearchParams, TaskSortBy},
};
use sqlx::PgConnection;

impl Paginate<TaskSearchParams> for TaskMinimal {
    async fn page(
        params: &TaskSearchParams,
        connection: &mut PgConnection,
    ) -> Result<Page<Self>, ModelError> {
        let offset = (params.page.saturating_sub(1)) * params.page_size;
        let title_pattern = format!(
            "%{}%",
            params.title_pattern.clone().unwrap_or("".to_string())
        );
        let order_by = match params.sort_by {
            TaskSortBy::CreateTime => "tsk_created_at DESC",
            TaskSortBy::UpdateTime => "tsk_updated_at DESC",
            TaskSortBy::ExpiredTime => "tsk_expires_at DESC",
            TaskSortBy::RescheduleTime => "tsk_reschedule_at DESC",
        };

        let query = format!(
            r#"SELECT 
                tsk_id AS id, tsk_cat_id AS category_id, tsk_title AS title, tsk_description AS description,
                tsk_status AS "status: _", tsk_usr_comment AS user_comment, tsk_created_at AS created_at,
                tsk_updated_at AS updated_at, tsk_expires_at AS expires_at, tsk_reschedule_at AS reschedule_at
            FROM tasks
            WHERE tsk_cat_id = $1 AND tsk_title ILIKE $2
            ORDER BY {}
            LIMIT $3 OFFSET $4"#,
            order_by
        );

        let items = sqlx::query_as::<_, TaskMinimal>(&query)
            .bind(params.category_id)
            .bind(&title_pattern)
            .bind(params.page_size as i64)
            .bind(offset as i64)
            .fetch_all(&mut *connection)
            .await?;

        let total_items = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM tasks WHERE tsk_cat_id = $1 AND tsk_title ILIKE $2",
            params.category_id,
            title_pattern
        )
        .fetch_one(connection)
        .await?
        .unwrap_or(0);

        Ok(Page::build_from(
            items,
            total_items,
            params.page,
            params.page_size,
        ))
    }
}
