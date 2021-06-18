use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::Result;

use super::{ApiData, ApiObject};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MappingType {
    Group,
    Manga,
    Chapter,
    Tag,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MappingIdType {
    MappingId,
}

#[derive(Debug, Deserialize, Clone, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MappingIdAttributes {
    pub r#type: MappingType,
    pub legacy_id: u32,
    pub new_id: Uuid,
}

pub type MappingId = ApiObject<MappingIdAttributes, MappingIdType>;
pub type MappingIdResponse = Result<ApiData<MappingId>>;
