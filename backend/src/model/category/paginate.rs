use sqlx::PgConnection;

use crate::model::{
    category::{CategoryMinimal, CategorySearchParams, CategorySortBy},
    error::ModelError,
    pagination::{Page, Paginate},
};

impl Paginate<CategorySearchParams> for CategoryMinimal {
    async fn page(
        params: &CategorySearchParams,
        connection: &mut PgConnection,
    ) -> Result<Page<Self>, ModelError> {
        let offset = (params.page.saturating_sub(1)) * params.page_size;
        let name_pattern = format!(
            "%{}%",
            params.name_pattern.clone().unwrap_or("".to_string())
        );
        let order_by = match params.sort_by {
            CategorySortBy::CreateTime => "created_at DESC",
            CategorySortBy::UpdateTime => "updated_at DESC",
            CategorySortBy::TaskCount => "task_count DESC",
            CategorySortBy::Progress => "progress DESC",
        };

        let query = format!(
            r#"SELECT 
                id, user_id, name, image_url, description, 
                CAST(progress AS DOUBLE PRECISION) as progress,
                task_count
            FROM categories_with_tasks_count
            WHERE user_id = $1 AND name ILIKE $2
            ORDER BY {}
            LIMIT $3 OFFSET $4"#,
            order_by
        );

        let items = sqlx::query_as::<_, CategoryMinimal>(&query)
            .bind(params.user_id)
            .bind(&name_pattern)
            .bind(params.page_size as i64)
            .bind(offset as i64)
            .fetch_all(&mut *connection)
            .await?;

        let total_items = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM categories_with_tasks_count WHERE user_id = $1 AND name ILIKE $2",
            params.user_id,
            name_pattern
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
