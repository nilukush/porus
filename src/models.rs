// models.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PocketRequestTokenResponse {
    pub code: String,
    pub state: Option<String>,
}

impl std::fmt::Display for PocketRequestTokenResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Code: {}, State: {:?}", self.code, self.state)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PocketAccessTokenResponse {
    pub access_token: String,
    pub username: String,
}

impl std::fmt::Display for PocketAccessTokenResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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

#[derive(Debug, Deserialize, Serialize)]
pub struct PocketResponse {
    pub status: usize,
    pub complete: usize,
    pub list: std::collections::HashMap<String, PocketItem>,
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
    pub tags: std::collections::HashMap<String, Tag>,
    pub image: Option<Image>,
    pub images: std::collections::HashMap<String, Image>,
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
