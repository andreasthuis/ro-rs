use std::sync::Arc;
use serde::{Deserialize, Serialize};

use crate::client::ClientInner;

/// Represents a Roblox place.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Place {
    pub id: u64,
    pub source_name: Option<String>,
    pub source_description: Option<String>,
    pub url: Option<String>,
    pub builder: Option<String>,
    pub builder_id: Option<u64>,
    pub is_playable: Option<bool>,
    pub reason_prohibited: Option<String>,
    pub universe_id: Option<u64>,
    pub universe_root_place_id: Option<u64>,
    pub price: Option<i64>,
    pub image_token: Option<String>,

    #[serde(skip)]
    pub(crate) _client: Option<Arc<ClientInner>>,
}
