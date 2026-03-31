use std::sync::Arc;
use serde::{Deserialize, Serialize};

use crate::client::ClientInner;
use crate::error::Result;

/// The authenticated user's promotion channels.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PromotionChannels {
    pub facebook: Option<String>,
    pub twitter: Option<String>,
    pub youtube: Option<String>,
    pub twitch: Option<String>,
    pub guilded: Option<String>,
}

/// Provides methods to interact with the authenticated user's account settings.
pub struct AccountProvider {
    pub(crate) client: Arc<ClientInner>,
}

impl AccountProvider {
    pub fn new(client: Arc<ClientInner>) -> Self {
        Self { client }
    }

    /// Returns the authenticated user's description.
    pub async fn get_description(&self) -> Result<String> {
        let url = self
            .client
            .url_generator
            .get_url("accountinformation", "v1/description");
        let resp = self.client.http.get(&url).send().await?;
        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(crate::error::RobloxError::from_status(status, &body));
        }
        let data: serde_json::Value = resp.json().await?;
        Ok(data["description"].as_str().unwrap_or("").to_string())
    }

    /// Updates the authenticated user's description.
    pub async fn set_description(&self, description: &str) -> Result<()> {
        let url = self
            .client
            .url_generator
            .get_url("accountinformation", "v1/description");
        let resp = self
            .client
            .http
            .post(&url)
            .json(&serde_json::json!({ "description": description }))
            .send()
            .await?;
        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(crate::error::RobloxError::from_status(status, &body));
        }
        Ok(())
    }

    /// Returns the authenticated user's promotion channels.
    pub async fn get_promotion_channels(&self) -> Result<PromotionChannels> {
        let url = self
            .client
            .url_generator
            .get_url("accountinformation", "v1/promotion-channels");
        let resp = self.client.http.get(&url).send().await?;
        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(crate::error::RobloxError::from_status(status, &body));
        }
        Ok(resp.json().await?)
    }

    /// Returns the authenticated user's gender.
    pub async fn get_gender(&self) -> Result<u8> {
        let url = self
            .client
            .url_generator
            .get_url("accountinformation", "v1/gender");
        let resp = self.client.http.get(&url).send().await?;
        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(crate::error::RobloxError::from_status(status, &body));
        }
        let data: serde_json::Value = resp.json().await?;
        Ok(data["gender"].as_u64().unwrap_or(1) as u8)
    }
}
