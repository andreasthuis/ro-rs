use std::sync::Arc;
use crate::client::ClientInner;
use crate::error::Result;
use crate::groups::Group;

/// Represents a Roblox group ID without necessarily having loaded any group data.
///
/// Call [`BaseGroup::expand`] to fetch the full [`Group`] object.
#[derive(Debug, Clone)]
pub struct BaseGroup {
    pub id: u64,
    pub(crate) client: Arc<ClientInner>,
}

impl BaseGroup {
    pub fn new(client: Arc<ClientInner>, id: u64) -> Self {
        Self { id, client }
    }

    /// Fetches the full group data for this group ID.
    pub async fn expand(&self) -> Result<Group> {
        self.client.get_group(self.id).await
    }
}
