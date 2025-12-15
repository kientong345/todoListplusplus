#[derive(Clone)]
pub struct EmailClient {
    client: reqwest::Client,
}

impl EmailClient {
    pub fn new() -> Self {
        Self { client: reqwest::Client::new() }
    }
}
