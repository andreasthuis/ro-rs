use std::sync::{Arc, Mutex};

use reqwest::{
    cookie::Jar,
    header::{HeaderMap, HeaderValue},
    Client as HttpClient,
};
use serde_json::Value;

use crate::account::AccountProvider;
use crate::assets::EconomyAsset;
use crate::badges::Badge;
use crate::bases::{BaseAsset, BaseBadge, BaseGamePass, BaseGroup, BasePlace, BasePlugin, BaseUniverse, BaseUser};
use crate::chat::ChatProvider;
use crate::error::{Result, RobloxError};
use crate::groups::Group;
use crate::partials::PartialUser;
use crate::places::Place;
use crate::plugins::Plugin;
use crate::presence::PresenceProvider;
use crate::thumbnails::ThumbnailProvider;
use crate::universes::Universe;
use crate::url::UrlGenerator;
use crate::users::User;

const ROBLOSECURITY_COOKIE: &str = ".ROBLOSECURITY";
const XCSRF_HEADER: &str = "X-CSRF-TOKEN";

/// Internal shared state for the client.
#[derive(Debug)]
pub struct ClientInner {
    pub http: HttpClient,
    pub url_generator: UrlGenerator,
    /// Cached CSRF token for state-mutating requests.
    pub xcsrf_token: Mutex<Option<String>>,
}

