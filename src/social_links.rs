use serde::{Deserialize, Serialize};

/// The type of social platform a link points to.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum SocialLinkType {
    Facebook,
    Twitter,
    YouTube,
    Twitch,
    GooglePlus,
    Discord,
    RobloxGroup,
    Amazon,
    #[serde(other)]
    Unknown,
}

/// Represents a social link attached to a group or universe.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialLink {
    pub id: u64,
    pub r#type: SocialLinkType,
    pub url: String,
    pub title: String,
}
