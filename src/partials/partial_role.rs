use serde::{Deserialize, Serialize};

/// A lightweight role object returned by many group endpoints.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PartialRole {
    pub id: u64,
    pub name: String,
    pub rank: u8,
}
