extern crate yt_api;

use std::str::FromStr;

use yt_api::api::Api;
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

    let playlists = api.playlists(&channel.id);

    println!("{:?}", playlists);

    Ok(())
}
