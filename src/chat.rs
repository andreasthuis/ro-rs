use std::sync::Arc;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::client::ClientInner;
use crate::error::Result;

/// Represents metadata about the Roblox chat system.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatMetadata {
    pub is_active_chat_user_timeout_seconds: Option<u64>,
    pub chat_max_group_size: Option<u32>,
    pub chat_auto_mute_timeout_aftershort_timed_out_seconds: Option<u64>,
    pub language_for_privacy_mode_filtering: Option<String>,
    pub max_conversation_title_length: Option<u32>,
    pub number_of_mos_recently_shown_conversations: Option<u32>,
    pub page_size_of_conversation_titles: Option<u32>,
    pub page_size_of_conversations: Option<u32>,
    pub page_size_of_messages_in_conversations: Option<u32>,
    pub page_size_of_search_results: Option<u32>,
}

/// A message inside a Roblox chat conversation.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessage {
    pub id: Option<String>,
    pub sender_type: Option<String>,
    pub sent: Option<DateTime<Utc>>,
    pub read: Option<bool>,
    pub message_type: Option<String>,
    pub sender_target_id: Option<u64>,
    pub content: Option<String>,
    pub link: Option<serde_json::Value>,
    pub decoration_type: Option<String>,
}

/// A Roblox chat conversation.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatConversation {
    pub id: u64,
    pub title: Option<String>,
    pub initiator: Option<serde_json::Value>,
    pub has_unread_messages: Option<bool>,
    pub participants: Option<Vec<serde_json::Value>>,
    pub conversation_type: Option<String>,
    pub conversation_title: Option<serde_json::Value>,
    pub last_updated: Option<DateTime<Utc>>,
}

/// Provides methods to interact with the Roblox chat API.
pub struct ChatProvider {
    pub(crate) client: Arc<ClientInner>,
}

impl ChatProvider {
    pub fn new(client: Arc<ClientInner>) -> Self {
        Self { client }
    }

    /// Fetches the chat system metadata.
    pub async fn get_metadata(&self) -> Result<ChatMetadata> {
        let url = self
            .client
            .url_generator
            .get_url("chat", "v2/metadata");
        let resp = self.client.http.get(&url).send().await?;
        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(crate::error::RobloxError::from_status(status, &body));
        }
        Ok(resp.json().await?)
    }

    /// Fetches the authenticated user's conversations, paginated by `page_number`.
    pub async fn get_conversations(
        &self,
        page_number: u32,
        page_size: u32,
    ) -> Result<Vec<ChatConversation>> {
        let url = self
            .client
            .url_generator
            .get_url("chat", "v2/get-user-conversations");
        let resp = self
            .client
            .http
            .get(&url)
            .query(&[
                ("pageNumber", page_number.to_string()),
                ("pageSize", page_size.to_string()),
            ])
            .send()
            .await?;
        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(crate::error::RobloxError::from_status(status, &body));
        }
        Ok(resp.json().await?)
    }

    /// Fetches messages from a specific conversation.
    pub async fn get_messages(
        &self,
        conversation_id: u64,
        page_size: u32,
        exclusive_start_message_id: Option<String>,
    ) -> Result<Vec<ChatMessage>> {
        let url = self
            .client
            .url_generator
            .get_url("chat", "v2/get-messages");
        let mut query = vec![
            ("conversationId", conversation_id.to_string()),
            ("pageSize", page_size.to_string()),
        ];
        if let Some(ref id) = exclusive_start_message_id {
            query.push(("exclusiveStartMessageId", id.clone()));
        }
        let resp = self
            .client
            .http
            .get(&url)
            .query(&query)
            .send()
            .await?;
        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(crate::error::RobloxError::from_status(status, &body));
        }
        Ok(resp.json().await?)
    }

    /// Sends a message to a conversation.
    pub async fn send_message(
        &self,
        message: &str,
        conversation_id: u64,
    ) -> Result<serde_json::Value> {
        let url = self
            .client
            .url_generator
            .get_url("chat", "v2/send-message");
        let resp = self
            .client
            .http
            .post(&url)
            .json(&serde_json::json!({
                "message": message,
                "conversationId": conversation_id
            }))
            .send()
            .await?;
        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(crate::error::RobloxError::from_status(status, &body));
        }
        Ok(resp.json().await?)
    }
}
