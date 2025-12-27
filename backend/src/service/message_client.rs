#[derive(Clone)]
pub struct MessageClient {
    client: reqwest::Client,
}

impl MessageClient {
    pub fn new() -> Self {
        Self { client: reqwest::Client::new() }
    }
}
