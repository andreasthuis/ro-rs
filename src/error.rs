use thiserror::Error;

/// All errors that can be returned by this crate.
#[derive(Debug, Error)]
pub enum RobloxError {
    /// An HTTP transport error occurred.
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// A JSON parse error occurred.
    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),

    /// A generic bad request (HTTP 400).
    #[error("Bad request: {0}")]
    BadRequest(String),

    /// The requested resource was not found (HTTP 404).
    #[error("Not found: {0}")]
    NotFound(String),

    /// The client is not authenticated (HTTP 401).
    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    /// The client is forbidden from accessing the resource (HTTP 403).
    #[error("Forbidden: {0}")]
    Forbidden(String),

    /// Too many requests were sent (HTTP 429).
    #[error("Too many requests")]
    TooManyRequests,

    /// The server returned an unexpected error (HTTP 5xx).
    #[error("Server error: {0}")]
    ServerError(String),

    /// An invalid user ID or username was provided.
    #[error("User not found")]
    UserNotFound,

    /// An invalid group ID was provided.
    #[error("Group not found")]
    GroupNotFound,

    /// An invalid asset ID was provided.
    #[error("Asset not found")]
    AssetNotFound,

    /// An invalid badge ID was provided.
    #[error("Badge not found")]
    BadgeNotFound,

    /// An invalid place ID was provided.
    #[error("Place not found")]
    PlaceNotFound,

    /// An invalid universe ID was provided.
    #[error("Universe not found")]
    UniverseNotFound,

    /// An invalid plugin ID was provided.
    #[error("Plugin not found")]
    PluginNotFound,

    /// An unexpected error occurred.
    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl RobloxError {
    /// Convert an HTTP status code and body into a `RobloxError`.
    pub fn from_status(status: reqwest::StatusCode, body: &str) -> Self {
        match status.as_u16() {
            400 => RobloxError::BadRequest(body.to_string()),
            401 => RobloxError::Unauthorized(body.to_string()),
            403 => RobloxError::Forbidden(body.to_string()),
            404 => RobloxError::NotFound(body.to_string()),
            429 => RobloxError::TooManyRequests,
            500..=599 => RobloxError::ServerError(body.to_string()),
            _ => RobloxError::Unknown(format!("Status {}: {}", status, body)),
        }
    }
}

pub type Result<T> = std::result::Result<T, RobloxError>;
