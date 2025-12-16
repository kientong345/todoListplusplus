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
};

pub async fn get_me(
    State(state): State<AppState>,
    Extension(access_claims): Extension<AccessClaims>,
) -> Result<(StatusCode, Json<UserInfoDto>), ControllerError> {
    let user_id = access_claims.sub.parse().unwrap_or(-1);
    let mut connection = state.db.get_connection().await?;
    let user = UserInfo::get_by_id(user_id, &mut *connection).await?;
    Ok((StatusCode::OK, Json(user.into())))
}

pub async fn update_me(
    State(state): State<AppState>,
    Extension(access_claims): Extension<AccessClaims>,
    Json(payload): Json<UserUpdateDto>,
) -> Result<StatusCode, ControllerError> {
    let user_id = access_claims.sub.parse().unwrap_or(-1);
    let mut connection = state.db.start_transaction().await?;
    UserDatabase::update(&payload.bind(user_id), &mut *connection).await?;
    connection.commit().await?;
    Ok(StatusCode::OK)
}
