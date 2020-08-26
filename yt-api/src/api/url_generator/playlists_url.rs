use crate::api::request_parameter::collection::RequestParametersSet;
use crate::api::url_generator::ApiUrl;
use crate::api::request_parameter::parts_holder::PartsHolder;
use crate::channel::channel_id::ChannelId;

#[derive(Clone)]
pub struct PlaylistsUrl {
    parts: PartsHolder<Part>,
    id: String,
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Part {
    ContentDetails,
    Id,
    Localizations,
    Snippet,
    Player,
    Status,
}

impl AsRef<str> for Part {
    fn as_ref(&self) -> &str {
        match self {
            Part::Snippet => "snippet",
            Part::ContentDetails => "contentDetails",
            Part::Id => "id",
            Part::Localizations => "localizations",
            Part::Status => "status",
            Part::Player => "player"
        }
    }
}

impl PlaylistsUrl {
    pub fn part_content_details(&mut self) -> &mut Self {
        self.parts.toggle_part(Part::ContentDetails);
        self
    }

    pub fn part_snippet(&mut self) -> &mut Self {
        self.parts.toggle_part(Part::Snippet);
        self
    }

    pub fn part_status(&mut self) -> &mut Self {
        self.parts.toggle_part(Part::Status);
        self
    }

    pub fn part_player(&mut self) -> &mut Self {
        self.parts.toggle_part(Part::Player);
        self
    }

    pub fn part_id(&mut self) -> &mut Self {
        self.parts.toggle_part(Part::Id);
        self
    }

    pub fn part_localizations(&mut self) -> &mut Self {
        self.parts.toggle_part(Part::Localizations);
        self
    }

    pub fn channel_id<T: Into<ChannelId>>(&mut self, channel_id: T) -> &mut Self {
        self.id = channel_id.into().0.clone();
        self
    }
}

impl Default for PlaylistsUrl {
    fn default() -> Self {
        PlaylistsUrl {
            parts: PartsHolder::default(),
            id: "".to_string(),
        }
    }
}

impl ApiUrl for PlaylistsUrl {
    fn name(&self) -> String {
        "playlists".to_string()
    }

    fn params(&self) -> RequestParametersSet {
        let parts = &self.parts;
        RequestParametersSet::default()
            .append_plain("channelId", &self.id)
            .append(parts)
    }
}