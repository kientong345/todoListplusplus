use async_trait::async_trait;

use crate::service::error::ServiceError;

#[derive(Debug, Clone)]
pub enum RecipientIdentifier {
    Messenger { psid: String },
    Gmail { gmail: String },
    Phone { phone: String },
}

#[async_trait]
pub trait MessageClient: Sync + Send {
    async fn send(
        &self,
        recipient_identifier: RecipientIdentifier,
        message: String,
    ) -> Result<(), ServiceError>;

    async fn receive(&self) -> Result<(RecipientIdentifier, String), ServiceError>;

    async fn is_available(&self) -> bool;

    async fn is_receivable(&self) -> bool;
}

#[derive(Debug, Clone)]
pub struct GmailClient {
    client: reqwest::Client,
    server_url: String,
}

impl GmailClient {
    pub fn new(server_url: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            server_url: server_url.to_string(),
        }
    }
}

#[async_trait]
impl MessageClient for GmailClient {
    async fn send(
        &self,
        recipient_identifier: RecipientIdentifier,
        message: String,
    ) -> Result<(), ServiceError> {
        let recipient_gmail = match recipient_identifier {
            RecipientIdentifier::Gmail { gmail } => gmail,
            _ => {
                return Err(ServiceError::MessageClientError(
                    "Unsupported recipient identifier".to_string(),
                ))
            }
        };

        #[derive(serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct SendGmailRequest {
            recipient_gmail: String,
            message: String,
        }

        let req = SendGmailRequest {
            recipient_gmail,
            message,
        };

        let server_endpoint = format!("{}{}", self.server_url, "/message/gmail");
        let res = self.client.post(&server_endpoint).json(&req).send().await?;

        if !res.status().is_success() {
            return Err(ServiceError::MessageClientError(
                "Failed to send message".to_string(),
            ));
        }

        Ok(())
    }

    async fn receive(&self) -> Result<(RecipientIdentifier, String), ServiceError> {
        todo!()
    }

    async fn is_available(&self) -> bool {
        todo!()
    }

    async fn is_receivable(&self) -> bool {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct MessengerClient {
    client: reqwest::Client,
    server_url: String,
}

impl MessengerClient {
    pub fn new(server_url: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            server_url: server_url.to_string(),
        }
    }
}

#[async_trait]
impl MessageClient for MessengerClient {
    async fn send(
        &self,
        recipient_identifier: RecipientIdentifier,
        message: String,
    ) -> Result<(), ServiceError> {
        let receiver_id = match recipient_identifier {
            RecipientIdentifier::Messenger { psid } => psid,
            _ => {
                return Err(ServiceError::MessageClientError(
                    "Unsupported recipient identifier".to_string(),
                ))
            }
        };

        #[derive(serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct SendMessengerRequest {
            receiver_id: String,
            message: String,
        }

        let req = SendMessengerRequest {
            receiver_id,
            message,
        };

        let server_endpoint = format!("{}{}", self.server_url, "/message/facebook");
        let res = self.client.post(&server_endpoint).json(&req).send().await?;

        if !res.status().is_success() {
            return Err(ServiceError::MessageClientError(
                "Failed to send message".to_string(),
            ));
        }

        Ok(())
    }

    async fn receive(&self) -> Result<(RecipientIdentifier, String), ServiceError> {
        todo!()
    }

    async fn is_available(&self) -> bool {
        todo!()
    }

    async fn is_receivable(&self) -> bool {
        todo!()
    }
}
