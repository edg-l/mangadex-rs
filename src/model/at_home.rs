use serde::Deserialize;

use crate::FromResponse;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AtHomeServer {
    pub base_url: String,
}

impl FromResponse for AtHomeServer {
    type Response = Self;

    fn from_response(res: Self::Response) -> Self {
        res
    }
}
