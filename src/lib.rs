use reqwest::header;
use reqwest::Client;
use reqwest::Error as ReqwestError;
use serde::{Deserialize, Serialize, Serializer};
use serde_json::Error as JsonError;
use std::collections::HashMap;
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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PocketTag {
    pub tag: String,
    pub item_count: usize,
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

#[derive(Debug, Deserialize, Serialize)]
pub struct PocketResponse {
    pub status: usize,
    pub complete: usize,
    pub list: HashMap<String, PocketItem>,
    pub error: Option<String>,
    pub search_meta: SearchMeta,
    pub since: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PocketItem {
    pub item_id: String,
    pub resolved_id: String,
    pub given_url: String,
    pub given_title: String,
    pub favorite: String,
    pub status: String,
    pub time_added: String,
    pub time_updated: String,
    pub time_read: String,
    pub time_favorited: String,
    pub sort_id: usize,
    pub resolved_title: String,
    pub resolved_url: String,
    pub excerpt: String,
    pub is_article: String,
    pub is_index: String,
    pub has_video: String,
    pub has_image: String,
    pub word_count: String,
    pub lang: String,
    pub top_image_url: String,
    pub tags: HashMap<String, Tag>,
    pub image: Option<Image>,
    pub images: HashMap<String, Image>,
    pub listen_duration_estimate: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Tag {
    pub item_id: String,
    pub tag: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Image {
    pub item_id: String,
    pub src: String,
    pub width: String,
    pub height: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchMeta {
    pub search_type: String,
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

    pub async fn get_tags_with_article_count(
        &self,
        access_token: &str,
    ) -> Result<Vec<PocketTag>, CustomError> {
        let url = format!("{}/get", POCKET_API_URL);

        let params = [
            ("consumer_key", self.consumer_key.as_str()),
            ("access_token", access_token),
            ("state", "all"),
            ("detailType", "complete"),
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

        let parsed_response: PocketResponse =
            serde_json::from_str(&body).map_err(|err| CustomError::from(err))?;

        let mut tags_with_article_count: HashMap<String, usize> = HashMap::new();
        for item in parsed_response.list.values() {
            for tag in item.tags.keys() {
                let tag_entry = tags_with_article_count.entry(tag.clone()).or_insert(0);
                *tag_entry += 1;
            }
        }

        let tags_with_article_count = tags_with_article_count
            .into_iter()
            .map(|(tag, item_count)| PocketTag { tag, item_count })
            .collect();

        Ok(tags_with_article_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::Method;
    use httpmock::MockServer;

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

    #[tokio::test]
    async fn test_get_tags_with_article_count() {
        // Initialize the PocketSdk with a dummy consumer key
        let pocket_sdk = PocketSdk::new("80908-b39061ed0999bb292f0fe716".to_string());

        // Mock the HTTP response body
        let response_body = r#"{"status":1,"complete":1,"list":{"30211549":{"item_id":"30211549","resolved_id":"30211549","given_url":"https:\/\/www.homeremediesweb.com","given_title":"","favorite":"0","status":"0","time_added":"1537696723","time_updated":"1537696723","time_read":"0","time_favorited":"0","sort_id":0,"resolved_title":"Good Health Starts At Home","resolved_url":"https:\/\/www.homeremediesweb.com\/","excerpt":"Welcome to Home Remedies Web, one of the premier online resources for finding free home remedies from around the world. Natural remedies, home remedies, and herbal supplements are growing in popularity as more and more people discover the magical benefits of these forms of alternative medicine.","is_article":"0","is_index":"1","has_video":"0","has_image":"1","word_count":"96","lang":"en","top_image_url":"https:\/\/www.homeremediesweb.com\/assets\/img\/logo_social.jpg","listen_duration_estimate":37}},"error":null,"search_meta":{"search_type":"normal"},"since":1685373830}"#;

        // Create a mock server
        let server = MockServer::start();

        // Create a mock response
        let _mock = server.mock(|when, then| {
            when.method(Method::POST)
                .path("/get")
                .header("Content-Type", "application/json; charset=UTF-8");
            then.status(200)
                .header("Content-Type", "application/json; charset=UTF-8")
                .body(response_body);
        });

        // Perform the test
        let access_token = "f52be2ce-e2e1-366d-0acb-b19670";
        let result = pocket_sdk.get_tags_with_article_count(access_token).await;

        if let Err(err) = &result {
            eprintln!("Error: {}", err);
        }

        assert!(result.is_ok());
        let tags_with_article_count = result.unwrap();
        println!("Tags with article count: {:?}", tags_with_article_count);
    }
}
