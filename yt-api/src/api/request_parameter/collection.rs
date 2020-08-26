use reqwest::Url;

use crate::api::request_parameter::RequestParameter;

pub struct RequestParametersSet {
    items: Vec<RequestParameter>
}

impl Default for RequestParametersSet {
    fn default() -> Self {
        RequestParametersSet { items: Default::default() }
    }
}

impl RequestParametersSet {
    pub fn write_to_url<'a>(&self, url: &'a mut Url) -> &'a mut Url {
        self.items.iter().for_each(|p| { url.query_pairs_mut().append_pair(&p.key, &p.value); });
        url
    }

    pub fn append<T: RequestParametersSetter>(self, parameter: &T) -> Self {
        parameter.set(self)
    }
    pub fn append_plain<K: AsRef<str>, V: AsRef<str>>(mut self, key: K, value: V) -> Self {
        self.items.push(RequestParameter::from_plain(key, value));
        self
    }

    pub fn append_iter<K: AsRef<str>, V: IntoIterator<Item=U>, U: AsRef<str>>(mut self, key: K, value: V) -> Self {
        self.items.push(RequestParameter::from_iterator(key, value));
        self
    }
}

pub trait RequestParametersSetter {
    fn set(&self, set: RequestParametersSet) -> RequestParametersSet;
}