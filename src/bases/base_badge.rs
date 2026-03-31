use std::sync::Arc;
use crate::client::ClientInner;
use crate::error::Result;
use crate::badges::Badge;

/// Represents a Roblox badge ID.
#[derive(Debug, Clone)]
pub struct BaseBadge {
    pub id: u64,
    pub(crate) client: Arc<ClientInner>,
}

impl BaseBadge {
    pub fn new(client: Arc<ClientInner>, id: u64) -> Self {
        Self { id, client }
    }

    /// Fetches the full badge data.
    pub async fn expand(&self) -> Result<Badge> {
        self.client.get_badge(self.id).await
    }
}
