//! Legacy mapping

use derive_builder::Builder;
use serde::Serialize;

use crate::model::legacy::*;

/// Legacy id mapping
///
/// Call to `POST /legacy/mapping`
#[derive(Debug, Builder, Serialize, Clone, PartialEq, Eq)]
#[builder(setter(into))]
#[serde(rename_all = "camelCase")]
pub struct LegacyMapping {
    /// Legacy id type
    #[builder(setter(name = "query_type"))]
    pub r#type: MappingType,

    /// List of legacy ids
    #[builder(setter(each = "add_id"))]
    pub ids: Vec<u32>,
}

impl_endpoint! {
    POST "/legacy/mapping",
    #[body] LegacyMapping,
    Vec<MappingIdResponse>
}

#[cfg(test)]
mod tests {
    use crate::Client;

    use super::*;
    use httpmock::{Method::POST, MockServer};
    use pretty_assertions::assert_eq;
    use serde_json::json;
    use uuid::Uuid;

    #[tokio::test]
    async fn legacy_mapping() {
        let server = MockServer::start_async().await;
        let mock = server
            .mock_async(|when, then| {
                when.method(POST)
                    .path("/legacy/mapping")
                    .header("Content-Type", "application/json")
                    .json_body(json!({
                        "type": "manga",
                        "ids": [1]
                    }));
                then.header("Content-Type", "application/json")
                    .json_body(json!([
                        {
                            "result": "ok",
                            "data": {
                                "id": "24b6d026-a7cb-498e-8717-26b2831cf318",
                                "type": "mapping_id",
                                "attributes": {
                                    "type": "manga",
                                    "legacyId": 1,
                                    "newId": "c0ee660b-f9f2-45c3-8068-5123ff53f84a",
                                },
                            }
                        }
                    ]));
            })
            .await;

        let client = Client::new(&server.base_url()).unwrap();
        let mappings = LegacyMapping {
            r#type: MappingType::Manga,
            ids: vec![1],
        }
        .send(&client)
        .await
        .expect("Failed to parse");

        mock.assert_async().await;
        assert_eq!(mappings.len(), 1);

        let mapping = mappings[0].as_ref().unwrap();
        assert_eq!(mapping.relationships.len(), 0);
        assert_eq!(
            mapping.data,
            MappingId {
                id: Uuid::parse_str("24b6d026-a7cb-498e-8717-26b2831cf318").unwrap(),
                r#type: MappingIdType::MappingId,
                attributes: MappingIdAttributes {
                    r#type: MappingType::Manga,
                    legacy_id: 1,
                    new_id: Uuid::parse_str("c0ee660b-f9f2-45c3-8068-5123ff53f84a").unwrap()
                }
            }
        );
    }
}
