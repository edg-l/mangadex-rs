use std::borrow::Cow;

use isolanguage_1::LanguageCode;
use reqwest::Method;
use serde::{de::DeserializeOwned, Deserialize, Deserializer, Serialize};
use uuid::Uuid;

use crate::{
    errors::{ApiErrors, Errors},
    ApiResult, Result,
};

pub type LocalizedString = std::collections::HashMap<LanguageCode, String>;

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct Relationship {
    pub id: Uuid,
    pub r#type: ResourceType,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ApiData<T> {
    pub data: T,
    #[serde(default)]
    pub relationships: Vec<Relationship>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ApiObject<A, T = ResourceType> {
    pub id: Uuid,
    pub r#type: T,
    pub attributes: A,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct NoData;

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct Results<T> {
    pub results: Vec<T>,
    pub limit: i32,
    pub offset: i32,
    pub total: i32,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct PaginationQuery {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

impl PaginationQuery {
    pub fn new(limit: Option<i32>, offset: Option<i32>) -> Self {
        Self { limit, offset }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum OrderType {
    Asc,
    Desc,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ResourceType {
    Manga,
    Chapter,
    CoverArt,
    Author,
    Artist,
    ScanlationGroup,
    Tag,
    User,
    CustomList,
}

pub(crate) trait UrlSerdeQS {
    fn query_qs<T: Serialize>(self, query: &T) -> Self;
}

impl UrlSerdeQS for url::Url {
    fn query_qs<T: Serialize>(mut self, query: &T) -> Self {
        self.set_query(Some(
            &serde_qs::to_string(query).expect("Failed to encode query string"),
        ));
        self
    }
}

pub(crate) fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + Deserialize<'de>,
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}

pub trait FromResponse: Sized {
    type Response;

    fn from_response(res: Self::Response) -> Self;
}

impl<T> FromResponse for Result<T, Errors> {
    type Response = ApiResult<T, ApiErrors>;

    fn from_response(value: Self::Response) -> Self {
        value.into_result().map_err(|e| e.into())
    }
}

impl<T> FromResponse for Results<Result<T, Errors>> {
    type Response = Results<ApiResult<T, ApiErrors>>;

    fn from_response(value: Self::Response) -> Self {
        Results {
            results: value
                .results
                .into_iter()
                .map(|r| r.into_result().map_err(|e| e.into()))
                .collect(),
            offset: value.offset,
            limit: value.limit,
            total: value.total,
        }
    }
}

impl<T> FromResponse for Vec<Result<T, Errors>> {
    type Response = Vec<ApiResult<T, ApiErrors>>;

    fn from_response(value: Self::Response) -> Self {
        value
            .into_iter()
            .map(|r| r.into_result().map_err(|e| e.into()))
            .collect()
    }
}

pub trait Endpoint {
    type Response: FromResponse;
    type Query: Serialize;
    type Body: Serialize;

    fn path(&self) -> Cow<str>;

    fn method(&self) -> Method {
        Method::GET
    }

    fn require_auth(&self) -> bool {
        false
    }

    fn query(&self) -> Option<&Self::Query> {
        None
    }

    fn body(&self) -> Option<&Self::Body> {
        None
    }
}

impl crate::Client {
    pub async fn endpoint<E>(&self, endpoint: &E) -> Result<E::Response>
    where
        E: Endpoint,
        <<E as Endpoint>::Response as FromResponse>::Response: DeserializeOwned,
    {
        let mut endpoint_url = self.base_url.join(&endpoint.path())?;
        if let Some(query) = endpoint.query() {
            endpoint_url = endpoint_url.query_qs(query);
        }

        let mut res = self.http.request(endpoint.method(), endpoint_url);
        if let Some(body) = endpoint.body() {
            res = res.json(body);
        }

        if endpoint.require_auth() {
            let tokens = self.require_tokens()?;
            res = res.bearer_auth(&tokens.session);
        }

        let res = res
            .send()
            .await?
            .json::<<E::Response as FromResponse>::Response>()
            .await?;

        Ok(FromResponse::from_response(res))
    }
}
