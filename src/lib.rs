use reqwest::Client;
use serde::Deserialize;

const POCKET_API_URL: &str = "https://getpocket.com/v3";

#[derive(Debug, Deserialize)]
pub struct PocketRequestTokenResponse {
    pub code: String,
    pub state: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PocketAccessTokenResponse {
    pub access_token: String,
    pub username: String,
}

#[derive(Clone)] // Add the Clone trait
pub struct PocketSdk {
    consumer_key: String,
    redirect_uri: String,
    client: Client,
}

impl PocketSdk {
    pub fn new(consumer_key: String, redirect_uri: String) -> Self {
        let client = Client::new();
        PocketSdk {
            consumer_key,
            redirect_uri,
            client,
        }
    }

    pub async fn obtain_request_token(&self) -> Result<PocketRequestTokenResponse, reqwest::Error> {
        let url = format!("{}/oauth/request", POCKET_API_URL);

        let params = [
            ("consumer_key", self.consumer_key.as_str()),
            ("redirect_uri", self.redirect_uri.as_str()),
        ];

        let response = self
            .client
            .post(&url)
            .form(&params)
            .send()
            .await?
            .json::<PocketRequestTokenResponse>()
            .await?;

        Ok(response)
    }

    pub fn build_authorization_url(&self, request_token: &str) -> String {
        format!(
            "{}/oauth/authorize?request_token={}&redirect_uri={}",
            POCKET_API_URL, request_token, self.redirect_uri
        )
    }

    pub async fn convert_request_token_to_access_token(
        &self,
        request_token: &str,
    ) -> Result<PocketAccessTokenResponse, reqwest::Error> {
        let url = format!("{}/oauth/authorize", POCKET_API_URL);

        let params = [
            ("consumer_key", self.consumer_key.as_str()),
            ("code", request_token),
        ];

        let response = self
            .client
            .post(&url)
            .form(&params)
            .send()
            .await?
            .json::<PocketAccessTokenResponse>()
            .await?;

        Ok(response)
    }
}
