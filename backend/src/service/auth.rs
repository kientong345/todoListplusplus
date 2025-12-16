use serde::de::DeserializeOwned;
use sqlx::PgConnection;

use crate::{
    config::auth::AuthConfig,
    model::{
        user::{UserCreateParams, UserDatabase},
        user_auth::{
            auth::{LoginSchema, RegisterSchema},
            oauth::OAuthSchema,
            AccessClaims, RefreshClaims,
        },
    },
    service::error::ServiceError,
    utils::{bcrypt_hash, decode_jwt, generate_jwt},
};

#[derive(Clone)]
pub struct AuthService {
    config: AuthConfig,
}

impl AuthService {
    pub fn new(config: AuthConfig) -> Self {
        Self { config }
    }

    pub fn decode_jwt<C: Clone + DeserializeOwned>(&self, jwt: &str) -> Result<C, ServiceError> {
        let secret = self.config.jwt_secret.as_bytes().to_vec();
        Ok(decode_jwt::<C>(jwt, &secret)?)
    }

    pub fn generate_token_pair(&self, user: &UserDatabase) -> (String, String) {
        let now = chrono::Utc::now();
        let secret = self.config.jwt_secret.as_bytes().to_vec();

        let access_claims = AccessClaims {
            sub: user.id.to_string(),
            // role: user.role.to_string(),
            exp: (now + chrono::Duration::minutes(self.config.access_token_expire_min)).timestamp(),
        };

        let access_token = generate_jwt(&access_claims, &secret);

        let refresh_claims = RefreshClaims {
            sub: user.id.to_string(),
            exp: (now + chrono::Duration::minutes(self.config.refresh_token_expire_min))
                .timestamp(),
        };

        let refresh_token = generate_jwt(&refresh_claims, &secret);

        (access_token, refresh_token)
    }

    pub async fn register(
        &self,
        conn: &mut PgConnection,
        registration: RegisterSchema,
    ) -> Result<UserDatabase, ServiceError> {
        if UserDatabase::is_email_exist(&registration.email, conn).await? {
            return Err(ServiceError::EmailTaken {
                email: registration.email,
            });
        }
        let password_hash = bcrypt_hash(&registration.password)?;
        let post_user = UserCreateParams {
            google_id: None,
            display_name: registration.display_name,
            email: registration.email,
            password_hash: Some(password_hash),
            avatar_url: None,
            description: None,
            // role: None, // default is "user"
        };
        let user = UserDatabase::create_from(&post_user, conn).await?;
        Ok(user)
    }

    pub async fn login(
        &self,
        conn: &mut PgConnection,
        login_form: LoginSchema,
    ) -> Result<(UserDatabase, String, String), ServiceError> {
        let user =
            UserDatabase::validate_login(&login_form.email, &login_form.password, conn).await?;

        let (access_token, refresh_token) = self.generate_token_pair(&user);

        Ok((user, access_token, refresh_token))
    }

    pub async fn google_login(
        &self,
        conn: &mut PgConnection,
        oauth: OAuthSchema,
    ) -> Result<(UserDatabase, String, String), ServiceError> {
        let user = if !UserDatabase::is_email_exist(&oauth.email, conn).await? {
            let post_user = UserCreateParams {
                google_id: Some(oauth.google_id),
                display_name: oauth.display_name,
                email: oauth.email,
                password_hash: None,
                avatar_url: None,
                description: None,
                // role: None, // default is "user"
            };
            UserDatabase::create_from(&post_user, conn).await?
        } else {
            UserDatabase::get_by_email(&oauth.email, conn).await?
        };

        let (access_token, refresh_token) = self.generate_token_pair(&user);

        Ok((user, access_token, refresh_token))
    }
}
