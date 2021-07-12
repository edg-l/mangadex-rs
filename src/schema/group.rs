use chrono::{DateTime, Utc};
use serde::Deserialize;

use super::{user::User, ApiData, ApiObject, Results};
use crate::Result;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ScanlationGroupAttributes {
    pub name: String,
    pub leader: User,
    pub website: Option<String>,
    pub irc_server: Option<String>,
    pub irc_channel: Option<String>,
    pub discord: Option<String>,
    pub contact_email: Option<String>,
    pub description: Option<String>,
    pub locked: bool,
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
