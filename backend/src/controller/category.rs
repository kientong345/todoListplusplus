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
        category::{
            dto::{
                CategoryCreateDto, CategoryDeleteDto, CategoryDetailDto, CategoryMinimalDto,
                CategorySearchDto, CategoryUpdateDto,
            },
            CategoryDatabase, CategoryDetail, CategoryMinimal,
        },
        pagination::Paginate,
        user_auth::AccessClaims,
    },
};

pub async fn get_page(
    State(state): State<AppState>,
    Query(query): Query<CategorySearchDto>,
    Extension(access_claims): Extension<AccessClaims>,
) -> Result<Json<Value>, ControllerError> {
    let user_id = access_claims.sub.parse().unwrap_or(-1);
    let mut connection = state.db.start_transaction().await?;

    let page = CategoryMinimal::page(&query.bind(user_id), &mut *connection)
        .await?
        .map_into::<CategoryMinimalDto>();

    connection.commit().await?;

    Ok(Json(json!(page)))
}

pub async fn find_by_id(
    State(state): State<AppState>,
    Path(category_id): Path<i32>,
    Extension(_access_claims): Extension<AccessClaims>,
) -> Result<Json<Value>, ControllerError> {
    let mut connection = state.db.start_transaction().await?;

    let category: CategoryDetailDto = CategoryDetail::get_by_id(category_id, &mut *connection)
        .await?
        .into();

    connection.commit().await?;

    Ok(Json(json!(category)))
}

pub async fn create(
    State(state): State<AppState>,
    Extension(access_claims): Extension<AccessClaims>,
    Json(payload): Json<CategoryCreateDto>,
) -> Result<StatusCode, ControllerError> {
    let user_id = access_claims.sub.parse().unwrap_or(-1);
    let mut connection = state.db.start_transaction().await?;
    CategoryDatabase::create_from(&payload.bind(user_id), &mut *connection).await?;
    connection.commit().await?;

    Ok(StatusCode::CREATED)
}

pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Extension(access_claims): Extension<AccessClaims>,
) -> Result<StatusCode, ControllerError> {
    let user_id = access_claims.sub.parse().unwrap_or(-1);
    let mut connection = state.db.start_transaction().await?;

    let validated_id = CategoryDeleteDto::from(id)
        .validate(user_id, &mut *connection)
        .await?;

    CategoryDatabase::delete_by_id(validated_id, &mut *connection).await?;

    connection.commit().await?;

    Ok(StatusCode::OK)
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Extension(access_claims): Extension<AccessClaims>,
    Json(payload): Json<CategoryUpdateDto>,
) -> Result<StatusCode, ControllerError> {
    let user_id = access_claims.sub.parse().unwrap_or(-1);
    let mut connection = state.db.start_transaction().await?;

    let validated_params = payload.bind(id).validate(user_id, &mut *connection).await?;

    CategoryDatabase::update(&validated_params, &mut *connection).await?;

    connection.commit().await?;

    Ok(StatusCode::OK)
}
