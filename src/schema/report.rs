use serde::{Deserialize, Serialize};

use super::{ApiData, ApiObject, LocalizedString, Results};
use crate::Result;

/// The report category.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ReportCategory {
    Manga,
    Chapter,
    ScanlationGroup,
    User,
}

impl std::fmt::Display for ReportCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Manga => write!(f, "manga"),
            Self::Chapter => write!(f, "chapter"),
            Self::ScanlationGroup => write!(f, "scanlation_group"),
            Self::User => write!(f, "user"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReportAttributes {
    pub reason: LocalizedString,
    pub details_required: bool,
    pub category: ReportCategory,
    pub version: i32,
}

pub type Report = ApiObject<ReportAttributes>;
pub type ReportResponse = Result<ApiData<Report>>;
pub type ReportList = Results<ReportResponse>;
