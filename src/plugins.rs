use std::sync::Arc;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::client::ClientInner;

/// Represents a Roblox plugin.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Plugin {
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
    pub comments_enabled: Option<bool>,
    pub version_id: Option<u64>,
    pub created: Option<DateTime<Utc>>,
    pub updated: Option<DateTime<Utc>>,

    #[serde(skip)]
    pub(crate) _client: Option<Arc<ClientInner>>,
}
