use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct OAuthSchema {
    pub google_id: String,
    pub display_name: String,
    pub email: String,
    pub avatar_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct OAuthResponse {
    pub access_token: String,
    pub expires_in: i32,
    pub refresh_token: String,
    pub scope: String,
    pub token_type: String,
    pub id_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct GoogleUserResult {
    pub id: String,
    pub email: String,
    pub verified_email: bool,
    pub name: String,
    pub given_name: String,
    pub family_name: String,
    pub picture: String,
}

impl Into<OAuthSchema> for GoogleUserResult {
    fn into(self) -> OAuthSchema {
        OAuthSchema {
            google_id: self.id,
            display_name: self.name,
            email: self.email,
            avatar_url: self.picture,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AuthorizationCode {
    pub code: String,
}
