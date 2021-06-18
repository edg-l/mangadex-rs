use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{model::OrderType, Result};

use super::{ApiData, ApiObject, Results};

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum CoverOrder {
    CreatedAt(OrderType),
    UpdatedAt(OrderType),
    Volume(OrderType),
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CoverAttributes {
    pub volume: Option<String>,
    pub file_name: String,
    pub description: Option<String>,
    pub version: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub type Cover = ApiObject<CoverAttributes>;
pub type CoverResponse = Result<ApiData<Cover>>;
pub type CoverList = Results<CoverResponse>;
