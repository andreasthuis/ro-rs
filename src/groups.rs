use std::sync::Arc;
use serde::{Deserialize, Serialize};

use crate::client::ClientInner;
use crate::error::Result;
use crate::partials::{PartialUser, PartialRole};
use crate::roles::Role;
use crate::shout::Shout;
use crate::utilities::page::{PageIterator, SortOrder};

/// Represents a Roblox group.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub owner: Option<PartialUser>,
    pub shout: Option<Shout>,
    pub member_count: u64,
    pub is_builders_club_only: bool,
    pub public_entry_allowed: bool,
    #[serde(default)]
    pub is_locked: bool,
    pub has_verified_badge: bool,

    #[serde(skip)]
    pub(crate) client: Option<Arc<ClientInner>>,
}

impl Group {
    /// Updates the group shout. Returns `(old_shout, new_shout)`.
    pub async fn update_shout(&mut self, message: &str) -> Result<(Option<Shout>, Option<Shout>)> {
        let client = self.client.as_ref().expect("client not set");
        let url = client
            .url_generator
            .get_url("groups", &format!("v1/groups/{}/status", self.id));

        let resp = client
            .http
            .patch(&url)
            .json(&serde_json::json!({ "message": message }))
            .send()
            .await?;

        if !resp.status().is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(crate::error::RobloxError::from_status(
                reqwest::StatusCode::from_u16(400).unwrap(),
                &body,
            ));
        }

        let data: serde_json::Value = resp.json().await?;
        let old_shout = self.shout.clone();
        let new_shout = if data.is_null() {
            None
        } else {
            Some(serde_json::from_value::<Shout>(data)?)
        };
        self.shout = new_shout.clone();
        Ok((old_shout, new_shout))
    }

    /// Fetches the list of roles (ranks) in this group.
    pub async fn get_roles(&self) -> Result<Vec<Role>> {
        let client = self.client.as_ref().expect("client not set");
        let url = client
            .url_generator
            .get_url("groups", &format!("v1/groups/{}/roles", self.id));
        let resp = client.http.get(&url).send().await?;
        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(crate::error::RobloxError::from_status(status, &body));
        }
        let data: serde_json::Value = resp.json().await?;
        let roles: Vec<Role> = serde_json::from_value(data["roles"].clone())?;
        Ok(roles)
    }

    /// Fetches a paginated iterator of members in this group.
    pub fn get_members(
        &self,
        page_size: u32,
        sort_order: SortOrder,
        max_items: Option<usize>,
    ) -> PageIterator<GroupMember> {
        let client = self.client.as_ref().expect("client not set");
        let url = client
            .url_generator
            .get_url("groups", &format!("v1/groups/{}/users", self.id));
        PageIterator::new(client.http.clone(), url, page_size, sort_order, max_items)
    }

    /// Fetches group settings (requires authentication as the group owner).
    pub async fn get_settings(&self) -> Result<GroupSettings> {
        let client = self.client.as_ref().expect("client not set");
        let url = client
            .url_generator
            .get_url("groups", &format!("v1/groups/{}/settings", self.id));
        let resp = client.http.get(&url).send().await?;
        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(crate::error::RobloxError::from_status(status, &body));
        }
        Ok(resp.json().await?)
    }

    /// Looks up a user's role in this group. Returns `None` if the user is not a member.
    pub async fn get_member_role(&self, user_id: u64) -> Result<Option<PartialRole>> {
        let client = self.client.as_ref().expect("client not set");
        let url = client.url_generator.get_url(
            "groups",
            &format!("v1/users/{}/groups/roles", user_id),
        );
        let resp = client.http.get(&url).send().await?;
        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(crate::error::RobloxError::from_status(status, &body));
        }
        let data: serde_json::Value = resp.json().await?;
        let entries = data["data"].as_array().cloned().unwrap_or_default();
        for entry in entries {
            if entry["group"]["id"].as_u64() == Some(self.id) {
                let role: PartialRole = serde_json::from_value(entry["role"].clone())?;
                return Ok(Some(role));
            }
        }
        Ok(None)
    }
}

/// A member entry returned from the group users endpoint.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupMember {
    pub user: PartialUser,
    pub role: PartialRole,
}

/// Group settings object.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupSettings {
    pub is_approval_required: bool,
    pub is_builders_club_required: bool,
    pub are_enemies_allowed: bool,
    pub are_group_games_visible: bool,
    pub are_group_funds_visible: bool,
    pub are_group_policies_visible: Option<bool>,
}
