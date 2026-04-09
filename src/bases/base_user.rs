use std::sync::Arc;
use crate::client::ClientInner;
use crate::error::Result;
use crate::users::User;

/// Represents a Roblox user ID without necessarily having loaded any user data.
///
/// Call [`BaseUser::expand`] to fetch the full [`User`] object.
#[derive(Debug, Clone)]
pub struct BaseUser {
    pub id: i64,
    pub(crate) client: Arc<ClientInner>,
}

impl BaseUser {
    pub fn new(client: Arc<ClientInner>, id: i64) -> Self {
        Self { id, client }
    }

    /// Fetches the full user profile for this user ID.
    pub async fn expand(&self) -> Result<User> {
        self.client.get_user(self.id).await
    }
}
