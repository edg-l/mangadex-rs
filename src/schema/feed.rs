use serde::Serialize;

use super::OrderType;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum FeedOrder {
    Volume(OrderType),
    Chapter(OrderType),
}
