use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub access_token_expire_min: i64,
    pub refresh_token_expire_min: i64,
}

impl AuthConfig {
    pub fn get() -> AuthConfig {
        AuthConfig {
            jwt_secret: std::env::var("JWT_SECRET").expect("JWT_SECRET is not set"),
            access_token_expire_min: 15,
            refresh_token_expire_min: 7 * 24 * 60,
        }
    }
}
