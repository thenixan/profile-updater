use std::error::Error;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct Parameters {
    pub api_key: String,
    pub channel_id: String,
}

impl Parameters {
    pub fn try_init() -> Result<Self, ParametersError> {
        let api_key = std::env::var("YT_API_KEY")
            .map_err(|_| ParametersError::NoYouTubeApiKeyVariable)
            .and_then(|s|
                if s.is_empty() {
                    Err(ParametersError::NoYouTubeApiKeyVariable)
                } else {
                    Ok(s)
                })?;
        let channel_id = std::env::var("YT_CHANNEL_ID")
            .map_err(|_| ParametersError::NoYouTubeChannelIdVariable)
            .and_then(|s|
                if s.is_empty() {
                    Err(ParametersError::NoYouTubeChannelIdVariable)
                } else {
                    Ok(s)
                })?;
        Ok(Parameters { api_key, channel_id })
    }
}

#[derive(Debug)]
pub enum ParametersError {
    NoYouTubeApiKeyVariable,
    NoYouTubeChannelIdVariable,
}

impl std::fmt::Display for ParametersError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ParametersError::NoYouTubeApiKeyVariable => "no YT_API_KEY env variable found",
            ParametersError::NoYouTubeChannelIdVariable => "no YT_CHANNEL_ID env variable found"
        })
    }
}

impl Error for ParametersError {}
