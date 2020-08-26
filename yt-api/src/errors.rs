error_chain! {

        foreign_links {
            WrongApiKeyError(crate::api_key::FromStrEmptyError);
        }

        errors {
            NoYouTubeChannelIdVariable(e: String) {
                description("no YT_CHANNEL_ID variable found")
                display("no YT_CHANNEL_ID variable found: '{}'", e)
            }
            NoYouTubeApiKeyVariable(e: String) {
                description("no YT_API_KEY variable found")
                display("no YT_API_KEY variable found: '{}'", e)
            }
            CannotLoadChannelsList(e: String) {
                description("cannot load playlists")
                display("cannot load playlists: '{}'", e)
            }
            CannotLoadPlaylistItems(e: String) {
                description("cannot load playlist items")
                display("cannot load playlist items: '{}'", e)
            }
        }
        skip_msg_variant
    }