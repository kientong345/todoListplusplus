use thiserror::Error;

#[derive(Error, Debug)]
pub enum ModelError {
    #[error("database error: {0}")]
    Sqlx(#[from] sqlx::Error),

    #[error("deserialize error: {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("bad request: {0}")]
    BadRequest(String),

    #[error("permission denied: {0}")]
    PermissionDenied(String),

    #[error("invalid auth schema: {0}")]
    InvalidAuthSchema(String),

    #[error("wrong password for email: {email}")]
    WrongPasswordForEmail { email: String },
}

impl ModelError {
    pub fn get_code(&self) -> u16 {
        match self {
            ModelError::Sqlx(_) => 50001,
            ModelError::SerdeJson(_) => 50005,
            ModelError::BadRequest(_) => 40000,
            ModelError::PermissionDenied(_) => 40300,
            ModelError::InvalidAuthSchema(_) => 40001,
            ModelError::WrongPasswordForEmail { .. } => 40100,
        }
    }
}
