//! MangaDex@Home server

use reqwest::Url;
use serde::Serialize;
use uuid::Uuid;

use crate::model::at_home::*;
use crate::{Client, Result};

/// Get MangaDex@Home server URL
///
/// Call to `GET /at-home/server/{chapterId}
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetAtHomeServer<'a> {
    /// Chapter ID
    #[serde(skip)]
    pub chapter_id: &'a Uuid,

    /// Force selecting from MangaDex@Home servers that use the standard HTTPS port 443.
    ///
    /// While the conventional port for HTTPS traffic is 443 and servers are encouraged to use it,
    /// it is not a hard requirement as it technically isn't anything special.
    ///
    /// However, some misbehaving school/office network will at time block traffic to non-standard
    /// ports, and setting this flag to true will ensure selection of a server that uses these.
    pub force_port443: bool,
}

impl_endpoint! {
    GET ("/at-home/server/{:x}", chapter_id),
    #[query] GetAtHomeServer<'_>,
    #[no_send] AtHomeServer
}

impl GetAtHomeServer<'_> {
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
        GetAtHomeServer {
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
        let res = GetAtHomeServer {
            chapter_id,
            force_port443: false,
        }
        .send(&client)
        .await
        .expect("Failed to resolve at-home request");
        assert_eq!(res.port_or_known_default(), Some(443));
    }
}
