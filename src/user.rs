use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct UserAttributes {
    pub username: String,
    pub version: i32,
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub r#type: String,
    pub attributes: UserAttributes,
}
