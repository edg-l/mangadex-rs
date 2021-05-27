use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{ApiData, ApiObject, Client, Result};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MappingType {
    Group,
    Manga,
    Chapter,
    Tag,
}

#[derive(Debug, Serialize, Clone, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MappingQuery {
    pub r#type: MappingType,
    pub ids: Vec<u32>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MappingIdType {
    MappingId,
}

type MappingId = ApiObject<MappingIdAttributes, MappingIdType>;
pub type MappingIdResponse = Result<ApiData<MappingId>>;

#[derive(Debug, Deserialize, Clone, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MappingIdAttributes {
    pub r#type: MappingType,
    pub legacy_id: u32,
    pub new_id: Uuid,
}

impl Client {
    pub async fn legacy_mapping(&self, query: &MappingQuery) -> Result<Vec<MappingIdResponse>> {
        let endpoint = self.base_url.join("/legacy/mapping")?;
        let res = self.http.post(endpoint).json(query).send().await?;

        Self::json_api_result_vec(res).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn legacy_mapping() {
        let client = Client::default();
        let mapping = client
            .legacy_mapping(&MappingQuery {
                r#type: MappingType::Manga,
                ids: vec![1],
            })
            .await
            .unwrap();
        assert_eq!(
            mapping[0].as_ref().unwrap().data,
            MappingId {
                id: Uuid::parse_str("24b6d026-a7cb-498e-8717-26b2831cf318").unwrap(),
                r#type: MappingIdType::MappingId,
                attributes: MappingIdAttributes {
                    r#type: MappingType::Manga,
                    legacy_id: 1,
                    new_id: Uuid::parse_str("c0ee660b-f9f2-45c3-8068-5123ff53f84a").unwrap()
                }
            }
        )
    }
}
