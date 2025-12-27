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
    controller::error::ControllerError,
    model::{
        pagination::Paginate,
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

pub async fn get_page(
    State(state): State<AppState>,
    Query(query): Query<TaskSearchDto>,
    Path(category_id): Path<String>,
    Extension(_access_claims): Extension<AccessClaims>,
) -> Result<Json<Value>, ControllerError> {
    // let user_id = access_claims.sub.parse().unwrap();
    let mut connection = state.db.start_transaction().await?;

    let page = TaskMinimal::page(&query.bind(category_id), &mut *connection)
        .await?
        .map_into::<TaskMinimalDto>();

    connection.commit().await?;

    Ok(Json(json!(page)))
}

pub async fn find_by_id(
    State(state): State<AppState>,
    Path((_category_id, task_id)): Path<(String, String)>,
    Extension(_access_claims): Extension<AccessClaims>,
) -> Result<Json<Value>, ControllerError> {
    let task_id = Uuid::from_str(&task_id).unwrap();
    let mut connection = state.db.start_transaction().await?;

    let task: TaskDetailDto = TaskDetail::get_by_id(task_id, &mut *connection)
        .await?
        .into();

    connection.commit().await?;

    Ok(Json(json!(task)))
}

pub async fn create(
    State(state): State<AppState>,
    Path(category_id): Path<String>,
    Extension(_access_claims): Extension<AccessClaims>,
    Json(payload): Json<TaskCreateDto>,
) -> Result<StatusCode, ControllerError> {
    let mut connection = state.db.start_transaction().await?;
    let new_task = TaskDatabase::create_from(&payload.bind(category_id), &mut *connection).await?;
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

    Ok(StatusCode::CREATED)
}

pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Extension(access_claims): Extension<AccessClaims>,
) -> Result<StatusCode, ControllerError> {
    let user_id = access_claims.sub.parse().unwrap();
    let mut connection = state.db.start_transaction().await?;

    let validated_id = TaskDeleteDto::from(id)
        .validate(user_id, &mut *connection)
        .await?;

    TaskDatabase::delete_by_id(validated_id, &mut *connection).await?;

    connection.commit().await?;

    Ok(StatusCode::OK)
}

pub async fn update(
    State(state): State<AppState>,
    Path((_category_id, task_id)): Path<(String, String)>,
    Extension(access_claims): Extension<AccessClaims>,
    Json(payload): Json<TaskUpdateDto>,
) -> Result<StatusCode, ControllerError> {
    let user_id = access_claims.sub.parse().unwrap();
    let mut connection = state.db.start_transaction().await?;

    let validated_params = payload
        .bind(task_id)
        .validate(user_id, &mut *connection)
        .await?;

    TaskDatabase::update(&validated_params, &mut *connection).await?;

    connection.commit().await?;

    Ok(StatusCode::OK)
}
