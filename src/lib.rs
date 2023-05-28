use reqwest::header;
use reqwest::Client;
use reqwest::Error as ReqwestError;
use serde::{Deserialize, Serialize, Serializer};
use serde_json::Error as JsonError;
use std::convert::From;
use std::fmt;

const POCKET_API_URL: &str = "https://getpocket.com/v3";
const POCKET_API_URL_WITHOUT_VERSION: &str = "https://getpocket.com";

#[derive(Debug, Deserialize, Serialize)]
pub struct PocketRequestTokenResponse {
    pub code: String,
    pub state: Option<String>,
}

impl fmt::Display for PocketRequestTokenResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Code: {}, State: {:?}", self.code, self.state)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PocketAccessTokenResponse {
    pub access_token: String,
    pub username: String,
}

impl fmt::Display for PocketAccessTokenResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Access Token: {}, Username: {:?}",
            self.access_token, self.username
        )
    }
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

impl Serialize for CustomError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            CustomError::ReqwestError(_) => {
                // Customize the serialization of the ReqwestError variant
                serializer.serialize_unit()
            }
            CustomError::JsonError(_) => {
                // Customize the serialization of the JsonError variant
                serializer.serialize_unit()
            } // Handle serialization of other error variants...
        }
    }
}

#[derive(Clone)] // Add the Clone trait
pub struct PocketSdk {
    consumer_key: String,
    client: Client,
}

impl PocketSdk {
    pub fn new(consumer_key: String) -> Self {
        let client = Client::new();
        PocketSdk {
            consumer_key,
            client,
        }
    }

    pub async fn obtain_request_token(
        &self,
        redirect_uri: &str,
    ) -> Result<PocketRequestTokenResponse, CustomError> {
        let url = format!("{}/oauth/request", POCKET_API_URL);

        let params = [
            ("consumer_key", self.consumer_key.as_str()),
            ("redirect_uri", redirect_uri),
        ];

        let response = self
            .client
            .post(&url)
            .header(header::CONTENT_TYPE, "application/json; charset=UTF-8")
            .header("X-Accept", "application/json")
            .form(&params)
            .send()
            .await?;

        let body = response.text().await?;
        println!("Response body: {}", body);

        let parsed_response = serde_json::from_str::<PocketRequestTokenResponse>(&body)
            .map_err(|err| CustomError::from(err))?;

        Ok(parsed_response)
    }

    pub fn build_authorization_url(&self, request_token: &str, redirect_uri: &str) -> String {
        format!(
            "{}/oauth/authorize?request_token={}&redirect_uri={}",
            POCKET_API_URL_WITHOUT_VERSION, request_token, redirect_uri
        )
    }

    pub async fn convert_request_token_to_access_token(
        &self,
        request_token: &str,
    ) -> Result<PocketAccessTokenResponse, CustomError> {
        let url = format!("{}/oauth/authorize", POCKET_API_URL);

        let params = [
            ("consumer_key", self.consumer_key.as_str()),
            ("code", request_token),
        ];

        let response = self
            .client
            .post(&url)
            .header(header::CONTENT_TYPE, "application/json; charset=UTF-8")
            .header("X-Accept", "application/json")
            .form(&params)
            .send()
            .await?;

        let body = response.text().await?;
        println!("Response body: {}", body);

        let parsed_response = serde_json::from_str::<PocketAccessTokenResponse>(&body)
            .map_err(|err| CustomError::from(err))?;

        Ok(parsed_response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_obtain_request_token() {
        let pocket_sdk = PocketSdk::new("80908-b39061ed0999bb292f0fe716".to_string());
        let redirect_uri = "http://example.com"; // Specify the redirect URI here
        let result = pocket_sdk.obtain_request_token(redirect_uri).await;

        assert!(result.is_ok());
        let request_token_response = result.unwrap();
        println!("Request token response: {:?}", request_token_response);
    }

    #[tokio::test]
    async fn test_convert_request_token_to_access_token() {
        let pocket_sdk = PocketSdk::new("80908-b39061ed0999bb292f0fe716".to_string());

        // Step 1: Obtain the request token
        let result = pocket_sdk.obtain_request_token("http:://example.com").await;
        assert!(result.is_ok());
        let request_token_response = result.unwrap();
        println!("Request token response: {:?}", request_token_response);

        // Step 2: Build the authorization URL
        let request_token = request_token_response.code;
        let authorization_url =
            pocket_sdk.build_authorization_url(&request_token, "http:://example.com");
        println!("Authorization URL: {}", authorization_url);

        // Step 3: Instruct the user to open the authorization URL and authorize the application

        // Step 4: Convert the request token to an access token
        println!("Please authorize the application and press Enter to continue...");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let result = pocket_sdk
            .convert_request_token_to_access_token(&request_token)
            .await;
        assert!(result.is_ok());
        let access_token_response = result.unwrap();
        println!("Access token response: {:?}", access_token_response);
    }
}