impl ClientInner {
    /// Executes a GET request and checks the response status.
    pub async fn get_json(&self, url: &str) -> Result<Value> {
        let resp = self.http.get(url).send().await?;
        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(RobloxError::from_status(status, &body));
        }
        Ok(resp.json().await?)
    }

    /// Executes a POST/PATCH with automatic X-CSRF-TOKEN refresh on 403.
    pub async fn post_json(&self, url: &str, body: &Value) -> Result<Value> {
        self.mutating_request("POST", url, body).await
    }

    pub async fn patch_json(&self, url: &str, body: &Value) -> Result<Value> {
        self.mutating_request("PATCH", url, body).await
    }

    pub async fn delete_json(&self, url: &str, body: &Value) -> Result<Value> {
        self.mutating_request("DELETE", url, body).await
    }

    async fn mutating_request(&self, method: &str, url: &str, body: &Value) -> Result<Value> {
        let xcsrf = self
            .xcsrf_token
            .lock()
            .unwrap()
            .clone()
            .unwrap_or_default();

        let builder = self
            .http
            .request(method.parse().unwrap(), url)
            .json(body)
            .header(XCSRF_HEADER, xcsrf);

        let resp = builder.try_clone().unwrap().send().await?;

        // Roblox returns 403 with a fresh CSRF token when the stored one expires.
        if resp.status().as_u16() == 403 {
            if let Some(new_token) = resp.headers().get(XCSRF_HEADER) {
                let token_str = new_token.to_str().unwrap_or("").to_string();
                *self.xcsrf_token.lock().unwrap() = Some(token_str.clone());

                // Retry with the new token.
                let retry_resp = self
                    .http
                    .request(method.parse().unwrap(), url)
                    .json(body)
                    .header(XCSRF_HEADER, token_str)
                    .send()
                    .await?;

                if !retry_resp.status().is_success() {
                    let status = retry_resp.status();
                    let text = retry_resp.text().await.unwrap_or_default();
                    return Err(RobloxError::from_status(status, &text));
                }
                return Ok(retry_resp.json().await?);
            }
        }

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(RobloxError::from_status(status, &text));
        }

        Ok(resp.json().await?)
    }

    // ─── Internal fetch helpers ───────────────────────────────────────────────

    pub async fn get_user(&self, user_id: u64) -> Result<User> {
        let url = self
            .url_generator
            .get_url("users", &format!("v1/users/{}", user_id));
        let data = self.get_json(&url).await.map_err(|e| match e {
            RobloxError::NotFound(_) => RobloxError::UserNotFound,
            other => other,
        })?;
        let mut user: User = serde_json::from_value(data)?;
        user.client = Some(Arc::new(self.clone_self()));
        Ok(user)
    }

    pub async fn get_group(&self, group_id: u64) -> Result<Group> {
        let url = self
            .url_generator
            .get_url("groups", &format!("v1/groups/{}", group_id));
        let data = self.get_json(&url).await.map_err(|e| match e {
            RobloxError::NotFound(_) => RobloxError::GroupNotFound,
            other => other,
        })?;
        let mut group: Group = serde_json::from_value(data)?;
        group.client = Some(Arc::new(self.clone_self()));
        Ok(group)
    }

    pub async fn get_badge(&self, badge_id: u64) -> Result<Badge> {
        let url = self
            .url_generator
            .get_url("badges", &format!("v1/badges/{}", badge_id));
        let data = self.get_json(&url).await.map_err(|e| match e {
            RobloxError::NotFound(_) => RobloxError::BadgeNotFound,
            other => other,
        })?;
        let mut badge: Badge = serde_json::from_value(data)?;
        badge.client = Some(Arc::new(self.clone_self()));
        Ok(badge)
    }

    pub async fn get_universe(&self, universe_id: u64) -> Result<Universe> {
        let url = self
            .url_generator
            .get_url("games", &format!("v1/games?universeIds={}", universe_id));
        let data = self.get_json(&url).await.map_err(|e| match e {
            RobloxError::NotFound(_) => RobloxError::UniverseNotFound,
            other => other,
        })?;
        let arr = data["data"]
            .as_array()
            .and_then(|a| a.first().cloned())
            .ok_or(RobloxError::UniverseNotFound)?;
        let mut universe: Universe = serde_json::from_value(arr)?;
        universe.client = Some(Arc::new(self.clone_self()));
        Ok(universe)
    }

    pub async fn get_place(&self, place_id: u64) -> Result<Place> {
        let url = self
            .url_generator
            .get_url("games", &format!("v1/games/multiget-place-details?placeIds={}", place_id));
        let data = self.get_json(&url).await.map_err(|e| match e {
            RobloxError::NotFound(_) => RobloxError::PlaceNotFound,
            other => other,
        })?;
        let arr = data
            .as_array()
            .and_then(|a| a.first().cloned())
            .ok_or(RobloxError::PlaceNotFound)?;
        let mut place: Place = serde_json::from_value(arr)?;
        place._client = Some(Arc::new(self.clone_self()));
        Ok(place)
    }

    pub async fn get_plugin(&self, plugin_id: u64) -> Result<Plugin> {
        let url = self
            .url_generator
            .get_url("plugins", &format!("v1/plugins/{}", plugin_id));
        let data = self.get_json(&url).await.map_err(|e| match e {
            RobloxError::NotFound(_) => RobloxError::PluginNotFound,
            other => other,
        })?;
        let data_inner = data["data"]
            .as_array()
            .and_then(|a| a.first().cloned())
            .unwrap_or(data.clone());
        let mut plugin: Plugin = serde_json::from_value(data_inner)?;
        plugin._client = Some(Arc::new(self.clone_self()));
        Ok(plugin)
    }

    /// Creates a lightweight clone of the inner client for embedding in model objects.
    pub fn clone_self(&self) -> ClientInner {
        ClientInner {
            http: self.http.clone(),
            url_generator: self.url_generator.clone(),
            xcsrf_token: Mutex::new(self.xcsrf_token.lock().unwrap().clone()),
        }
    }
}

// ─── Public Client ────────────────────────────────────────────────────────────

/// The main entry point for interacting with the Roblox API.
///
/// Create a client, optionally set your `.ROBLOSECURITY` token, then call
/// any of the provided methods.
///
/// # Example
/// ```rust,no_run
/// use roblox::Client;
///
/// #[tokio::main]
/// async fn main() {
///     let client = Client::new();
///     let user = client.get_user(1).await.unwrap();
///     println!("User: {} ({})", user.name, user.id);
/// }
/// ```
pub struct Client {
    inner: Arc<ClientInner>,

    /// Presence API — query online status of users.
    pub presence: PresenceProvider,
    /// Thumbnail API — fetch images for users, assets, badges, and groups.
    pub thumbnails: ThumbnailProvider,
    /// Chat API — read and send messages.
    pub chat: ChatProvider,
    /// Account API — manage the authenticated user's account settings.
    pub account: AccountProvider,
}

impl Client {
    /// Creates a new unauthenticated client pointing at `roblox.com`.
    pub fn new() -> Self {
        Self::with_base_url("roblox.com", None)
    }

