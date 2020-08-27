extern crate yt_api;

use std::fs::File;
use std::io::Write;
use std::str::FromStr;

use itertools::Itertools;

use yt_api::api::Api;
use yt_api::api::scopes::playlist_items::PlaylistItemsApi;
use yt_api::api::scopes::playlists::PlaylistApi;
use yt_api::api_key::ApiKey;
use yt_api::channel::channel_id::ChannelId;

use crate::parameters::Parameters;

mod parameters;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let parameters = Parameters::try_init()?;
    let api_key = ApiKey::from_str(&parameters.api_key)?;
    let api = Api::with_key(api_key);

    let channel_id = ChannelId::from(parameters.channel_id);
    let channel = api.channel(&channel_id);

    let playlists = PlaylistApi::list(&api, &channel.id)?;

    let mut output = File::create("README.md")?;

    writeln!(output, "[![](https://img.shields.io/badge/youtube-seems%2Fnerdy-red?style=plastic&logo=youtube)](https://www.youtube.com/channel/UCA7ymlAF32Up8VKeDVv9uQw)")?;
    writeln!(output)?;

    playlists.iter().for_each(|playlist| {
        writeln!(output, "- [{}](https://www.youtube.com/playlist?list={})", playlist.snippet.title, playlist.id).unwrap();
        if let Ok(playlist_items) = PlaylistItemsApi::list(&api, &playlist.id) {
            playlist_items.into_iter().sorted_by(|a, b| b.snippet.position.cmp(&a.snippet.position)).for_each(|item| {
                writeln!(output, "  - [{title} ![](https://img.shields.io/youtube/views/{id}?style=social)](https://youtu.be/{id})",
                         title = item.snippet.title,
                         id = item.content_details.video_id).unwrap();
                writeln!(output).unwrap();
            })
        }
    });


    Ok(())
}
