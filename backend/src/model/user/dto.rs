use std::str::FromStr;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    model::user::{UserInfo, UserUpdateParams},
    utils::datetime_to_string,
};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfoDto {
    pub id: String,
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
            id: value.id.to_string(),
            display_name: value.display_name,
            email: value.email,
            avatar_url: value.avatar_url,
            description: value.description,
            created_at: value.created_at.map(|dt| datetime_to_string(dt)),
            updated_at: value.updated_at.map(|dt| datetime_to_string(dt)),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserUpdateDto {
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub description: Option<String>,
}

impl UserUpdateDto {
    pub fn bind(self, id: String) -> UserUpdateParams {
        UserUpdateParams {
            id: Uuid::from_str(&id).unwrap(),
            display_name: self.display_name,
            avatar_url: self.avatar_url,
            description: self.description,
        }
    }
}
