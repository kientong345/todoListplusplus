-- Add up migration script here
CREATE OR REPLACE VIEW categories_with_tasks_count AS
SELECT
    c.cat_id AS id,
    c.cat_usr_id AS user_id,
    c.cat_name AS name,
    c.cat_image_url AS image_url,
    c.cat_description AS description,
    c.cat_created_at AS created_at,
    c.cat_updated_at AS updated_at,
    COALESCE(t.opened_task_count, 0) AS opened_task_count,
    COALESCE(t.canceled_task_count, 0) AS canceled_task_count,
    COALESCE(t.done_task_count, 0) AS done_task_count,
    (
        COALESCE(t.opened_task_count, 0) + COALESCE(t.canceled_task_count, 0) + COALESCE(t.done_task_count, 0)
    ) AS task_count,
    CASE 
        WHEN (
            COALESCE(t.opened_task_count, 0) + COALESCE(t.canceled_task_count, 0) + COALESCE(t.done_task_count, 0)
        ) = 0 
        THEN 0
        ELSE COALESCE(t.done_task_count, 0) * 1.0 / 
            (
                COALESCE(t.opened_task_count, 0) + COALESCE(t.done_task_count, 0)
            )
    END AS progress
FROM categories AS c
LEFT JOIN (
    SELECT 
        tsk_cat_id,
        COUNT(CASE WHEN tsk_status = 'open' THEN tsk_id END) AS opened_task_count,
        COUNT(CASE WHEN tsk_status = 'cancel' THEN tsk_id END) AS canceled_task_count,
        COUNT(CASE WHEN tsk_status = 'done' THEN tsk_id END) AS done_task_count
    FROM tasks
    GROUP BY tsk_cat_id
) AS t ON t.tsk_cat_id = c.cat_id;