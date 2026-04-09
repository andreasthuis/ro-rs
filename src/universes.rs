use std::sync::Arc;
use serde::{Deserialize, Serialize};

use crate::client::ClientInner;
use crate::error::Result;
use crate::utilities::page::{PageIterator, SortOrder};

/// The genre of a Roblox universe.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum UniverseGenre {
    All,
    TownAndCity,
    Fantasy,
    SciFi,
    Ninja,
    Scary,
    Pirate,
    Adventure,
    Sports,
    Rpg,
    Comedy,
    Western,
    Military,
    Building,
}

/// The avatar type used by the universe.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum UniverseAvatarType {
    MorphToR6,
    MorphToR15,
    PlayerChoice,
}

/// Live stats for a universe (concurrent players, active servers, etc.).
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UniverseLiveStats {
    pub total_player_count: u64,
    pub game_count: u64,
    pub player_counts_by_device_type: std::collections::HashMap<String, u64>,
}

/// The voting status for a universe.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UniverseVoteStatus {
    pub can_vote: bool,
    pub user_vote: Option<bool>, // true = upvote, false = downvote, None = no vote
    pub reason_for_not_being_able_to_vote: Option<String>,
}

/// Represents a Roblox universe (game).
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Universe {
    pub id: u64,
    pub root_place_id: Option<u64>,
    pub name: String,
    pub description: Option<String>,
    pub source_name: Option<String>,
    pub source_description: Option<String>,
    pub creator: Option<UniverseCreator>,
    pub price: Option<i64>,
    pub allowed_gear_genres: Option<Vec<String>>,
    pub allowed_gear_categories: Option<Vec<String>>,
    pub is_genre_enforced: Option<bool>,
    pub copying_allowed: Option<bool>,
    pub playing: Option<u64>,
    pub visits: Option<u64>,
    pub max_players: Option<u32>,
    pub created: Option<String>,
    pub updated: Option<String>,
    pub studio_access_to_apis_allowed: Option<bool>,
    pub create_vip_servers_allowed: Option<bool>,
    pub universe_avatar_type: Option<String>,
    pub genre: Option<String>,
    pub is_all_genre: Option<bool>,
    pub is_favorited_by_user: Option<bool>,
    pub favorited_count: Option<u64>,

    #[serde(skip)]
    pub(crate) client: Option<Arc<ClientInner>>,
}

impl Universe {
    /// Fetches the live stats for this universe.
    pub async fn get_live_stats(&self) -> Result<UniverseLiveStats> {
        let client = self.client.as_ref().expect("client not set");
        let url = client
            .url_generator
            .get_url("games", &format!("v1/games/{}/live-stats", self.id));
        let resp = client.http.get(&url).send().await?;
        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(crate::error::RobloxError::from_status(status, &body));
        }
        Ok(resp.json().await?)
    }

    /// Fetches the favorite count for this universe.
    pub async fn get_favorite_count(&self) -> Result<u64> {
        let client = self.client.as_ref().expect("client not set");
        let url = client
            .url_generator
            .get_url("games", &format!("v1/games/{}/favorites/count", self.id));
        let resp = client.http.get(&url).send().await?;
        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(crate::error::RobloxError::from_status(status, &body));
        }
        let data: serde_json::Value = resp.json().await?;
        Ok(data["favoritesCount"].as_u64().unwrap_or(0))
    }

    /// Sets or removes the favorite status for the authenticated user.
    pub async fn set_favorite(&self, favorite: bool) -> Result<()> {
        let client = self.client.as_ref().expect("client not set");
        let url = client
            .url_generator
            .get_url("games", &format!("v1/games/{}/favorites", self.id));
        
        let body = serde_json::json!({ "isFavorited": favorite });
        let resp = client.http.post(&url).json(&body).send().await?;
        
        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(crate::error::RobloxError::from_status(status, &body));
        }
        Ok(())
    }

    /// Fetches the vote status (can the user vote, how did they vote).
    pub async fn get_vote_status(&self) -> Result<UniverseVoteStatus> {
        let client = self.client.as_ref().expect("client not set");
        let url = client
            .url_generator
            .get_url("games", &format!("v1/games/{}/votes", self.id));
        let resp = client.http.get(&url).send().await?;
        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(crate::error::RobloxError::from_status(status, &body));
        }
        Ok(resp.json().await?)
    }

    /// Sets the user's vote (true = upvote, false = downvote).
    pub async fn set_vote(&self, vote: bool) -> Result<()> {
        let client = self.client.as_ref().expect("client not set");
        let url = client
            .url_generator
            .get_url("games", &format!("v1/games/{}/user-votes", self.id));
        
        let body = serde_json::json!({ "vote": vote });
        let resp = client.http.patch(&url).json(&body).send().await?;
        
        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(crate::error::RobloxError::from_status(status, &body));
        }
        Ok(())
    }

    /// Fetches badges awarded by this universe, returning a [`PageIterator`].
    pub fn get_badges(
        &self,
        page_size: u32,
        sort_order: SortOrder,
        max_items: Option<usize>,
    ) -> PageIterator<crate::badges::Badge> {
        let client = self.client.as_ref().expect("client not set");
        let url = client
            .url_generator
            .get_url("badges", &format!("v1/universes/{}/badges", self.id));
        PageIterator::new(client.http.clone(), url, page_size, sort_order, max_items)
    }

    /// Fetches recommended universes based on this universe.
    pub fn get_recommendations(
        &self,
        page_size: u32,
        max_items: Option<usize>,
    ) -> PageIterator<Universe> {
        let client = self.client.as_ref().expect("client not set");
        let url = client
            .url_generator
            .get_url("games", &format!("v1/games/{}/recommendations", self.id));
        // Recommendations usually use Ascending order by default in Roblox APIs
        PageIterator::new(client.http.clone(), url, page_size, SortOrder::Ascending, max_items)
    }
}

/// Represents the creator of a universe (user or group).
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UniverseCreator {
    pub id: u64,
    pub name: String,
    pub creator_type: String,
    pub is_rni_verified: Option<bool>,
    pub has_verified_badge: Option<bool>,
}