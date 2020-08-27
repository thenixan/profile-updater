use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct PlaylistResponse {
    #[serde(rename = "items")]
    pub items: Vec<PlaylistDescription>,
    #[serde(rename = "kind")]
    pub kind: String,
}


#[derive(Clone, Deserialize, Debug)]
pub struct PlaylistContentDetails {
    #[serde(rename = "itemCount")]
    pub count: u32
}

#[derive(Clone, Deserialize, Debug)]
pub struct PlaylistDescription {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "snippet")]
    pub snippet: PlaylistSnippet,
    #[serde(rename = "contentDetails")]
    pub content_details: PlaylistContentDetails,
}

#[derive(Clone, Deserialize, Debug)]
pub struct PlaylistSnippet {
    #[serde(rename = "title")]
    pub title: String
}