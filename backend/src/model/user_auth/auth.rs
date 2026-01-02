use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{model::error::ModelError, utils::validate_email_name};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegisterSchema {
    pub display_name: String,
    pub email: String,
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
    pub email: String,
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
