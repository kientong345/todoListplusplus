use crate::model::{
    error::ModelError,
    user::{UserCreateParams, UserDatabase},
};

impl UserDatabase {
    pub async fn create_from(
        params: &UserCreateParams,
        connection: &mut sqlx::PgConnection,
    ) -> Result<UserDatabase, ModelError> {
        let user = sqlx::query_as!(
            UserDatabase,
            r#"INSERT INTO users (
                usr_google_id, usr_display_name, usr_email, usr_password_hash,
                usr_avatar_url, usr_description
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING
                usr_id AS id,
                usr_google_id AS google_id,
                usr_display_name AS display_name,
                usr_email AS email,
                usr_password_hash AS password_hash,
                usr_avatar_url AS avatar_url,
                usr_description AS description,
                usr_created_at AS created_at,
                usr_updated_at AS updated_at"#,
            params.google_id,
            params.display_name,
            params.email,
            params.password_hash,
            params.avatar_url,
            params.description,
        )
        .fetch_one(connection)
        .await?;

        Ok(user)
    }
}
