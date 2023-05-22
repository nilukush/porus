use reqwest::header;
use reqwest::Client;
use reqwest::Error as ReqwestError;
use serde::Deserialize;
use serde_json::json;
use serde_json::Error as JsonError;
use std::convert::From;
use std::fmt;

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

#[derive(Debug)]
pub enum CustomError {
    ReqwestError(ReqwestError),
    JsonError(JsonError),
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomError::ReqwestError(err) => write!(f, "ReqwestError: {}", err),
            CustomError::JsonError(err) => write!(f, "JsonError: {}", err),
        }
    }
}

impl std::error::Error for CustomError {}

impl From<ReqwestError> for CustomError {
    fn from(err: ReqwestError) -> Self {
        CustomError::ReqwestError(err)
    }
}

impl From<JsonError> for CustomError {
    fn from(err: JsonError) -> Self {
        CustomError::JsonError(err)
    }
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

    pub async fn obtain_request_token(&self) -> Result<PocketRequestTokenResponse, CustomError> {
        let url = format!("{}/oauth/request", POCKET_API_URL);

        let params = [
            ("consumer_key", self.consumer_key.as_str()),
            ("redirect_uri", self.redirect_uri.as_str()),
        ];

        let response = self
            .client
            .post(&url)
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .form(&params)
            .send()
            .await?;

        let body = response.text().await?;
        println!("Response body: {}", body);

        let body_json = convert_body_text_to_json(body);
        println!("{}", body_json);

        let parsed_response = serde_json::from_str::<PocketRequestTokenResponse>(&body_json)
            .map_err(|err| CustomError::from(err))?;

        Ok(parsed_response)
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
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .form(&params)
            .send()
            .await?
            .json::<PocketAccessTokenResponse>()
            .await?;

        Ok(response)
    }
}

fn convert_body_text_to_json(body: String) -> String {
    let mut parts = body.splitn(2, '=');

    let key = parts.next().unwrap_or("");
    let value = parts.next().unwrap_or("");

    let json_data = json!({
        key: value,
    });
    json_data.to_string()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_obtain_request_token() {
        let pocket_sdk = PocketSdk::new(
            "80908-b39061ed0999bb292f0fe716".to_string(),
            "pocketapp1234:authorizationFinished".to_string(),
        );
        let result = pocket_sdk.obtain_request_token().await;

        assert!(result.is_ok());
        let request_token_response = result.unwrap();
        println!("Request token response: {:?}", request_token_response);
    }

    #[tokio::test]
    async fn test_convert_request_token_to_access_token() {
        let pocket_sdk = PocketSdk::new(
            "80908-b39061ed0999bb292f0fe716".to_string(),
            "pocketapp1234:authorizationFinished".to_string(),
        );
        let request_token = "YOUR_REQUEST_TOKEN";
        let result = pocket_sdk
            .convert_request_token_to_access_token(request_token)
            .await;

        assert!(result.is_ok());
        let access_token_response = result.unwrap();
        println!("Access token response: {:?}", access_token_response);
    }
}
