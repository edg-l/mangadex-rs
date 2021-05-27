use crate::ApiObject;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UserAttributes {
    pub username: String,
    pub version: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserType {
    User,
}

pub type User = ApiObject<UserAttributes, UserType>;
