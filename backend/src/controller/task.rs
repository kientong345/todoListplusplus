use std::str::FromStr;

use axum::{
    extract::{Path, Query, State},
    Extension, Json,
};
use reqwest::StatusCode;
use serde_json::{json, Value};
use uuid::Uuid;

use crate::{
    app::AppState,
    cache::{Caching, DEFAULT_TTL_SECONDS},
    controller::error::ControllerError,
    model::{
        pagination::{PageDto, Paginate},
        task::{
            dto::{
                TaskCreateDto, TaskDeleteDto, TaskDetailDto, TaskMinimalDto, TaskSearchDto,
                TaskUpdateDto,
            },
            TaskDatabase, TaskDetail, TaskMinimal,
        },
        user_auth::AccessClaims,
    },
    service::task_scheduler::{UpdateEvent, UpdateEventType},
    utils::pg_interval_to_time,
};

#[utoipa::path(
    get,
    path = "/tasks/{category_id}",
    params(
        ("category_id" = String, Path, description = "Category ID"),
        ("page" = i32, Query, description = "Page number"),
        ("pageSize" = i32, Query, description = "Page size"),
        ("sortBy" = String, Query, description = "Sort by"),
    ),
    responses(
        (status = 200, description = "Success", body = PageDto<TaskMinimalDto>),
    ),
)]
pub async fn get_page(
    State(state): State<AppState>,
    Query(query): Query<TaskSearchDto>,
    Path(category_id): Path<String>,
    Extension(_access_claims): Extension<AccessClaims>,
) -> Result<Json<Value>, ControllerError> {
    // let user_id = access_claims.sub.parse().unwrap();
    let mut query = query.clone();
    if let Some(statuses) = &mut query.status {
        statuses.sort();
    }
    let cache_key = format!(
        "todolist++:tasks:title_pattern={}&status={}&page={}&pageSize={}&sortBy={}",
        query.title_pattern.clone().unwrap_or("".to_string()),
        query.status.clone().unwrap_or(vec![]).join(","),
        query.page,
        query.page_size,
        query.sort_by
    );

    if let Ok(Some(tasks)) = state.cache.get::<PageDto<TaskMinimalDto>>(&cache_key).await {
        return Ok(Json(json!(tasks)));
    }

    let mut connection = state.db.start_transaction().await?;

    let page = TaskMinimal::page(&query.bind(category_id), &mut *connection)
        .await?
        .map_into::<TaskMinimalDto>();

    connection.commit().await?;

    let _ = state
        .cache
        .set::<PageDto<TaskMinimalDto>>(&cache_key, &page, DEFAULT_TTL_SECONDS)
        .await;

    Ok(Json(json!(page)))
}

#[utoipa::path(
    get,
    path = "/tasks/{category_id}/{task_id}",
    params(
        ("category_id" = String, Path, description = "Category ID"),
        ("task_id" = String, Path, description = "Task ID"),
    ),
    responses(
        (status = 200, description = "Success", body = TaskDetailDto),
    ),
)]
pub async fn find_by_id(
    State(state): State<AppState>,
    Path((_category_id, task_id)): Path<(String, String)>,
    Extension(_access_claims): Extension<AccessClaims>,
) -> Result<Json<Value>, ControllerError> {
    let task_id = Uuid::from_str(&task_id).unwrap();
    let cache_key = format!("todolist++:tasks:{}", task_id);

    if let Ok(Some(task)) = state.cache.get::<TaskDetailDto>(&cache_key).await {
        return Ok(Json(json!(task)));
    }

    let mut connection = state.db.start_transaction().await?;

    let task: TaskDetailDto = TaskDetail::get_by_id(task_id, &mut *connection)
        .await?
        .into();

    connection.commit().await?;

    let _ = state
        .cache
        .set::<TaskDetailDto>(&cache_key, &task, DEFAULT_TTL_SECONDS)
        .await;

    Ok(Json(json!(task)))
}

#[utoipa::path(
    post,
    path = "/tasks/{category_id}",
    params(
        ("category_id" = String, Path, description = "Category ID"),
    ),
    request_body = TaskCreateDto,
    responses(
        (status = 201, description = "Success", body = TaskDetailDto),
    ),
)]
pub async fn create(
    State(state): State<AppState>,
    Path(category_id): Path<String>,
    Extension(_access_claims): Extension<AccessClaims>,
    Json(payload): Json<TaskCreateDto>,
) -> Result<StatusCode, ControllerError> {
    let gmt = state.config.app_config.gmt.clone();
    let create_params = payload.bind(category_id).align_expiration(&gmt);
    let cache_key_prefix = "todolist++:tasks";

    let mut connection = state.db.start_transaction().await?;
    let new_task = TaskDatabase::create_from(&create_params, &mut *connection).await?;
    connection.commit().await?;

    if let Some(expires_at) = new_task.expires_at {
        let schedule_update_event = UpdateEvent {
            task_id: new_task.id,
            r#type: UpdateEventType::NewScheduledTask {
                expires_at,
                cycle_time: new_task.cycle_time.map(|x| pg_interval_to_time(x)),
                notify_time: new_task.notify_time,
            },
        };
        state
            .scheduler_service
            .trigger_schedule_update_event(schedule_update_event)
            .await
            .expect("oof");
    }

    let _ = state.cache.delete_prefix(&cache_key_prefix).await;

    Ok(StatusCode::CREATED)
}

#[utoipa::path(
    delete,
    path = "/tasks/{category_id}/{task_id}",
    params(
        ("category_id" = String, Path, description = "Category ID"),
        ("task_id" = String, Path, description = "Task ID"),
    ),
    responses(
        (status = 200, description = "Success", body = TaskDetailDto),
    ),
)]
pub async fn delete(
    State(state): State<AppState>,
    Path((_category_id, task_id)): Path<(String, String)>,
    Extension(access_claims): Extension<AccessClaims>,
) -> Result<StatusCode, ControllerError> {
    let user_id = access_claims.sub.parse().unwrap();
    let cache_key_prefix = "todolist++:tasks";

    let mut connection = state.db.start_transaction().await?;

    let validated_id = TaskDeleteDto::from(task_id)
        .validate(user_id, &mut *connection)
        .await?;

    TaskDatabase::delete_by_id(validated_id, &mut *connection).await?;

    connection.commit().await?;

    let _ = state.cache.delete_prefix(&cache_key_prefix).await;

    Ok(StatusCode::OK)
}

#[utoipa::path(
    put,
    path = "/tasks/{category_id}/{task_id}",
    params(
        ("category_id" = String, Path, description = "Category ID"),
        ("task_id" = String, Path, description = "Task ID"),
    ),
    request_body = TaskUpdateDto,
    responses(
        (status = 200, description = "Success", body = TaskDetailDto),
    ),
)]
pub async fn update(
    State(state): State<AppState>,
    Path((_category_id, task_id)): Path<(String, String)>,
    Extension(access_claims): Extension<AccessClaims>,
    Json(payload): Json<TaskUpdateDto>,
) -> Result<StatusCode, ControllerError> {
    let user_id = access_claims.sub.parse().unwrap();
    let cache_key_prefix = "todolist++:tasks";

    let mut connection = state.db.start_transaction().await?;

    let validated_params = payload
        .bind(task_id)
        .validate(user_id, &mut *connection)
        .await?;

    TaskDatabase::update(&validated_params, &mut *connection).await?;

    connection.commit().await?;

    let _ = state.cache.delete_prefix(&cache_key_prefix).await;

    Ok(StatusCode::OK)
}
