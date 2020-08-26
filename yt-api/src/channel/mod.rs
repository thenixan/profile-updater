use crate::channel::channel_id::ChannelId;

pub mod channel_id;

#[derive(Debug)]
pub struct Channel {
    pub id: ChannelId
}

impl Channel {
    pub fn with_id(id: ChannelId) -> Self {
        Channel { id }
    }
}