use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

use crate::{model::error::ModelError, service::error::ServiceError};

#[derive(Error, Debug)]
pub enum ControllerError {
    #[error("model error: {0}")]
    Model(#[from] ModelError),

    #[error("service error: {0}")]
    Service(#[from] ServiceError),

    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("register error: {0}")]
    InvalidRegistration(String),

    #[error("login error: {0}")]
    InvalidLoginForm(String),

    #[error("token exchange error: {0}")]
    TokenExchange(#[from] reqwest::Error),
}

impl ControllerError {
    fn get_status_code(&self) -> (StatusCode, u16) {
        match self {
            ControllerError::Model(e) => {
                let err_code = e.get_code();
                (
                    StatusCode::from_u16(err_code / 100).unwrap_or_default(),
                    err_code,
                )
            }
            ControllerError::Service(e) => {
                let err_code = e.get_code();
                (
                    StatusCode::from_u16(err_code / 100).unwrap_or_default(),
                    err_code,
                )
            }
            ControllerError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, 50010),
            ControllerError::InvalidRegistration(_) => (StatusCode::BAD_REQUEST, 40010),
            ControllerError::InvalidLoginForm(_) => (StatusCode::BAD_REQUEST, 40011),
            ControllerError::TokenExchange(_) => (StatusCode::INTERNAL_SERVER_ERROR, 50011),
        }
    }
}

impl IntoResponse for ControllerError {
    fn into_response(self) -> Response {
        let (status_code, code) = self.get_status_code();
        let message = self.to_string();
        let body = Json(json!({ "code": code, "message": message }));

        (status_code, body).into_response()
    }
}
