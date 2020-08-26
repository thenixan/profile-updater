use crate::api::request_parameter::collection::RequestParametersSet;
use crate::api::url_generator::ApiUrl;
use crate::api::request_parameter::parts_holder::PartsHolder;

#[derive(Clone)]
pub struct ChannelsUrl {
    parts: PartsHolder<Part>,
    ids: Vec<String>,
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Part {
    AuditDetails,
    BrandingSettings,
    ContentDetails,
    ContentOwnerDetails,
    Id,
    Localizations,
    Snippet,
    Statistics,
    Status,
    TopicDetails,
}

impl AsRef<str> for Part {
    fn as_ref(&self) -> &str {
        match self {
            Part::Snippet => "snippet",
            Part::AuditDetails => "auditDetails",
            Part::BrandingSettings => "brandingSettings",
            Part::ContentDetails => "contentDetails",
            Part::ContentOwnerDetails => "contentOwnerDetails",
            Part::Id => "id",
            Part::Localizations => "localizations",
            Part::Statistics => "statistics",
            Part::Status => "status",
            Part::TopicDetails => "topicDetails",
        }
    }
}


impl Default for ChannelsUrl {
    fn default() -> Self {
        ChannelsUrl {
            parts: PartsHolder::default(),
            ids: vec![],
        }
    }
}

impl ChannelsUrl {
    pub fn part_audit_details(&mut self) -> &mut Self {
        self.parts.toggle_part(Part::AuditDetails);
        self
    }

    pub fn part_branding_settings(&mut self) -> &mut Self {
        self.parts.toggle_part(Part::BrandingSettings);
        self
    }

    pub fn part_content_owner_details(&mut self) -> &mut Self {
        self.parts.toggle_part(Part::ContentOwnerDetails);
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

    pub fn part_status(&mut self) -> &mut Self {
        self.parts.toggle_part(Part::Status);
        self
    }

    pub fn part_statistics(&mut self) -> &mut Self {
        self.parts.toggle_part(Part::Statistics);
        self
    }

    pub fn part_topic_details(&mut self) -> &mut Self {
        self.parts.toggle_part(Part::TopicDetails);
        self
    }

    pub fn part_content_details(&mut self) -> &mut Self {
        self.parts.toggle_part(Part::ContentDetails);
        self
    }

    pub fn part_snippet(&mut self) -> &mut Self {
        self.parts.toggle_part(Part::Snippet);
        self
    }

    pub fn id<T: Into<String>>(&mut self, id: T) -> &mut Self {
        self.ids.push(id.into());
        self
    }
}


impl ApiUrl for ChannelsUrl {
    fn name(&self) -> String {
        "channels".to_string()
    }

    fn params(&self) -> RequestParametersSet {
        let parts = &self.parts;
        RequestParametersSet::default()
            .append_iter("id", &self.ids)
            .append(parts)
    }
}