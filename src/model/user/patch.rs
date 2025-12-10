use chrono::Utc;

use crate::model::{
    error::ModelError,
    user::{UserDatabase, UserUpdateParams},
};

impl UserDatabase {
    pub async fn update(
        &self,
        params: &UserUpdateParams,
        connection: &mut sqlx::PgConnection,
    ) -> Result<UserDatabase, ModelError> {
        let user = sqlx::query_as!(
            UserDatabase,
            r#"
            UPDATE users
            SET
                usr_display_name = $2,
                usr_avatar_url = $3,
                usr_description = $4,
                usr_updated_at = $5
            WHERE usr_id = $1
            RETURNING
                usr_id AS id,
                usr_google_id AS google_id,
                usr_display_name AS display_name,
                usr_email AS email,
                usr_password_hash AS password_hash,
                usr_avatar_url AS avatar_url,
                usr_description AS description,
                usr_created_at AS created_at,
                usr_updated_at AS updated_at
            "#,
            self.id,
            params.display_name,
            params.avatar_url,
            params.description,
            Utc::now(),
        )
        .fetch_one(connection)
        .await?;

        Ok(user)
    }
}
