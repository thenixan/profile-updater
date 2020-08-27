use crate::api::request_parameter::collection::{RequestParametersSet, RequestParametersSetter};
use crate::api::request_parameter::page::PageParameter;
use crate::api::request_parameter::parts_holder::PartsHolder;
use crate::api::url_generator::ApiUrl;

#[derive(Clone)]
struct MaxItems(Option<u32>);

impl RequestParametersSetter for MaxItems {
    fn set(&self, set: RequestParametersSet) -> RequestParametersSet {
        if let Some(max) = self.0 {
            set.append_plain("maxItems", max.to_string())
        } else {
            set
        }
    }
}

#[derive(Clone)]
pub struct PlaylistItemsUrl {
    parts: PartsHolder<Part>,
    id: String,
    page: PageParameter,
    limiter: MaxItems,
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Part {
    ContentDetails,
    Id,
    Snippet,
    Status,
}

impl AsRef<str> for Part {
    fn as_ref(&self) -> &str {
        match self {
            Part::Snippet => "snippet",
            Part::ContentDetails => "contentDetails",
            Part::Id => "id",
            Part::Status => "status",
        }
    }
}

impl PlaylistItemsUrl {
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

    pub fn part_id(&mut self) -> &mut Self {
        self.parts.toggle_part(Part::Id);
        self
    }

    pub fn playlist_id(&mut self, playlist_id: String) -> &mut Self {
        self.id = playlist_id;
        self
    }

    pub fn page(&mut self, page: PageParameter) -> &mut Self {
        self.page = page;
        self
    }

    pub fn limit(&mut self, limit: Option<u32>) -> &mut Self {
        self.limiter = MaxItems(limit);
        self
    }
}

impl Default for PlaylistItemsUrl {
    fn default() -> Self {
        PlaylistItemsUrl {
            parts: PartsHolder::default(),
            id: "".to_string(),
            page: PageParameter::default(),
            limiter: MaxItems(None),
        }
    }
}

impl ApiUrl for PlaylistItemsUrl {
    fn name(&self) -> String {
        "playlistItems".to_string()
    }

    fn params(&self) -> RequestParametersSet {
        let parts = &self.parts;
        RequestParametersSet::default()
            .append_plain("playlistId", &self.id)
            .append(&self.page)
            .append(parts)
            .append(&self.limiter)
    }
}