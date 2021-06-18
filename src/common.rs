use serde::{Deserialize, Deserializer, Serialize};
use std::borrow::Cow;

pub(crate) trait UrlSerdeQS {
    fn query_qs<T: Serialize>(self, query: &T) -> Self;
}

impl UrlSerdeQS for url::Url {
    fn query_qs<T: Serialize>(mut self, query: &T) -> Self {
        self.set_query(Some(
            &serde_qs::to_string(query).expect("Failed to encode query string"),
        ));
        self
    }
}

pub(crate) fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + Deserialize<'de>,
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}

/// So this trait isn't ideal, but it's the only practical way I could figure out on stable Rust,
// without specialization. With that in mind, please don't expose it to the public API.
pub(crate) trait FromResponse: Sized {
    type Response;

    fn from_response(res: Self::Response) -> Self;
}

pub(crate) trait Endpoint {
    type Query: Serialize;
    type Body: Serialize;
    type Response: FromResponse;

    fn path(&self) -> Cow<str>;

    fn method(&self) -> reqwest::Method {
        reqwest::Method::GET
    }

    fn require_auth(&self) -> bool {
        false
    }

    fn query(&self) -> Option<&Self::Query> {
        None
    }

    fn body(&self) -> Option<&Self::Body> {
        None
    }
}
