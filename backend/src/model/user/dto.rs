use std::str::FromStr;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    model::user::{UserInfo, UserUpdateParams},
    utils::datetime_to_string,
};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserInfoDto {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub id: String,

    #[schema(example = "My Display Name")]
    pub display_name: String,

    #[schema(example = "myemail@example.com")]
    pub email: String,

    #[schema(example = "https://example.com/avatar.jpg")]
    pub avatar_url: Option<String>,

    #[schema(example = "This is a user description")]
    pub description: Option<String>,

    #[schema(example = "2022-01-01T00:00:00.000Z")]
    pub created_at: Option<String>,

    #[schema(example = "2022-01-01T00:00:00.000Z")]
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

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserUpdateDto {
    #[schema(example = "My Display Name")]
    pub display_name: Option<String>,

    #[schema(example = "https://example.com/avatar.jpg")]
    pub avatar_url: Option<String>,

    #[schema(example = "This is a user description")]
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
