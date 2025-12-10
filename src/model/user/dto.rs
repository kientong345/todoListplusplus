use serde::{Deserialize, Serialize};

use crate::model::user::{UserCreateParams, UserInfo, UserUpdateParams};

#[derive(Debug, Clone, Serialize)]
pub struct UserInfoDto {
    pub id: i32,
    pub display_name: String,
    pub email: String,
    pub avatar_url: Option<String>,
    pub description: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl From<UserInfo> for UserInfoDto {
    fn from(value: UserInfo) -> Self {
        Self {
            id: value.id,
            display_name: value.display_name,
            email: value.email,
            avatar_url: value.avatar_url,
            description: value.description,
            created_at: value.created_at.map(|dt| dt.to_rfc3339()),
            updated_at: value.updated_at.map(|dt| dt.to_rfc3339()),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserCreateDto {
    pub google_id: Option<String>,
    pub display_name: String,
    pub email: String,
    pub password_hash: Option<String>,
    pub avatar_url: Option<String>,
    pub description: Option<String>,
}

impl Into<UserCreateParams> for UserCreateDto {
    fn into(self) -> UserCreateParams {
        UserCreateParams {
            google_id: self.google_id,
            display_name: self.display_name,
            email: self.email,
            password_hash: self.password_hash,
            avatar_url: self.avatar_url,
            description: self.description,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserUpdateDto {
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub description: Option<String>,
}

impl Into<UserUpdateParams> for UserUpdateDto {
    fn into(self) -> UserUpdateParams {
        UserUpdateParams {
            display_name: self.display_name,
            avatar_url: self.avatar_url,
            description: self.description,
        }
    }
}
