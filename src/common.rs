use isolanguage_1::LanguageCode;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type LocalizedString = std::collections::HashMap<LanguageCode, String>;

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Relationship {
    pub id: Uuid,
    pub r#type: ResourceType,
}

/// Common values returned in the "result" field from most responses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ApiResult {
    /// There was no error.
    Ok,
    /// There was an error.
    Error,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ApiObject<A, T = ResourceType> {
    pub id: Uuid,
    pub r#type: T,
    pub attributes: A,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ApiObjectResult<T> {
    pub result: ApiResult,
    pub data: T,
    pub relationships: Vec<Relationship>,
}

#[derive(Debug, Deserialize)]
pub struct Results<T> {
    pub results: Vec<T>,
    pub limit: i32,
    pub offset: i32,
    pub total: i32,
}

/// A response for endpoints which only give a simple result.
#[derive(Debug, Deserialize)]
pub struct SimpleApiResponse {
    result: ApiResult,
}

#[derive(Debug, Serialize, Default)]
pub struct PaginationQuery {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ResourceType {
    Manga,
    Chapter,
    CoverArt,
    Author,
    Artist,
    ScanlationGroup,
    Tag,
    User,
    CustomList,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    struct MyData {
        pub result: ApiResult,
    }

    #[test]
    fn deserializes_expect_error_struct() {
        let data = serde_json::json!({
            "result": "bad"
        });
        let data = serde_json::from_value::<MyData>(data);
        assert!(data.is_err())
    }

    #[test]
    fn deserializes_from_struct_ok() {
        let data = serde_json::json!({
            "result": "ok"
        });
        let data: MyData = serde_json::from_value(data).unwrap();
        assert_eq!(ApiResult::Ok, data.result);
    }

    #[test]
    fn deserializes_from_struct_error() {
        let data = serde_json::json!({
            "result": "error"
        });
        let data: MyData = serde_json::from_value(data).unwrap();
        assert_eq!(ApiResult::Error, data.result);
    }

    #[test]
    fn deserializes_expect_error() {
        let data = serde_json::json!("hello");
        let data = serde_json::from_value::<ApiResult>(data);
        assert!(data.is_err())
    }

    #[test]
    fn deserializes_from_ok() {
        let data = serde_json::json!("ok");
        assert_eq!(
            ApiResult::Ok,
            serde_json::from_value::<ApiResult>(data).unwrap()
        );
    }

    #[test]
    fn deserializes_from_error() {
        let data = serde_json::json!("error");
        assert_eq!(
            ApiResult::Error,
            serde_json::from_value::<ApiResult>(data).unwrap()
        );
    }
}
