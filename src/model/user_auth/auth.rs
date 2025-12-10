use serde::{Deserialize, Serialize};

use crate::{model::error::ModelError, utils::validate_email_name};

#[derive(Debug, Clone, Deserialize)]
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

#[derive(Debug, Clone, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessClaims {
    pub sub: String,
    // pub role: String,
    pub exp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshClaims {
    pub sub: String,
    pub exp: i64,
}
