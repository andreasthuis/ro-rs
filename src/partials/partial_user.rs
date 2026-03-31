use serde::{Deserialize, Serialize};

/// A lightweight user object returned by many endpoints that do not need
/// the full user profile (name + id only).
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PartialUser {
    pub id: u64,
    pub name: Option<String>,
    pub display_name: Option<String>,
    pub has_verified_badge: Option<bool>,
}

/// A partial user returned when looking up a user by username, also
/// carrying the requested (input) username.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestedUsernamePartialUser {
    pub id: u64,
    pub name: String,
    pub display_name: String,
    pub has_verified_badge: Option<bool>,
    pub requested_username: Option<String>,
}
