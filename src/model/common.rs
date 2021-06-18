use isolanguage_1::LanguageCode;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    errors::{ApiErrors, Errors},
    FromResponse,
};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum OrderType {
    Asc,
    Desc,
}

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
pub(crate) struct ApiResult<T, E = ApiErrors>(
    #[serde(with = "ApiResultDef")] std::result::Result<T, E>,
);

impl<T, E> ApiResult<T, E> {
    pub fn into_result(self) -> Result<T, E> {
        self.0
    }
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
