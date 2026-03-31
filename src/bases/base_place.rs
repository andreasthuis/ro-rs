use std::sync::Arc;
use crate::client::ClientInner;
use crate::error::Result;
use crate::places::Place;

/// Represents a Roblox place ID. Places are a form of Asset.
#[derive(Debug, Clone)]
pub struct BasePlace {
    pub id: u64,
    pub(crate) client: Arc<ClientInner>,
}

impl BasePlace {
    pub fn new(client: Arc<ClientInner>, id: u64) -> Self {
        Self { id, client }
    }

    /// Fetches the full place data.
    pub async fn expand(&self) -> Result<Place> {
        self.client.get_place(self.id).await
    }
}
