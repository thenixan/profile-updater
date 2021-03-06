use crate::api_key::ApiKey;
use crate::channel::Channel;
use crate::channel::channel_id::ChannelId;

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



