use std::sync::Arc;
use crate::client::ClientInner;

/// Represents a Roblox asset ID (image, model, audio, etc.).
#[derive(Debug, Clone)]
pub struct BaseAsset {
    pub id: u64,
    pub(crate) _client: Arc<ClientInner>,
}

impl BaseAsset {
    pub fn new(client: Arc<ClientInner>, id: u64) -> Self {
        Self { id, _client: client }
    }
}
