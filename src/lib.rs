pub mod account;
pub mod athome;
pub mod auth;
pub mod author;
pub mod captcha;
pub mod chapter;
pub mod common;
pub mod errors;
pub mod feed;
pub mod group;
pub mod infrastructure;
pub mod legacy;
pub mod list;
pub mod manga;
pub mod user;

pub use reqwest;

use reqwest::Url;

static APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "-rs",
    "/",
    env!("CARGO_PKG_VERSION"),
);

/// The client used to talk to the api.
pub struct Client {
    http: reqwest::Client,
    base_url: Url,
    tokens: Option<auth::AuthTokens>,
}

impl Client {
    /// Create a new client.
    pub fn new() -> reqwest::Result<Self> {
        let client = reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            .build()?;

        let base_url = Url::parse("https://api.mangadex.org/").expect("error parsing the base url");

        Ok(Self {
            http: client,
            base_url,
            tokens: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ctor::ctor;
    use pretty_assertions::assert_eq;

    #[ctor]
    fn init() {
        dotenv::dotenv().ok();
    }

    pub fn has_tokens() -> bool {
        std::env::var("TEST_MANGADEX_TOKEN_REFRESH").is_ok()
    }

    pub fn get_tokens() -> auth::AuthTokens {
        auth::AuthTokens {
            refresh: std::env::var("TEST_MANGADEX_TOKEN_REFRESH").unwrap(),
            session: std::env::var("TEST_MANGADEX_TOKEN_SESSION").unwrap(),
        }
    }

    pub fn get_auth_details() -> (String, String) {
        let username = std::env::var("TEST_MANGADEX_USERNAME").unwrap();
        let password = std::env::var("TEST_MANGADEX_PASSWORD").unwrap();
        (username, password)
    }

    #[test]
    fn client_new() {
        let client = Client::new();
        assert_eq!(client.is_ok(), true);
    }
}
