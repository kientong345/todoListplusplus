use axum::{
    body::Body,
    extract::{Path, Request, State},
    middleware::Next,
    response::Response,
    Extension,
};
use reqwest::StatusCode;
use uuid::Uuid;

use crate::{
    app::AppState,
    infrastructures::cache::DEFAULT_TTL_SECONDS,
    model::{category::CategoryDatabase, task::TaskDatabase, user_auth::AccessClaims},
};

pub async fn category_ownership_check(
    State(state): State<AppState>,
    Path(category_id): Path<String>,
    Extension(access_claims): Extension<AccessClaims>,
    req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let user_id: Uuid = access_claims.sub.parse().unwrap();
    let category_id = Uuid::parse_str(&category_id).unwrap();

    let cache_key = format!("todolist++:categories:{}:owner:{}", category_id, user_id);

    if let Ok(Some(owner_id)) = state.cache.get::<String>(&cache_key).await {
        if owner_id != user_id.to_string() {
            return Err(StatusCode::UNAUTHORIZED);
        } else {
            return Ok(next.run(req).await);
        }
    }

    let mut connection = state
        .db
        .start_transaction()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let owner_id = CategoryDatabase::get_owner_id(category_id, &mut *connection)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    connection
        .commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let _ = state
        .cache
        .set::<String>(&cache_key, &owner_id.to_string(), DEFAULT_TTL_SECONDS)
        .await;

    if owner_id != user_id {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(next.run(req).await)
}

pub async fn task_ownership_check(
    State(state): State<AppState>,
    Path((category_id, task_id)): Path<(String, String)>,
    Extension(access_claims): Extension<AccessClaims>,
    req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let user_id: Uuid = access_claims.sub.parse().unwrap();
    let task_id = Uuid::parse_str(&task_id).unwrap();

    let cache_key = format!(
        "todolist++:categories:{}:tasks:{}:owner:{}",
        category_id, task_id, user_id
    );

    if let Ok(Some(owner_id)) = state.cache.get::<String>(&cache_key).await {
        if owner_id != user_id.to_string() {
            return Err(StatusCode::UNAUTHORIZED);
        } else {
            return Ok(next.run(req).await);
        }
    }

    let mut connection = state
        .db
        .start_transaction()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let owner_id = TaskDatabase::get_owner_id(task_id, &mut *connection)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    connection
        .commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let _ = state
        .cache
        .set::<String>(&cache_key, &owner_id.to_string(), DEFAULT_TTL_SECONDS)
        .await;

    if owner_id != user_id {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(next.run(req).await)
}
