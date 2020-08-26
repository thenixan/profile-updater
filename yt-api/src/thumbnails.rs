use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Thumbnail {
    #[serde(flatten)]
    pub dimensions: Dimensions,
    #[serde(rename = "url")]
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct Dimensions {
    #[serde(rename = "width")]
    pub width: u16,
    #[serde(rename = "height")]
    pub height: u16,
}