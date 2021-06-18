use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{ApiData, ApiObject, OrderType, Results};
use crate::Result;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChapterAttributes {
    pub title: String,
    pub volume: Option<String>,
    pub translated_language: String,
    pub hash: String,
    pub data: Vec<String>,
    pub data_saver: Vec<String>,
    pub uploader: Uuid,
    pub version: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub publish_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ChapterOrder {
    CreatedAt(OrderType),
    UpdatedAt(OrderType),
    PublishAt(OrderType),
    Volume(OrderType),
    Chapter(OrderType),
}

pub type Chapter = ApiObject<ChapterAttributes>;
pub type ChapterResponse = Result<ApiData<Chapter>>;
pub type ChapterList = Results<ChapterResponse>;
