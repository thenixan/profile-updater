use chrono::{DateTime, Utc};
use reqwest::Url;

use crate::api::Api;
use crate::api::request_parameter::page::PageParameter;
use crate::api::response::pagination::PagedResponse;
use crate::api::response::playlist::{PlaylistDescription, PlaylistResponse};
use crate::api::scopes::ApiError;
use crate::api::url_generator::UrlGenerator;
use crate::channel::channel_id::ChannelId;
use crate::errors::ErrorKind::CannotLoadChannelsList;
use crate::playlist_items::{PlaylistItem, PlaylistItemsResponse};

pub trait PlaylistItemsApi {
    fn list(&self, playlist_id: &str) -> Result<Vec<PlaylistItem>, ApiError>;
}

impl PlaylistItemsApi for Api {
    fn list(&self, channel_id: &str) -> Result<Vec<PlaylistItem>, ApiError> {
        let mut responses = vec![];

        let latest_response = load_page(self, channel_id, &PageParameter::default())?;
        responses.push(latest_response.content.items);
        let mut next_page_token = latest_response.next_page_token;
        while let Some(ref page_token) = next_page_token {
            let latest_response = load_page(self, channel_id, &PageParameter::with_id(page_token))?;
            responses.push(latest_response.content.items);
            next_page_token = latest_response.next_page_token;
        };
        Ok(responses.into_iter().flatten().collect())
    }
}

fn load_page(api: &Api, playlist_id: &str, page: &PageParameter) -> Result<PagedResponse<PlaylistItemsResponse>, ApiError> {
    let url_generator = UrlGenerator::from_api(api);

    //TODO I don't like the following - suggest inventing some pretty-faced conversions
    let url: Url = url_generator.playlist_items_url().apply(|api| {
        api.part_content_details().part_snippet().playlist_id(playlist_id.to_string()).page(page.clone());
    }).into();
    let response = reqwest::blocking::get(url).map_err(|e| ApiError::with(e.to_string()))?;
    if !response.status().is_success() {
        Err(ApiError::with(format!("error code: {code}", code = response.status().as_u16())))
    } else {
        let response: PagedResponse<PlaylistItemsResponse> = response.json().map_err(|e| ApiError::with(e.to_string()))?;
        Ok(response)
    }
}

