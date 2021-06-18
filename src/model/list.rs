use serde::{Deserialize, Serialize};

use crate::Result;

use super::{user::User, ApiData, ApiObject, Results};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum CustomListVisibility {
    Public,
    Private,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CustomListAttributes {
    name: String,
    visibility: CustomListVisibility,
    owner: User,
    version: i32,
}

pub type CustomList = ApiObject<CustomListAttributes>;
pub type CustomListResponse = Result<ApiData<CustomList>>;
pub type CustomListList = Results<CustomListResponse>;
