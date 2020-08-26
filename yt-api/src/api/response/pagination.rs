use std::fmt::Debug;

use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct PageInfo {
    #[serde(rename = "totalResults")]
    pub total_results: u32,
    #[serde(rename = "resultsPerPage")]
    pub results_per_page: u32,
}

#[derive(Deserialize, Clone, Debug)]
pub struct PagedResponse<T> where T: Clone + Debug {
    #[serde(rename = "etag")]
    pub etag: String,
    #[serde(rename = "nextPageToken")]
    pub next_page_token: Option<String>,
    #[serde(rename = "prevPageToken")]
    pub prev_page_token: Option<String>,
    #[serde(rename = "pageInfo")]
    pub page_info: PageInfo,
    #[serde(flatten)]
    pub content: T,
}