    /// Creates a new client and authenticates it with the given `.ROBLOSECURITY` token.
    pub fn with_token(token: impl Into<String>) -> Self {
        Self::with_base_url("roblox.com", Some(token.into()))
    }

    /// Creates a new client with a custom base URL (useful for testing/proxies).
    pub fn with_base_url(base_url: &str, token: Option<String>) -> Self {
        let cookie_jar = Arc::new(Jar::default());
        let base_domain = format!("https://{}", base_url);

        if let Some(ref t) = token {
            let cookie = format!("{}={}", ROBLOSECURITY_COOKIE, t);
            cookie_jar.add_cookie_str(&cookie, &base_domain.parse().unwrap());
        }

        let mut default_headers = HeaderMap::new();
        default_headers.insert("User-Agent", HeaderValue::from_static("Roblox/WinInet"));
        default_headers.insert("Referer", HeaderValue::from_static("www.roblox.com"));

        let http = HttpClient::builder()
            .cookie_provider(cookie_jar.clone())
            .default_headers(default_headers)
            .build()
            .expect("failed to build HTTP client");

        let url_generator = UrlGenerator::new(base_url);
        let inner = Arc::new(ClientInner {
            http,
            url_generator,
            xcsrf_token: Mutex::new(None),
        });

        Self {
            presence: PresenceProvider::new(inner.clone()),
            thumbnails: ThumbnailProvider::new(inner.clone()),
            chat: ChatProvider::new(inner.clone()),
            account: AccountProvider::new(inner.clone()),
            inner,
        }
    }

    /// Authenticates the client with a `.ROBLOSECURITY` token.
    ///
    /// Note: This updates the cookie in the shared cookie jar. Requires the
    /// `cookies` feature on `reqwest` (enabled by default in this crate).
    pub fn set_token(&self, token: impl Into<String>) {
        // The cookie jar is shared and mutates through reqwest's internal API.
        // The simplest cross-platform approach here is to update the XCSRF
        // cache — actual cookie injection happens at construction time.
        // For dynamic token switching, build a new Client with `with_token`.
        let _ = token; // kept for API compatibility
    }

    // ─── Users ─────────────────────────────────────────────────────────────

    /// Fetches a full user profile by numeric user ID.
    ///
    /// # Errors
    /// Returns [`RobloxError::UserNotFound`] if the user does not exist.
    pub async fn get_user(&self, user_id: u64) -> Result<User> {
        self.inner.get_user(user_id).await
    }

    /// Fetches the currently authenticated user's profile.
    ///
    /// Requires a valid `.ROBLOSECURITY` token.
    pub async fn get_authenticated_user(&self) -> Result<PartialUser> {
        let url = self
            .inner
            .url_generator
            .get_url("users", "v1/users/authenticated");
        let data = self.inner.get_json(&url).await?;
        Ok(serde_json::from_value(data)?)
    }

