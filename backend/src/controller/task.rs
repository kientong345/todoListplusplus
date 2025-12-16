use axum::{
    extract::{Path, Query, State},
    Extension, Json,
};
use reqwest::StatusCode;
use serde_json::{json, Value};

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
};

pub async fn get_page(
    State(state): State<AppState>,
    Query(query): Query<TaskSearchDto>,
    Extension(access_claims): Extension<AccessClaims>,
) -> Result<Json<Value>, ControllerError> {
    let user_id = access_claims.sub.parse().unwrap_or(-1);
    let mut connection = state.db.start_transaction().await?;

    let page = TaskMinimal::page(&query.bind(user_id), &mut *connection)
        .await?
        .map_into::<TaskMinimalDto>();

    connection.commit().await?;

    Ok(Json(json!(page)))
}

pub async fn find_by_id(
    State(state): State<AppState>,
    Path(_category_id): Path<i32>,
    Path(task_id): Path<i32>,
    Extension(_access_claims): Extension<AccessClaims>,
) -> Result<Json<Value>, ControllerError> {
    let mut connection = state.db.start_transaction().await?;

    let task: TaskDetailDto = TaskDetail::get_by_id(task_id, &mut *connection)
        .await?
        .into();

    connection.commit().await?;

    Ok(Json(json!(task)))
}

pub async fn create(
    State(state): State<AppState>,
    Path(category_id): Path<i32>,
    Extension(access_claims): Extension<AccessClaims>,
    Json(payload): Json<TaskCreateDto>,
) -> Result<StatusCode, ControllerError> {
    let _user_id = access_claims.sub.parse().unwrap_or(-1);
    let mut connection = state.db.start_transaction().await?;
    TaskDatabase::create_from(&payload.bind(category_id), &mut *connection).await?;
    connection.commit().await?;

    Ok(StatusCode::CREATED)
}

pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Extension(access_claims): Extension<AccessClaims>,
) -> Result<StatusCode, ControllerError> {
    let _user_id = access_claims.sub.parse().unwrap_or(-1);
    let mut connection = state.db.start_transaction().await?;

    let validated_id = TaskDeleteDto::from(id)
        .validate(_user_id, &mut *connection)
        .await?;

    TaskDatabase::delete_by_id(validated_id, &mut *connection).await?;

    connection.commit().await?;

    Ok(StatusCode::OK)
}

pub async fn update(
    State(state): State<AppState>,
    Path(_category_id): Path<i32>,
    Path(task_id): Path<i32>,
    Extension(access_claims): Extension<AccessClaims>,
    Json(payload): Json<TaskUpdateDto>,
) -> Result<StatusCode, ControllerError> {
    let _user_id = access_claims.sub.parse().unwrap_or(-1);
    let mut connection = state.db.start_transaction().await?;

    let validated_params = payload
        .bind(task_id)
        .validate(_user_id, &mut *connection)
        .await?;

    TaskDatabase::update(&validated_params, &mut *connection).await?;

    connection.commit().await?;

    Ok(StatusCode::OK)
}
