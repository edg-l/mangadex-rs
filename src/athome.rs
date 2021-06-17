use crate::{Client, FromResponse, Result};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AtHomeServerRes {
    pub base_url: String,
}

impl FromResponse for AtHomeServerRes {
    type Response = Self;

    fn from_response(res: Self::Response) -> Self {
        res
    }
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AtHomeServerReq<'a> {
    #[serde(skip)]
    chapter_id: &'a Uuid,
    force_port443: bool,
}

impl_endpoint! {
    GET ("/at-home/server/{:x}", chapter_id),
    #[query] AtHomeServerReq<'_>,
    #[no_send] AtHomeServerRes
}

impl AtHomeServerReq<'_> {
    pub async fn send(&self, client: &Client) -> Result<Url> {
        let r = client.send_request(self).await?;
        Ok(Url::parse(&r.base_url)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn at_home() {
        let client = Client::default();
        let chapter_id = &uuid::Uuid::parse_str("0e94efb5-6cb5-49fd-b522-51b4460c9821").unwrap();
        AtHomeServerReq {
            chapter_id,
            force_port443: false,
        }
        .send(&client)
        .await
        .expect("Failed to resolve at-home request");
    }

    #[tokio::test]
    async fn at_home_force443() {
        let client = Client::default();
        let chapter_id = &uuid::Uuid::parse_str("0e94efb5-6cb5-49fd-b522-51b4460c9821").unwrap();
        let res = AtHomeServerReq {
            chapter_id,
            force_port443: false,
        }
        .send(&client)
        .await
        .expect("Failed to resolve at-home request");
        assert_eq!(res.port_or_known_default(), Some(443));
    }
}
