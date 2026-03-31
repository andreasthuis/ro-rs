use serde::{Deserialize, Serialize};

/// Represents a role (rank) within a Roblox group.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Role {
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
    pub rank: u8,
    pub member_count: Option<u64>,
}
