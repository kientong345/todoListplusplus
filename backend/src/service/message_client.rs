use async_trait::async_trait;

use crate::service::error::ServiceError;

#[derive(Debug, Clone)]
pub enum UserIdentifier {
    Messenger { psid: String },
    Gmail { gmail: String },
    Phone { phone: String },
}

#[async_trait]
pub trait MessageClient: Sync + Send {
    async fn send(
        &self,
        user_identifier: UserIdentifier,
        message: String,
    ) -> Result<(), ServiceError>;

    async fn receive(&self) -> Result<(UserIdentifier, String), ServiceError>;

    async fn is_available(&self) -> bool;

    async fn is_receivable(&self) -> bool;
}

#[derive(Debug, Clone)]
pub struct GmailClient {
    client: reqwest::Client,
}

impl GmailClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl MessageClient for GmailClient {
    async fn send(
        &self,
        user_identifier: UserIdentifier,
        message: String,
    ) -> Result<(), ServiceError> {
        todo!()
    }

    async fn receive(&self) -> Result<(UserIdentifier, String), ServiceError> {
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
}

impl MessengerClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl MessageClient for MessengerClient {
    async fn send(
        &self,
        user_identifier: UserIdentifier,
        message: String,
    ) -> Result<(), ServiceError> {
        todo!()
    }

    async fn receive(&self) -> Result<(UserIdentifier, String), ServiceError> {
        todo!()
    }

    async fn is_available(&self) -> bool {
        todo!()
    }

    async fn is_receivable(&self) -> bool {
        todo!()
    }
}
