use serde::{Deserialize, Serialize};

use crate::Result;

use super::{ApiData, ApiObject};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserAttributes {
    pub username: String,
    pub version: i32,
}

#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum UserType {
    User,
}

pub type User = ApiObject<UserAttributes, UserType>;
pub type UserResponse = Result<ApiData<User>>;
