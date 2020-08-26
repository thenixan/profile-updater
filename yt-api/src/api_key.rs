use std::fmt::Display;
use std::ops::Deref;
use std::str::FromStr;

use serde::export::Formatter;

#[derive(Clone, Debug)]
pub struct ApiKey(String);

impl Deref for ApiKey {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


impl FromStr for ApiKey {
    type Err = FromStrEmptyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err(FromStrEmptyError {})
        } else {
            Ok(ApiKey(s.into()))
        }
    }
}

#[derive(Debug)]
pub struct FromStrEmptyError {}

impl Display for FromStrEmptyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "api key is empty")
    }
}

impl std::error::Error for FromStrEmptyError {}