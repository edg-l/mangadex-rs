use uuid::Uuid;
use serde::Deserialize;

pub type LocalizedString = std::collections::HashMap<String, String>;

#[derive(Debug, serde::Deserialize)]
pub struct Relationship {
    pub id: Uuid,
    pub r#type: String,
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

/// A response for endpoints which only give a simple result.
#[derive(Debug, serde::Deserialize)]
pub struct SimpleApiResponse {
    result: ApiResult,
}

#[derive(Debug, serde::Serialize)]
pub struct ListRequest {
    pub limit: i32,
    pub offset: i32,
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
