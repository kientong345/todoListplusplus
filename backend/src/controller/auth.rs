use axum::{
    extract::{Query, State},
    Json,
};
use axum_extra::extract::{
    cookie::{Cookie, SameSite},
    CookieJar,
};
use reqwest::StatusCode;
use serde_json::{json, Value};

use crate::{
    app::AppState,
    controller::error::ControllerError,
    model::user_auth::{
        auth::{LoginSchema, RegisterSchema},
        oauth::AuthorizationCode,
    },
    service::oauth_client::OAuthClient,
};

#[utoipa::path(
    post,
    path = "/auth/register",
    tag = "auth",
    request_body = RegisterSchema,
    responses(
        (status = 201, description = "Registration successful"),
        (status = 400, description = "Invalid input"),
    ),
)]
pub async fn handle_register(
    State(state): State<AppState>,
    Json(registration_data): Json<RegisterSchema>,
) -> Result<StatusCode, ControllerError> {
    let mut connection = state.db.start_transaction().await?;

    registration_data.validate()?;

    state
        .auth_service
        .register(&mut *connection, registration_data)
        .await?;

    connection.commit().await?;

    Ok(StatusCode::CREATED)
}

#[utoipa::path(
    post,
    path = "/auth/login",
    tag = "auth",
    request_body = LoginSchema,
    responses(
        (status = 200, description = "Login successful", body = inline(serde_json::Value), example = json!({"access_token": "jwt_token_here"})),
        (status = 401, description = "Invalid credentials"),
    ),
)]
pub async fn handle_login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(login_form): Json<LoginSchema>,
) -> Result<(CookieJar, Json<Value>), ControllerError> {
    let mut connection = state.db.get_connection().await?;

    login_form.validate()?;

    let (_, access_token, refresh_token) = state
        .auth_service
        .login(&mut *connection, login_form)
        .await?;

    let cookie: Cookie = Cookie::build(("refresh_token", refresh_token))
        .http_only(true)
        .secure(false)
        .same_site(SameSite::Lax)
        .path("/")
        .into();

    Ok((
        jar.add(cookie),
        Json(json!({
            "access_token": access_token,
        })),
    ))
}

#[utoipa::path(
    post,
    path = "/auth/google/login",
    tag = "auth",
    params(
        ("code" = String, Query, description = "Authorization code from Google"),
    ),
    responses(
        (status = 200, description = "Login successful", body = inline(serde_json::Value), example = json!({"access_token": "jwt_token_here"})),
        (status = 401, description = "Invalid authorization code"),
    ),
)]
pub async fn handle_google_login(
    State(state): State<AppState>,
    Query(auth_code): Query<AuthorizationCode>,
) -> Result<(CookieJar, Json<Value>), ControllerError> {
    let oauth_client = OAuthClient::init(&state.config.oauth_config);

    let token_response = oauth_client.exchange_for_token(&auth_code.code).await?;

    let google_user = oauth_client
        .get_google_user(&token_response.access_token, &token_response.id_token)
        .await?;

    let mut connection = state.db.start_transaction().await?;

    let (_, access_token, refresh_token) = state
        .auth_service
        .google_login(&mut *connection, google_user.into())
        .await?;

    connection.commit().await?;

    let cookie: Cookie = Cookie::build(("refresh_token", refresh_token))
        .http_only(true)
        .secure(false)
        .same_site(SameSite::Lax)
        .path("/")
        .into();

    let jar = CookieJar::new();

    Ok((
        jar.add(cookie),
        Json(json!({
            "access_token": access_token,
        })),
    ))
}

#[utoipa::path(
    post,
    path = "/auth/refresh",
    tag = "auth",
    responses(
        (status = 200, description = "Token refreshed", body = inline(serde_json::Value), example = json!({"access_token": "jwt_token_here"})),
        (status = 401, description = "Invalid or expired refresh token"),
    ),
)]
pub async fn handle_refresh(
    State(state): State<AppState>,
    jar: CookieJar,
) -> Result<(CookieJar, Json<Value>), ControllerError> {
    let old_refresh_token = jar
        .get("refresh_token")
        .ok_or(ControllerError::Unauthorized)?
        .value()
        .to_string();

    let mut connection = state.db.start_transaction().await?;
    let (access_token, refresh_token) = state
        .auth_service
        .refresh(&mut *connection, old_refresh_token)
        .await?;
    connection.commit().await?;

    let cookie: Cookie = Cookie::build(("refresh_token", refresh_token))
        .http_only(true)
        .secure(false)
        .same_site(SameSite::Lax)
        .path("/")
        .into();
    Ok((
        jar.add(cookie),
        Json(json!({
            "access_token": access_token,
        })),
    ))
}

#[utoipa::path(
    post,
    path = "/auth/logout",
    tag = "auth",
    responses(
        (status = 200, description = "Logout successful", body = inline(serde_json::Value), example = json!({"message": "Logout successful"})),
    ),
)]
pub async fn handle_logout(
    State(_state): State<AppState>,
    jar: CookieJar,
) -> Result<(CookieJar, Json<Value>), ControllerError> {
    let remove_cookie: Cookie = Cookie::build(("refresh_token", ""))
        .http_only(true)
        .secure(false)
        .same_site(SameSite::Lax)
        .path("/")
        .into();
    Ok((
        jar.add(remove_cookie),
        Json(json!({ "message": "Logout successful" })),
    ))
}
