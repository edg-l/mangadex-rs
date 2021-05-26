use isolanguage_1::LanguageCode;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type LocalizedString = std::collections::HashMap<LanguageCode, String>;

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Relationship {
    pub id: Uuid,
    pub r#type: ResourceType,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ApiData<T> {
    pub data: T,
    #[serde(default)]
    pub relationships: Vec<Relationship>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ApiObject<A, T = ResourceType> {
    pub id: Uuid,
    pub r#type: T,
    pub attributes: A,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NoData;

#[derive(Debug, Deserialize)]
pub struct Results<T> {
    pub results: Vec<T>,
    pub limit: i32,
    pub offset: i32,
    pub total: i32,
}

#[derive(Debug, Serialize, Default)]
pub struct PaginationQuery {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
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
