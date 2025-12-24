use uuid::Uuid;

use crate::model::{
    error::ModelError,
    user::{UserDatabase, UserUpdateParams},
};

impl UserDatabase {
    pub async fn update(
        params: &UserUpdateParams,
        connection: &mut sqlx::PgConnection,
    ) -> Result<UserDatabase, ModelError> {
        if let Some(display_name) = &params.display_name {
            Self::update_display_name(params.id, display_name, connection).await?;
        }
        if let Some(avatar_url) = &params.avatar_url {
            Self::update_avatar_url(params.id, avatar_url, connection).await?;
        }
        if let Some(description) = &params.description {
            Self::update_description(params.id, description, connection).await?;
        }

        let user = Self::get_by_id(params.id, connection).await?;

        Ok(user)
    }

    async fn update_display_name(
        user_id: Uuid,
        display_name: &str,
        connection: &mut sqlx::PgConnection,
    ) -> Result<(), ModelError> {
        sqlx::query!(
            r#"UPDATE users
            SET usr_display_name = $2
            WHERE usr_id = $1"#,
            user_id,
            display_name,
        )
        .execute(connection)
        .await?;
        Ok(())
    }

    async fn update_avatar_url(
        user_id: Uuid,
        avatar_url: &str,
        connection: &mut sqlx::PgConnection,
    ) -> Result<(), ModelError> {
        sqlx::query!(
            r#"UPDATE users
            SET usr_avatar_url = $2
            WHERE usr_id = $1"#,
            user_id,
            avatar_url,
        )
        .execute(connection)
        .await?;
        Ok(())
    }

    async fn update_description(
        user_id: Uuid,
        description: &str,
        connection: &mut sqlx::PgConnection,
    ) -> Result<(), ModelError> {
        sqlx::query!(
            r#"UPDATE users
            SET usr_description = $2
            WHERE usr_id = $1"#,
            user_id,
            description,
        )
        .execute(connection)
        .await?;
        Ok(())
    }
}
