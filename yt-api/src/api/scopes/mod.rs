use std::error::Error;
use std::fmt::{Display, Result};

use serde::export::Formatter;

pub mod playlists;
pub mod playlist_items;

#[derive(Clone, Debug)]
pub struct ApiError {
    cause: String
}

impl ApiError {
    pub fn with<T: AsRef<str>>(cause: T) -> Self {
        ApiError { cause: cause.as_ref().to_string() }
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "couldn't get the response: {}", self.cause)
    }
}

impl Error for ApiError {}