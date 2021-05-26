use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{errors::ApiErrors, ApiObject, ApiObjectResult, Client, ResourceType, Result};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MappingQuery {
    pub r#type: ResourceType,
    pub ids: Vec<u32>,
}

type MappingId = ApiObject<MappingIdAttributes>;
pub type MappingResponse = Vec<ApiObjectResult<MappingId>>;

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MappingIdAttributes {
    pub r#type: ResourceType,
    pub legacy_id: u32,
    pub new_id: Uuid,
}

impl Client {
    pub async fn legacy_mapping(&self, query: &MappingQuery) -> Result<MappingResponse> {
        let endpoint = self.base_url.join("/legacy/mapping")?;
        let res = self.http.post(endpoint).json(query).send().await?;
        let res = Self::deserialize_response::<MappingResponse, ApiErrors>(res).await?;

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn legacy_mapping() {
        let client = Client::new().unwrap();
        let mapping = client
            .legacy_mapping(&MappingQuery {
                r#type: ResourceType::Manga,
                ids: vec![1],
            })
            .await
            .unwrap();
        assert_eq!(
            mapping[0].data,
            MappingId {
                id: Uuid::parse_str("24b6d026-a7cb-498e-8717-26b2831cf318").unwrap(),
                r#type: ResourceType::MappingId,
                attributes: MappingIdAttributes {
                    r#type: ResourceType::Manga,
                    legacy_id: 1,
                    new_id: Uuid::parse_str("c0ee660b-f9f2-45c3-8068-5123ff53f84a").unwrap()
                }
            }
        )
    }
}
