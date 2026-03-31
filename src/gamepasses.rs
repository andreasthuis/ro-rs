use serde::{Deserialize, Serialize};

/// Represents a Roblox game pass.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GamePass {
    pub id: u64,
    pub name: String,
    pub display_name: Option<String>,
    pub product_id: Option<u64>,
    pub price: Option<i64>,
}
