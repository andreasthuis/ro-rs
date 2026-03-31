use std::sync::Arc;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::bases::BaseAsset;
use crate::client::ClientInner;
use crate::partials::PartialUniverse;

/// Badge award statistics.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BadgeStatistics {
    pub past_day_awarded_count: u64,
    pub awarded_count: u64,
    pub win_rate_percentage: f64,
}

/// Represents a Roblox badge.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Badge {
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
    pub display_name: String,
    pub display_description: Option<String>,
    pub enabled: bool,
    pub icon_image_id: u64,
    pub display_icon_image_id: u64,
    pub created: Option<DateTime<Utc>>,
    pub updated: Option<DateTime<Utc>>,
    pub statistics: BadgeStatistics,
    pub awarding_universe: Option<PartialUniverse>,

    #[serde(skip)]
    pub(crate) client: Option<Arc<ClientInner>>,
}

impl Badge {
    /// Returns a [`BaseAsset`] for the badge icon.
    pub fn icon(&self) -> Option<BaseAsset> {
        self.client
            .clone()
            .map(|c| BaseAsset::new(c, self.icon_image_id))
    }

    /// Returns a [`BaseAsset`] for the badge display icon.
    pub fn display_icon(&self) -> Option<BaseAsset> {
        self.client
            .clone()
            .map(|c| BaseAsset::new(c, self.display_icon_image_id))
    }
}
