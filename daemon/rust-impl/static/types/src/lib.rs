use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize)]
pub struct Payload {
    pub timestamp: u128,
    pub endpoint: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub endpoint: String,
}

#[derive(Debug)]
pub struct ValidationResult<'a> {
    pub body: &'a str,
    pub status: StatusCode,
}

impl<'a> fmt::Display for ValidationResult<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.body, self.status)
    }
}
