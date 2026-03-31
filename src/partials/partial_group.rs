use serde::{Deserialize, Serialize};

/// A lightweight group object used where only id/name are needed.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PartialGroup {
    pub id: u64,
    pub name: String,
    pub has_verified_badge: Option<bool>,
}
