use thiserror::Error;

use crate::model::error::ModelError;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("model error: {0}")]
    Model(#[from] ModelError),

    #[error("token exchange error: {0}")]
    TokenExchange(#[from] reqwest::Error),

    #[error("bcrypt error: {0}")]
    Bcrypt(#[from] bcrypt::BcryptError),

    #[error("jwt error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("email already taken: {email}")]
    EmailTaken { email: String },

    #[error("email does not exist: {email}")]
    EmailNotExist { email: String },

    #[error("bad submission: {0}")]
    BadSubmission(String),

    #[error("redis error: {0}")]
    RedisPool(#[from] deadpool_redis::PoolError),

    #[error("redis error: {0}")]
    Redis(#[from] redis::RedisError),
}

impl ServiceError {
    pub fn get_code(&self) -> u16 {
        match self {
            ServiceError::Model(_) => 50013,
            ServiceError::TokenExchange(_) => 50014,
            ServiceError::Bcrypt(_) => 50003,
            ServiceError::Jwt(_) => 50002,
            ServiceError::EmailTaken { .. } => 40001,
            ServiceError::EmailNotExist { .. } => 40002,
            ServiceError::BadSubmission(_) => 40005,
            ServiceError::RedisPool(_) => 50014,
            ServiceError::Redis(_) => 50015,
        }
    }
}
