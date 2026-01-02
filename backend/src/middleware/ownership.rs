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
    model::{category::CategoryDatabase, task::TaskDatabase, user_auth::AccessClaims},
};

pub async fn category_ownership_middleware(
    State(state): State<AppState>,
    Path(category_id): Path<String>,
    req: Request<Body>,
    Extension(access_claims): Extension<AccessClaims>,
    next: Next,
) -> Result<Response, StatusCode> {
    let user_id: Uuid = access_claims.sub.parse().unwrap();
    let category_id = Uuid::parse_str(&category_id).unwrap();
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

    if owner_id != user_id {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(next.run(req).await)
}

pub async fn task_ownership_middleware(
    State(state): State<AppState>,
    Path((_category_id, task_id)): Path<(String, String)>,
    req: Request<Body>,
    Extension(access_claims): Extension<AccessClaims>,
    next: Next,
) -> Result<Response, StatusCode> {
    let user_id: Uuid = access_claims.sub.parse().unwrap();
    let task_id = Uuid::parse_str(&task_id).unwrap();
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

    if owner_id != user_id {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(next.run(req).await)
}
