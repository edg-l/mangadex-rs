use serde::de::{Deserialize, Error, Visitor};



/// Common values returned in the "result" field from most responses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApiResult {
    /// There was no error.
    Ok,
    /// There was an error.
    Error,
}

impl<'de> Deserialize<'de> for ApiResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResultVisitor;

        impl<'de> Visitor<'de> for ResultVisitor {
            type Value = ApiResult;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("`ok` or `error`")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                match v {
                    "ok" => Ok(ApiResult::Ok),
                    "error" => Ok(ApiResult::Error),
                    v => Err(serde::de::Error::unknown_variant(v, &["ok", "error"])),
                }
            }
        }

        deserializer.deserialize_str(ResultVisitor)
    }
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
