use derive_builder::Builder;
use isolanguage_1::LanguageCode;
use serde::{Deserialize, Deserializer, Serialize};
use uuid::Uuid;

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

#[derive(Debug, Builder, Default, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
#[builder(setter(into, strip_option), default)]
pub struct PaginationQuery {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
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
