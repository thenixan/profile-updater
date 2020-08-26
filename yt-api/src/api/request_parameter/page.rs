use crate::api::request_parameter::collection::{RequestParametersSet, RequestParametersSetter};

#[derive(Clone)]
pub struct PageParameter {
    page: Option<String>
}

impl Default for PageParameter {
    fn default() -> Self {
        PageParameter { page: None }
    }
}

impl PageParameter {
    pub fn with_id<T: AsRef<str>>(t: T) -> Self {
        PageParameter { page: Some(t.as_ref().to_string()) }
    }
}

impl RequestParametersSetter for PageParameter {
    fn set<'a>(&self, set: RequestParametersSet) -> RequestParametersSet {
        if let Some(page) = &self.page {
            set.append_plain("pageToken", page)
        } else {
            set
        }
    }
}