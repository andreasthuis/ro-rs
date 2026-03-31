use serde::{Deserialize, Serialize};

/// A lightweight universe object.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PartialUniverse {
    pub id: u64,
    pub name: String,
    pub root_place_id: Option<u64>,
}
