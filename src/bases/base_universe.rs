use std::sync::Arc;
use crate::client::ClientInner;
use crate::error::Result;
use crate::universes::Universe;

/// Represents a Roblox universe ID.
#[derive(Debug, Clone)]
pub struct BaseUniverse {
    pub id: u64,
    pub(crate) client: Arc<ClientInner>,
}

impl BaseUniverse {
    pub fn new(client: Arc<ClientInner>, id: u64) -> Self {
        Self { id, client }
    }

    /// Fetches the full universe data.
    pub async fn expand(&self) -> Result<Universe> {
        self.client.get_universe(self.id).await
    }
}
