use crate::{errors::ApiErrors, Client, Result};
use reqwest::Url;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AtHomeServer {
    base_url: String,
}

impl Client {
    pub async fn at_home(&self, chapter_id: Uuid, force_port443: bool) -> Result<Url> {
        let mut endpoint = self
            .base_url
            .join("/at-home/server/")?
            .join(&format!("{}", chapter_id))?;

        if force_port443 {
            endpoint
                .query_pairs_mut()
                .append_pair("forcePort443", "true");
        }

        let res = self.http.get(endpoint).send().await?;
        let res = Self::deserialize_response::<AtHomeServer, ApiErrors>(res).await?;

        Ok(Url::parse(&res.base_url)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn at_home() {
        let client = Client::new().unwrap();
        let chapter_uuid = uuid::Uuid::parse_str("0e94efb5-6cb5-49fd-b522-51b4460c9821").unwrap();

        client.at_home(chapter_uuid, false).await.unwrap();
    }

    #[tokio::test]
    async fn at_home_force443() {
        let client = Client::new().unwrap();
        let chapter_uuid = uuid::Uuid::parse_str("0e94efb5-6cb5-49fd-b522-51b4460c9821").unwrap();

        let url = client.at_home(chapter_uuid, true).await.unwrap();
        assert_eq!(url.port_or_known_default(), Some(443));
    }
}
