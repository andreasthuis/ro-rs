use std::sync::Arc;
use crate::client::ClientInner;
use crate::error::Result;
use crate::plugins::Plugin;

/// Represents a Roblox plugin ID.
#[derive(Debug, Clone)]
pub struct BasePlugin {
    pub id: u64,
    pub(crate) client: Arc<ClientInner>,
}

impl BasePlugin {
    pub fn new(client: Arc<ClientInner>, id: u64) -> Self {
        Self { id, client }
    }

    /// Fetches the full plugin data.
    pub async fn expand(&self) -> Result<Plugin> {
        self.client.get_plugin(self.id).await
    }
}
