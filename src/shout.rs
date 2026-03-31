use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::partials::PartialUser;

/// Represents a group's shout (status message).
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Shout {
    pub body: String,
    pub poster: Option<PartialUser>,
    pub created: Option<DateTime<Utc>>,
    pub updated: Option<DateTime<Utc>>,
}
