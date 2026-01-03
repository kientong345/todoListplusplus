use axum::{extract::State, Extension, Json};
use reqwest::StatusCode;

use crate::{
    app::AppState,
    controller::error::ControllerError,
    model::{
        user::{
            dto::{UserInfoDto, UserUpdateDto},
            UserDatabase, UserInfo,
        },
        user_auth::AccessClaims,
    },
    service::cache::DEFAULT_TTL_SECONDS,
};

#[utoipa::path(
    get,
    path = "/users/me",
    responses(
        (status = 200, description = "Success", body = UserInfoDto),
    ),
)]
pub async fn get_me(
    State(state): State<AppState>,
    Extension(access_claims): Extension<AccessClaims>,
) -> Result<(StatusCode, Json<UserInfoDto>), ControllerError> {
    let user_id = access_claims.sub.parse().unwrap();
    let cache_key = format!("todolist++:users:{}", user_id);

    if let Ok(Some(user)) = state.cache.get::<UserInfoDto>(&cache_key).await {
        return Ok((StatusCode::OK, Json(user)));
    }

    let mut connection = state.db.get_connection().await?;
    let user: UserInfoDto = UserInfo::get_by_id(user_id, &mut *connection).await?.into();

    let _ = state
        .cache
        .set::<UserInfoDto>(&cache_key, &user, DEFAULT_TTL_SECONDS)
        .await;

    Ok((StatusCode::OK, Json(user)))
}

#[utoipa::path(
    put,
    path = "/users/me",
    request_body = UserUpdateDto,
    responses(
        (status = 200, description = "Success", body = UserInfoDto),
    ),
)]
pub async fn update_me(
    State(state): State<AppState>,
    Extension(access_claims): Extension<AccessClaims>,
    Json(payload): Json<UserUpdateDto>,
) -> Result<StatusCode, ControllerError> {
    let user_id = access_claims.sub.parse().unwrap();
    let cache_key = format!("todolist++:users:{}", user_id);

    let mut connection = state.db.start_transaction().await?;
    UserDatabase::update(&payload.bind(user_id), &mut *connection).await?;
    connection.commit().await?;

    let _ = state.cache.delete(&cache_key).await;

    Ok(StatusCode::OK)
}
