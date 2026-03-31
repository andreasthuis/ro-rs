/// Generates URLs for the Roblox API based on subdomain and path.
///
/// Roblox's API is spread across multiple subdomains, e.g.:
/// - `users.roblox.com` — user data
/// - `groups.roblox.com` — group data
/// - `presence.roblox.com` — presence
#[derive(Debug, Clone)]
pub struct UrlGenerator {
    pub base_url: String,
}

impl UrlGenerator {
    /// Creates a new `UrlGenerator` with the given base URL (e.g. `"roblox.com"`).
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
        }
    }

    /// Returns the full subdomain URL, e.g. `"https://users.roblox.com"`.
    pub fn get_subdomain(&self, subdomain: &str) -> String {
        format!("https://{}.{}", subdomain, self.base_url)
    }

    /// Returns a full URL for a given subdomain and path, e.g.
    /// `"https://users.roblox.com/v1/users/1"`.
    pub fn get_url(&self, subdomain: &str, path: &str) -> String {
        format!("https://{}.{}/{}", subdomain, self.base_url, path)
    }
}

impl Default for UrlGenerator {
    fn default() -> Self {
        Self::new("roblox.com")
    }
}
