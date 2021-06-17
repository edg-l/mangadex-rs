#[macro_use]
pub mod common;
pub mod account;
pub mod athome;
pub mod auth;
pub mod author;
pub mod captcha;
pub mod chapter;
pub mod errors;
pub mod group;
pub mod infrastructure;
pub mod jwt;
pub mod legacy;
pub mod list;
pub mod manga;
pub mod user;

pub use common::*;
pub use isolanguage_1;
pub use reqwest;

use errors::Result;
use reqwest::Url;

static APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "-rs",
    "/",
    env!("CARGO_PKG_VERSION"),
);

/// The client used to talk to the api.
#[derive(Debug, Clone)]
pub struct Client {
    http: reqwest::Client,
    base_url: Url,
    tokens: Option<auth::AuthTokens>,
}

impl Default for Client {
    fn default() -> Self {
        Self::new("https://api.mangadex.org/").expect("Error creating default API client")
    }
}

impl Client {
    /// Create a new client.
    pub fn new(base_url: &str) -> Result<Self> {
        let client = reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            .build()?;

        Ok(Self {
            http: client,
            base_url: Url::parse(base_url)?,
            tokens: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ctor::ctor;

    #[ctor]
    fn init() {
        dotenv::dotenv().ok();
    }

    #[test]
    fn client_new() {
        Client::default();
    }
}
