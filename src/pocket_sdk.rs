// pocket_sdk.rs

use reqwest::header;
use reqwest::Client;
use std::collections::HashMap;
use std::convert::From;

use crate::error::CustomError;
use crate::models::*;

const POCKET_API_URL: &str = "https://getpocket.com/v3";
const POCKET_API_URL_WITHOUT_VERSION: &str = "https://getpocket.com";

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
