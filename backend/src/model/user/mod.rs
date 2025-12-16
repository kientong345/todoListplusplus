use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;

pub mod dto;
pub mod get;
pub mod patch;
pub mod post;

#[derive(Debug, Clone, FromRow)]
pub struct UserDatabase {
    pub id: i32,
    pub google_id: Option<String>,
    pub display_name: String,
    pub email: String,
    pub password_hash: Option<String>,
    pub avatar_url: Option<String>,
    pub description: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct UserInfo {
    pub id: i32,
    pub display_name: String,
    pub email: String,
    pub avatar_url: Option<String>,
    pub description: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct UserCreateParams {
    pub google_id: Option<String>,
    pub display_name: String,
    pub email: String,
    pub password_hash: Option<String>,
    pub avatar_url: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UserUpdateParams {
    pub id: i32,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub description: Option<String>,
}
