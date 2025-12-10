use reqwest::Url;

use crate::{
    config::oauth::OAuthConfig,
    model::user_auth::oauth::{GoogleUserResult, OAuthResponse},
    service::error::ServiceError,
};

pub struct OAuthClient {
    client: reqwest::Client,
    config: OAuthConfig,
}

impl OAuthClient {
    pub fn init(config: &OAuthConfig) -> OAuthClient {
        OAuthClient {
            client: reqwest::Client::new(),
            config: config.clone(),
        }
    }

    pub async fn exchange_for_token(
        &self,
        authorization_code: &str,
    ) -> Result<OAuthResponse, ServiceError> {
        /*
        Gemini 3 Pro:
        "Tôi là ứng dụng A (client_id),
        tôi có mật khẩu B (client_secret).
        Người dùng đã đồng ý và đưa cho tôi mã C (code) thông qua địa chỉ D (redirect_uri).
        Vì tôi dùng quy trình chuẩn E (grant_type), hãy đưa cho tôi chìa khóa (access_token) để vào nhà."
        */
        let params = [
            ("grant_type", "authorization_code"),
            ("redirect_uri", self.config.redirect_url.as_str()),
            ("client_id", self.config.google_client_id.as_str()),
            ("code", authorization_code),
            ("client_secret", self.config.google_client_secret.as_str()),
        ];
        let response = self
            .client
            .post(self.config.token_url.as_str())
            .form(&params)
            .send()
            .await?;

        let oauth_response = response.json::<OAuthResponse>().await?;
        Ok(oauth_response)
    }

    pub async fn get_google_user(
        &self,
        access_token: &str,
        id_token: &str,
    ) -> Result<GoogleUserResult, ServiceError> {
        let mut url = Url::parse(&self.config.user_info_url).unwrap();
        url.query_pairs_mut().append_pair("alt", "json");
        url.query_pairs_mut()
            .append_pair("access_token", access_token);

        let response = self.client.get(url).bearer_auth(id_token).send().await?;

        let user_info = response.json::<GoogleUserResult>().await?;
        Ok(user_info)
    }
}
