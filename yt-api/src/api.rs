use reqwest::Url;

use crate::api::response::pagination::PagedResponse;
use crate::api::response::playlist::PlaylistResponse;
use crate::api::url_generator::UrlGenerator;
use crate::api_key::ApiKey;
use crate::channel::Channel;
use crate::channel::channel_id::ChannelId;
use crate::errors;
use crate::errors::ErrorKind::CannotLoadChannelsList;

pub mod url_generator;
pub mod request_parameter;
pub mod scopes;
pub mod response;

#[derive(Debug)]
pub struct Api {
    key: ApiKey,
}

impl Api {
    pub fn with_key(key: ApiKey) -> Self {
        Api { key }
    }

    pub fn channel(&self, channel_id: &ChannelId) -> Channel {
        Channel::with_id(channel_id.clone())
    }

    pub fn playlists(&self, channel_id: &ChannelId) -> Result<PagedResponse<PlaylistResponse>, errors::ErrorKind> {
        let url_generator = UrlGenerator::from_api(self);

        //TODO I don't like the following - suggest inventing some pretty-faced conversions
        let url: Url = url_generator.playlists_url().apply(|api| {
            api.part_content_details().part_snippet().channel_id(channel_id.clone());
        }).into();
        let response = reqwest::blocking::get(url).map_err(|e| CannotLoadChannelsList(e.to_string()))?;
        if !response.status().is_success() {
            Err(CannotLoadChannelsList(format!("error code: {code}", code = response.status().as_u16())))
        } else {
            let response: PagedResponse<PlaylistResponse> = response.json().map_err(|e| CannotLoadChannelsList(e.to_string()))?;
            Ok(response)
        }
    }
}



