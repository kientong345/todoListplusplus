use axum::{
    body::Body,
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::{app::AppState, model::user_auth::auth::AccessClaims};

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Response {
    let auth_header = match req.headers().get("Authorization") {
        Some(value) => value,
        None => return StatusCode::NON_AUTHORITATIVE_INFORMATION.into_response(),
    };

    let access_token = auth_header
        .to_str()
        .unwrap()
        .strip_prefix("Bearer ")
        .unwrap();

    let access_claims = match state.auth_service.decode_jwt::<AccessClaims>(access_token) {
        Ok(access_claims) => access_claims,
        Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
    };

    req.extensions_mut().insert(access_claims);

    next.run(req).await
}