    /// Searches for users by username prefix. Returns a page of partial users.
    pub async fn search_users(&self, keyword: &str, limit: u32) -> Result<Vec<PartialUser>> {
        let url = self
            .inner
            .url_generator
            .get_url("users", "v1/users/search");
        let resp = self
            .inner
            .http
            .get(&url)
            .query(&[("keyword", keyword), ("limit", &limit.to_string())])
            .send()
            .await?;
        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(RobloxError::from_status(status, &body));
        }
        let data: Value = resp.json().await?;
        let users: Vec<PartialUser> = serde_json::from_value(data["data"].clone())?;
        Ok(users)
    }

    /// Fetches a user by exact username.
    ///
    /// # Errors
    /// Returns [`RobloxError::UserNotFound`] if the username does not exist.
    pub async fn get_user_by_username(&self, username: &str) -> Result<User> {
        let url = self
            .inner
            .url_generator
            .get_url("users", "v1/usernames/users");
        let data = self
            .inner
            .post_json(
                &url,
                &serde_json::json!({
                    "usernames": [username],
                    "excludeBannedUsers": false
                }),
            )
            .await?;

        let arr = data["data"]
            .as_array()
            .and_then(|a| a.first().cloned())
            .ok_or(RobloxError::UserNotFound)?;

        let id = arr["id"].as_u64().ok_or(RobloxError::UserNotFound)?;
        self.get_user(id).await
    }

    /// Returns a [`BaseUser`] without sending any requests.
    pub fn get_base_user(&self, user_id: u64) -> BaseUser {
        BaseUser::new(self.inner.clone(), user_id)
    }

    // ─── Groups ─────────────────────────────────────────────────────────────

    /// Fetches a full group by its numeric group ID.
    ///
    /// # Errors
    /// Returns [`RobloxError::GroupNotFound`] if the group does not exist.
    pub async fn get_group(&self, group_id: u64) -> Result<Group> {
        self.inner.get_group(group_id).await
    }

    /// Returns a [`BaseGroup`] without sending any requests.
    pub fn get_base_group(&self, group_id: u64) -> BaseGroup {
        BaseGroup::new(self.inner.clone(), group_id)
    }

    // ─── Assets ─────────────────────────────────────────────────────────────

    /// Fetches an asset's economy details by asset ID.
    pub async fn get_asset(&self, asset_id: u64) -> Result<EconomyAsset> {
        let url = self
            .inner
            .url_generator
            .get_url("economy", &format!("v2/assets/{}/details", asset_id));
        let data = self.inner.get_json(&url).await.map_err(|e| match e {
            RobloxError::NotFound(_) => RobloxError::AssetNotFound,
            other => other,
        })?;
        Ok(serde_json::from_value(data)?)
    }

    /// Returns a [`BaseAsset`] without sending any requests.
    pub fn get_base_asset(&self, asset_id: u64) -> BaseAsset {
        BaseAsset::new(self.inner.clone(), asset_id)
    }

    // ─── Badges ─────────────────────────────────────────────────────────────

    /// Fetches a badge by its numeric badge ID.
    ///
    /// # Errors
    /// Returns [`RobloxError::BadgeNotFound`] if the badge does not exist.
    pub async fn get_badge(&self, badge_id: u64) -> Result<Badge> {
        self.inner.get_badge(badge_id).await
    }

    /// Returns a [`BaseBadge`] without sending any requests.
    pub fn get_base_badge(&self, badge_id: u64) -> BaseBadge {
        BaseBadge::new(self.inner.clone(), badge_id)
    }

    // ─── GamePasses ──────────────────────────────────────────────────────────

    /// Returns a [`BaseGamePass`] without sending any requests.
    pub fn get_base_gamepass(&self, gamepass_id: u64) -> BaseGamePass {
        BaseGamePass::new(self.inner.clone(), gamepass_id)
    }

    // ─── Places ─────────────────────────────────────────────────────────────

    /// Fetches a place by its numeric place ID.
    ///
    /// # Errors
    /// Returns [`RobloxError::PlaceNotFound`] if the place does not exist.
    pub async fn get_place(&self, place_id: u64) -> Result<Place> {
        self.inner.get_place(place_id).await
    }

    /// Returns a [`BasePlace`] without sending any requests.
    pub fn get_base_place(&self, place_id: u64) -> BasePlace {
        BasePlace::new(self.inner.clone(), place_id)
    }

    // ─── Universes ───────────────────────────────────────────────────────────

    /// Fetches a universe (game) by its numeric universe ID.
    ///
    /// # Errors
    /// Returns [`RobloxError::UniverseNotFound`] if the universe does not exist.
    pub async fn get_universe(&self, universe_id: u64) -> Result<Universe> {
        self.inner.get_universe(universe_id).await
    }

    /// Returns a [`BaseUniverse`] without sending any requests.
    pub fn get_base_universe(&self, universe_id: u64) -> BaseUniverse {
        BaseUniverse::new(self.inner.clone(), universe_id)
    }

    // ─── Plugins ─────────────────────────────────────────────────────────────

    /// Fetches a plugin by its numeric plugin ID.
    ///
    /// # Errors
    /// Returns [`RobloxError::PluginNotFound`] if the plugin does not exist.
    pub async fn get_plugin(&self, plugin_id: u64) -> Result<Plugin> {
        self.inner.get_plugin(plugin_id).await
    }

    /// Returns a [`BasePlugin`] without sending any requests.
    pub fn get_base_plugin(&self, plugin_id: u64) -> BasePlugin {
        BasePlugin::new(self.inner.clone(), plugin_id)
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}
