use sqlx::PgConnection;

use crate::model::{
    error::ModelError,
    user::{UserDatabase, UserInfo},
};

impl UserDatabase {
    pub async fn get_by_id(
        user_id: i32,
        connection: &mut PgConnection,
    ) -> Result<UserDatabase, ModelError> {
        let user = sqlx::query_as!(
            UserDatabase,
            r#"SELECT
                usr_id AS id, usr_google_id AS google_id,
                usr_display_name AS display_name,
                usr_email AS email,
                usr_password_hash AS password_hash,
                usr_avatar_url AS avatar_url,
                usr_description AS description,
                usr_created_at AS created_at,
                usr_updated_at AS updated_at
            FROM users WHERE usr_id = $1"#,
            user_id
        )
        .fetch_one(connection)
        .await?;

        Ok(user)
    }

    pub async fn get_by_email(
        email: &str,
        connection: &mut PgConnection,
    ) -> Result<UserDatabase, ModelError> {
        let user = sqlx::query_as!(
            UserDatabase,
            r#"SELECT
                usr_id AS id, usr_google_id AS google_id,
                usr_display_name AS display_name,
                usr_email AS email,
                usr_password_hash AS password_hash,
                usr_avatar_url AS avatar_url,
                usr_description AS description,
                usr_created_at AS created_at,
                usr_updated_at AS updated_at
            FROM users WHERE usr_email = $1"#,
            email
        )
        .fetch_one(connection)
        .await?;

        Ok(user)
    }

    pub async fn is_email_exist(
        email: &str,
        connection: &mut PgConnection,
    ) -> Result<bool, ModelError> {
        Ok(sqlx::query_scalar!(
            r#"SELECT EXISTS (SELECT 1 FROM users WHERE usr_email = $1)"#,
            email
        )
        .fetch_one(connection)
        .await?
        .unwrap_or(false))
    }

    pub async fn validate_login(
        email: &str,
        password: &str,
        connection: &mut PgConnection,
    ) -> Result<UserDatabase, ModelError> {
        let user = UserDatabase::get_by_email(email, connection).await?;
        let hash = user.password_hash.as_deref().unwrap_or("");
        if bcrypt::verify(password, hash).unwrap_or(false) {
            Ok(user)
        } else {
            Err(ModelError::WrongPasswordForEmail { email: user.email })
        }
    }
}

impl UserInfo {
    pub async fn get_by_id(
        user_id: i32,
        connection: &mut PgConnection,
    ) -> Result<UserInfo, ModelError> {
        let user = sqlx::query_as!(
            UserInfo,
            r#"SELECT
                usr_id AS id,
                usr_display_name AS display_name,
                usr_email AS email,
                usr_avatar_url AS avatar_url,
                usr_description AS description,
                usr_created_at AS created_at,
                usr_updated_at AS updated_at
            FROM users WHERE usr_id = $1"#,
            user_id
        )
        .fetch_one(connection)
        .await?;

        Ok(user)
    }

    pub async fn get_by_email(
        email: &str,
        connection: &mut PgConnection,
    ) -> Result<UserInfo, ModelError> {
        let user = sqlx::query_as!(
            UserInfo,
            r#"SELECT
                usr_id AS id,
                usr_display_name AS display_name,
                usr_email AS email,
                usr_avatar_url AS avatar_url,
                usr_description AS description,
                usr_created_at AS created_at,
                usr_updated_at AS updated_at
            FROM users WHERE usr_email = $1"#,
            email
        )
        .fetch_one(connection)
        .await?;

        Ok(user)
    }
}
