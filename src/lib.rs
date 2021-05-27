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
pub mod jwt;
pub mod legacy;
pub mod list;
pub mod manga;
pub mod user;

pub use common::*;
pub use isolanguage_1;
pub use reqwest;

use errors::{ApiErrors, Result};
use reqwest::{Response, Url};
use serde::{de::DeserializeOwned, Deserialize};

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

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", tag = "result")]
enum ApiResult<T, E = ApiErrors> {
    Ok(T),
    Error(E),
}

impl<T, E> ApiResult<T, E> {
    pub fn into_result(self) -> Result<T, E> {
        match self {
            ApiResult::Ok(val) => Ok(val),
            ApiResult::Error(err) => Err(err),
        }
    }
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

    /// Deserialize ApiResult<T> then convert to Result<T>
    async fn json_api_result<T>(res: Response) -> Result<T>
    where
        T: DeserializeOwned,
    {
        Ok(res.json::<ApiResult<T, ApiErrors>>().await?.into_result()?)
    }

    /// Deserialize as Results<ApiResult<T>> then convert to Results<Result<T>>
    async fn json_api_results<T>(res: Response) -> Result<Results<Result<T>>>
    where
        T: DeserializeOwned,
    {
        let res = res.json::<Results<ApiResult<T, ApiErrors>>>().await?;
        Ok(Results {
            results: res
                .results
                .into_iter()
                .map(|r| r.into_result().map_err(|e| e.into()))
                .collect(),
            offset: res.offset,
            limit: res.limit,
            total: res.total,
        })
    }

    /// Deserialize as Vec<ApiResult<T>> then convert to Vec<Result<T>>
    async fn json_api_result_vec<T>(res: Response) -> Result<Vec<Result<T>>>
    where
        T: DeserializeOwned,
    {
        let res = res.json::<Vec<ApiResult<T, ApiErrors>>>().await?;
        Ok(res
            .into_iter()
            .map(|r| r.into_result().map_err(|e| e.into()))
            .collect())
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
