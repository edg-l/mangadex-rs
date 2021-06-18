use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{ApiData, ApiObject, OrderType, Results};
use crate::Result;

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AuthorOrder {
    pub name: OrderType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AuthorAttributes {
    pub name: String,
    pub image_url: Option<String>,
    // pub biography: HashMap<String, String>,
    pub version: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub type Author = ApiObject<AuthorAttributes>;
pub type AuthorResponse = Result<ApiData<Author>>;
pub type AuthorList = Results<AuthorResponse>;
