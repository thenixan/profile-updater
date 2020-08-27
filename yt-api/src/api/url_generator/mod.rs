use std::ops::{Deref, DerefMut};

use reqwest::Url;

use crate::api::Api;
use crate::api::request_parameter::collection::RequestParametersSet;
use crate::api::url_generator::channels_url::ChannelsUrl;
use crate::api::url_generator::playlist_items_url::PlaylistItemsUrl;
use crate::api::url_generator::playlists_url::PlaylistsUrl;
use crate::base_url::{BASE_URL, BaseUrl};

mod channels_url;
mod playlists_url;
mod playlist_items_url;

pub struct UrlGenerator<'r> {
    api: &'r Api
}

impl<'r> UrlGenerator<'r> {
    pub fn from_api(api: &'r Api) -> Self {
        UrlGenerator { api }
    }

    pub fn channels_url(&self) -> UrlBuilder<ChannelsUrl> {
        UrlBuilder { api_url: ChannelsUrl::default(), key: &self.api.key }
    }

    pub fn playlists_url(&self) -> UrlBuilder<PlaylistsUrl> {
        UrlBuilder { api_url: PlaylistsUrl::default(), key: &self.api.key }
    }

    pub fn playlist_items_url(&self) -> UrlBuilder<PlaylistItemsUrl> {
        UrlBuilder { api_url: PlaylistItemsUrl::default(), key: &self.api.key }
    }
}

#[derive(Clone, Copy)]
pub struct UrlBuilder<'r, T> where T: ApiUrl {
    api_url: T,
    key: &'r str,
}

impl<'r, T> UrlBuilder<'r, T> where T: ApiUrl {
    pub fn apply<'a, F>(&mut self, mut f: F) -> &Self where F: FnMut(&mut T) {
        let api = &mut self.api_url;
        f(api);
        self
    }

    fn append_api_key<'a>(&self, url: &'a mut Url) -> &'a mut Url {
        url.query_pairs_mut().append_pair("key", self.key);
        url
    }
}

impl<'r, T> Into<Url> for &UrlBuilder<'r, T> where T: ApiUrl {
    fn into(self) -> Url {
        let mut url = BASE_URL.with_path(&self.name());
        self.append_api_key(&mut url);
        self.params().write_to_url(&mut url);
        url
    }
}

impl<'r, T> DerefMut for UrlBuilder<'r, T> where T: ApiUrl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.api_url
    }
}

impl<'r, T> Deref for UrlBuilder<'r, T> where T: ApiUrl {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.api_url
    }
}


pub trait ApiUrl: Clone {
    fn name(&self) -> String;

    fn params(&self) -> RequestParametersSet;
}