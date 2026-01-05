use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct OAuthSchema {
    /// Unique ID from the OAuth provider (e.g., Google ID)
    #[schema(example = "109876543210987654321")]
    pub google_id: String,

    /// User's display name from provider
    #[schema(example = "Google User")]
    pub display_name: String,

    /// User's email from provider
    #[schema(example = "user@gmail.com")]
    pub email: String,

    /// Avatar URL from provider
    #[schema(example = "https://lh3.googleusercontent.com/a/abc")]
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
    /// The authorization code received from Google OAuth flow
    #[schema(example = "4/0Af...")]
    pub code: String,
}
