use std::sync::Arc;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::bases::BaseUser;
use crate::client::ClientInner;
use crate::error::Result;
use crate::utilities::page::{PageIterator, SortOrder};

/// Represents a Roblox user's full profile data.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i64,
    pub name: String,
    pub display_name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub is_banned: bool,
    pub created: Option<DateTime<Utc>>,
    #[serde(default)]
    pub has_verified_badge: bool,
    pub external_app_display_name: Option<String>,

    #[serde(skip)]
    pub(crate) client: Option<Arc<ClientInner>>,
}

impl User {
    /// Returns a [`BaseUser`] for this user (lightweight, no extra API calls).
    pub fn to_base(&self) -> BaseUser {
        BaseUser::new(self.client.clone().expect("client not set"), self.id)
    }

    /// Gets the user's friend count.
    pub async fn get_friend_count(&self) -> Result<i64> {
        let client = self.client.as_ref().expect("client not set");
        let url = client
            .url_generator
            .get_url("friends", &format!("v1/users/{}/friends/count", self.id));
        let resp = client.http.get(&url).send().await?;
        check_status(&resp)?;
        let data: serde_json::Value = resp.json().await?;
        Ok(data["count"].as_i64().unwrap_or(0))
    }

    /// Gets the user's follower count.
    pub async fn get_follower_count(&self) -> Result<i64> {
        let client = self.client.as_ref().expect("client not set");
        let url = client
            .url_generator
            .get_url("friends", &format!("v1/users/{}/followers/count", self.id));
        let resp = client.http.get(&url).send().await?;
        check_status(&resp)?;
        let data: serde_json::Value = resp.json().await?;
        Ok(data["count"].as_i64().unwrap_or(0))
    }

    /// Gets the user's following count.
    pub async fn get_following_count(&self) -> Result<i64> {
        let client = self.client.as_ref().expect("client not set");
        let url = client
            .url_generator
            .get_url("friends", &format!("v1/users/{}/followings/count", self.id));
        let resp = client.http.get(&url).send().await?;
        check_status(&resp)?;
        let data: serde_json::Value = resp.json().await?;
        Ok(data["count"].as_i64().unwrap_or(0))
    }

    /// Returns a [`PageIterator`] over this user's followers.
    pub fn get_followers(
        &self,
        page_size: u32,
        sort_order: SortOrder,
        max_items: Option<usize>,
    ) -> PageIterator<BaseUserData> {
        let client = self.client.as_ref().expect("client not set");
        let url = client
            .url_generator
            .get_url("friends", &format!("v1/users/{}/followers", self.id));
        PageIterator::new(client.http.clone(), url, page_size, sort_order, max_items)
    }

    /// Returns a [`PageIterator`] over the users that this user is following.
    pub fn get_followings(
        &self,
        page_size: u32,
        sort_order: SortOrder,
        max_items: Option<usize>,
    ) -> PageIterator<BaseUserData> {
        let client = self.client.as_ref().expect("client not set");
        let url = client
            .url_generator
            .get_url("friends", &format!("v1/users/{}/followings", self.id));
        PageIterator::new(client.http.clone(), url, page_size, sort_order, max_items)
    }

    /// Returns a list of this user's friends.
    pub async fn get_friends(&self) -> Result<Vec<FriendData>> {
        let client = self.client.as_ref().expect("client not set");
        let url = client
            .url_generator
            .get_url("friends", &format!("v1/users/{}/friends", self.id));
        let resp = client.http.get(&url).send().await?;
        check_status(&resp)?;
        let data: serde_json::Value = resp.json().await?;
        let friends = serde_json::from_value(data["data"].clone())?;
        Ok(friends)
    }
}

/// Minimal user data item returned inside list endpoints (followers, followings, etc.).
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseUserData {
    pub id: u64,
    pub name: String,
    pub display_name: String,
    #[serde(default)]
    pub has_verified_badge: bool,
}

/// User data with online status, returned from the friends list endpoint.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FriendData {
    pub id: i64,
    pub name: String,
    pub display_name: String,
    #[serde(default)]
    pub is_online: bool,
    #[serde(default)]
    pub is_deleted: bool,
    #[serde(default)]
    pub has_verified_badge: bool,
}

fn check_status(resp: &reqwest::Response) -> Result<()> {
    if resp.status().is_success() {
        Ok(())
    } else {
        Err(crate::error::RobloxError::from_status(
            resp.status(),
            "Request failed",
        ))
    }
}
