use chrono::{DateTime, Utc};
use serde::Deserialize;

use super::{account::User, ApiData, ApiObject, Results};
use crate::Result;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ScanlationGroupAttributes {
    pub name: String,
    pub leader: User,
    pub version: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum ScanlationGroupType {
    ScanlationGroup,
}

pub type ScanlationGroup = ApiObject<ScanlationGroupAttributes, ScanlationGroupType>;
pub type ScanlationGroupData = ApiData<ScanlationGroup>;
pub type ScanlationGroupResponse = Result<ScanlationGroupData>;
pub type ScanlationGroupList = Results<ScanlationGroupResponse>;
