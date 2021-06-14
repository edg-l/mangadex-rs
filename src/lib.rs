pub mod account;
pub mod athome;
pub mod auth;
pub mod author;
pub mod captcha;
pub mod chapter;
pub mod common;
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
#[derive(Debug, Clone)]
pub struct Client {
    http: reqwest::Client,
    base_url: Url,
    tokens: Option<auth::AuthTokens>,
}

#[derive(Deserialize)]
#[serde(tag = "result", remote = "std::result::Result")]
enum ApiResultDef<T, E> {
    #[serde(rename = "ok")]
    Ok(T),
    #[serde(rename = "error")]
    Err(E),
}

#[derive(Deserialize)]
#[serde(bound = "T: DeserializeOwned, E: DeserializeOwned")]
pub struct ApiResult<T, E = ApiErrors>(#[serde(with = "ApiResultDef")] std::result::Result<T, E>);

impl<T, E> ApiResult<T, E> {
    fn into_result(self) -> Result<T, E> {
        self.0
    }
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

    #[ctor]
    fn init() {
        dotenv::dotenv().ok();
    }

    #[test]
    fn client_new() {
        Client::default();
    }
}
