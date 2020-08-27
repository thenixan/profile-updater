use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::errors;
use crate::errors::ErrorKind::CannotLoadPlaylistItems;
use crate::thumbnails::Thumbnail;

#[derive(Deserialize, Debug, Clone)]
pub struct PlaylistItemsResponse {
    #[serde(rename = "items")]
    pub items: Vec<PlaylistItem>
}

#[derive(Deserialize, Debug, Clone)]
pub struct PlaylistItemContentDetails {
    #[serde(rename = "videoId")]
    pub video_id: String,
    #[serde(rename = "videoPublishedAt")]
    pub published_at: DateTime<Utc>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PlaylistItemSnippet {
    #[serde(rename = "position")]
    pub position: u32,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "thumbnails")]
    pub thumbnails: HashMap<String, Thumbnail>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PlaylistItem {
    #[serde(rename = "contentDetails")]
    pub content_details: PlaylistItemContentDetails,
    #[serde(rename = "snippet")]
    pub snippet: PlaylistItemSnippet,
}


pub fn load_playlist_items(key: &String, playlist_id: &String) -> Result<Vec<PlaylistItem>, errors::ErrorKind> {
    let url = format!("https://www.googleapis.com/youtube/v3/playlistItems?key={key}&playlistId={id}&maxResults={maxResults}&part=snippet,contentDetails",
                      key = key, id = playlist_id, maxResults = 50);
    println!("Url: {}", url);
    let response = reqwest::blocking::get(&url).map_err(|e| CannotLoadPlaylistItems(e.to_string()))?;
    if !response.status().is_success() {
        Err(CannotLoadPlaylistItems(format!("error code: {code}", code = response.status().as_u16())))
    } else {
        let response: PlaylistItemsResponse = response.json().map_err(|e| CannotLoadPlaylistItems(e.to_string()))?;
        Ok(response.items)
    }
}