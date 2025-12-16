use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct OAuthConfig {
    pub google_client_id: String,
    pub google_client_secret: String,
    pub auth_url: String,
    pub token_url: String,
    pub redirect_url: String,
    pub user_info_url: String,
}

impl OAuthConfig {
    pub fn get() -> OAuthConfig {
        OAuthConfig {
            google_client_id: std::env::var("GOOGLE_CLIENT_ID")
                .expect("GOOGLE_CLIENT_ID is not set"),
            google_client_secret: std::env::var("GOOGLE_CLIENT_SECRET")
                .expect("GOOGLE_CLIENT_SECRET is not set"),
            auth_url: "https://accounts.google.com/o/oauth2/v2/auth".to_string(),
            token_url: "https://oauth2.googleapis.com/token".to_string(),
            redirect_url: "http://localhost:3000/oauth/login".to_string(),
            user_info_url: "https://www.googleapis.com/oauth2/v1/userinfo".to_string(),
        }
    }
}
