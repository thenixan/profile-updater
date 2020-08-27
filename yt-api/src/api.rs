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
}



