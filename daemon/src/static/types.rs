use {
    serde::{Deserialize, Serialize},
    std::fmt,
    warp::{http::StatusCode, reject::Reject},
};

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
pub struct ValidationResult {
    pub body: &'static str,
    pub status: StatusCode,
}

impl Reject for ValidationResult {}

impl fmt::Display for ValidationResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.body, self.status)
    }
}
