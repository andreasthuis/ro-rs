use std::sync::Arc;
use crate::client::ClientInner;

/// Represents a Roblox gamepass ID.
#[derive(Debug, Clone)]
pub struct BaseGamePass {
    pub id: u64,
    pub(crate) _client: Arc<ClientInner>,
}

impl BaseGamePass {
    pub fn new(client: Arc<ClientInner>, id: u64) -> Self {
        Self { id, _client: client }
    }
}
