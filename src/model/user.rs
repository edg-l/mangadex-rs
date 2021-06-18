use serde::{Deserialize, Serialize};

use super::{ApiData, ApiObject, OrderType, Results};
use crate::Result;

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

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum UserOrder {
    Username(OrderType),
}

pub type User = ApiObject<UserAttributes, UserType>;
pub type UserResponse = Result<ApiData<User>>;
pub type UserList = Results<UserResponse>;
