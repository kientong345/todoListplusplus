use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{model::error::ModelError, utils::validate_email_name};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegisterSchema {
    /// Desired display name for the new account
    #[schema(example = "New User")]
    pub display_name: String,

    /// Email address for registration
    #[schema(example = "newuser@example.com")]
    pub email: String,

    /// Password for the new account
    #[schema(example = "StrongPassword123!")]
    pub password: String,
}

impl RegisterSchema {
    pub fn validate(&self) -> Result<&Self, ModelError> {
        if let Err(e) = validate_email_name(&self.email) {
            return Err(ModelError::InvalidAuthSchema(format!(
                "Invalid email name: {}",
                &e
            )));
        }

        Ok(self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LoginSchema {
    /// Registered email address
    #[schema(example = "user@example.com")]
    pub email: String,

    /// Account password
    #[schema(example = "YourPassword123!")]
    pub password: String,
}

impl LoginSchema {
    pub fn validate(&self) -> Result<&Self, ModelError> {
        if let Err(e) = validate_email_name(&self.email) {
            return Err(ModelError::InvalidAuthSchema(format!(
                "Invalid email name: {}",
                &e
            )));
        }

        Ok(self)
    }
}
