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
