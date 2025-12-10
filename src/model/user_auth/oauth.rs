use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct OAuthSchema {
    pub google_id: String,
    pub display_name: String,
    pub email: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct OAuthResponse {
    pub access_token: String,
    pub id_token: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GoogleUserResult {
    pub id: String,
    pub email: String,
    pub verified_email: bool,
    pub name: String,
    pub given_name: String,
    pub family_name: String,
    pub picture: String,
    pub locale: String,
}

impl Into<OAuthSchema> for GoogleUserResult {
    fn into(self) -> OAuthSchema {
        OAuthSchema {
            google_id: self.id,
            display_name: self.name,
            email: self.email,
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizationCode {
    pub code: String,
    pub state: String,
}
