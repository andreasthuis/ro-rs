use std::sync::Arc;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::client::ClientInner;
use crate::error::Result;

/// The type of presence (what a user is currently doing).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum UserPresenceType {
    Offline = 0,
    Online = 1,
    InGame = 2,
    InStudio = 3,
}

impl UserPresenceType {
    pub fn from_int(v: u8) -> Self {
        match v {
            1 => UserPresenceType::Online,
            2 => UserPresenceType::InGame,
            3 => UserPresenceType::InStudio,
            _ => UserPresenceType::Offline,
        }
    }
}

/// Represents a user's current presence on Roblox.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Presence {
    pub user_presence_type: Option<u8>,
    pub last_location: Option<String>,
    pub place_id: Option<u64>,
    pub root_place_id: Option<u64>,
    pub game_id: Option<String>,
    pub universe_id: Option<u64>,
    pub user_id: Option<u64>,
    pub last_online: Option<DateTime<Utc>>,
}

impl Presence {
    /// Returns the presence type enum value.
    pub fn presence_type(&self) -> UserPresenceType {
        UserPresenceType::from_int(self.user_presence_type.unwrap_or(0))
    }
}

/// Provides methods to query presence data.
pub struct PresenceProvider {
    pub(crate) client: Arc<ClientInner>,
}

impl PresenceProvider {
    pub fn new(client: Arc<ClientInner>) -> Self {
        Self { client }
    }

    /// Fetches presence data for a list of user IDs.
    ///
    /// # Example
    /// ```rust,no_run
    /// # async fn example() {
    /// # use roblox::Client;
    /// let client = Client::new();
    /// let presences = client.presence.get_user_presences(&[1, 2, 3]).await.unwrap();
    /// for p in presences {
    ///     println!("user {:?} presence type: {:?}", p.user_id, p.presence_type());
    /// }
    /// # }
    /// ```
    pub async fn get_user_presences(&self, user_ids: &[u64]) -> Result<Vec<Presence>> {
        let url = self
            .client
            .url_generator
            .get_url("presence", "v1/presence/users");
        let resp = self
            .client
            .http
            .post(&url)
            .json(&serde_json::json!({ "userIds": user_ids }))
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(crate::error::RobloxError::from_status(status, &body));
        }

        let data: serde_json::Value = resp.json().await?;
        let presences: Vec<Presence> =
            serde_json::from_value(data["userPresences"].clone())?;
        Ok(presences)
    }
}
