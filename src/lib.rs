/*!
# roblox

A modern, asynchronous Rust library for the Roblox web API — a port of [ro.py](https://github.com/ro-py/ro.py).

## Quick start

```rust,no_run
use roblox::Client;

#[tokio::main]
async fn main() {
    // Unauthenticated — read-only public API
    let client = Client::new();

    let user = client.get_user(1).await.unwrap();
    println!("{} ({})", user.name, user.id);

    let group = client.get_group(1200769).await.unwrap();
    println!("{} — {} members", group.name, group.member_count);
}
```

## Authentication

Pass a `.ROBLOSECURITY` cookie to authenticate:

```rust,no_run
use roblox::Client;

let client = Client::with_token("_|WARNING:-DO-NOT-SHARE-THIS...");
let me = client.get_authenticated_user().await.unwrap();
println!("Logged in as {:?}", me.name);
```

## Providers

Grouped functionality is available through sub-providers on the client:

| Provider              | What it does                              |
|-----------------------|-------------------------------------------|
| `client.presence`     | Query user presence / online status       |
| `client.thumbnails`   | Fetch avatar, asset, badge thumbnails     |
| `client.chat`         | Read and send chat messages               |
| `client.account`      | Manage the authenticated account          |

## Pagination

Endpoints that return many items use [`utilities::page::PageIterator`].
Call `.all().await` to collect every page eagerly, or drive the iterator
manually with `.next_page().await` for streaming behaviour.

```rust,no_run
use roblox::{Client, utilities::page::SortOrder};

#[tokio::main]
async fn main() {
    let client = Client::new();
    let user = client.get_user(1).await.unwrap();

    // Collect ALL followers (auto-follows cursors)
    let followers = user
        .get_followers(100, SortOrder::Ascending, None)
        .all()
        .await
        .unwrap();

    println!("{} followers", followers.len());
}
```

## Modules

| Module         | Contents                                      |
|----------------|-----------------------------------------------|
| `client`       | [`Client`] — the root object                  |
| `users`        | [`users::User`], [`users::FriendData`]        |
| `groups`       | [`groups::Group`], [`groups::GroupMember`]    |
| `assets`       | [`assets::EconomyAsset`]                      |
| `badges`       | [`badges::Badge`], [`badges::BadgeStatistics`]|
| `presence`     | [`presence::Presence`], [`presence::PresenceProvider`] |
| `thumbnails`   | [`thumbnails::ThumbnailProvider`]             |
| `universes`    | [`universes::Universe`]                       |
| `places`       | [`places::Place`]                             |
| `plugins`      | [`plugins::Plugin`]                           |
| `chat`         | [`chat::ChatProvider`]                        |
| `gamepasses`   | [`gamepasses::GamePass`]                      |
| `roles`        | [`roles::Role`]                               |
| `shout`        | [`shout::Shout`]                              |
| `social_links` | [`social_links::SocialLink`]                  |
| `account`      | [`account::AccountProvider`]                  |
| `partials`     | Lightweight partial objects                   |
| `bases`        | ID-only base objects with lazy `.expand()`    |
| `utilities`    | Pagination helpers                            |
| `error`        | [`error::RobloxError`]                        |
| `url`          | [`url::UrlGenerator`]                         |
*/

pub mod account;
pub mod assets;
pub mod badges;
pub mod bases;
pub mod chat;
pub mod client;
pub mod error;
pub mod gamepasses;
pub mod groups;
pub mod partials;
pub mod places;
pub mod plugins;
pub mod presence;
pub mod roles;
pub mod shout;
pub mod social_links;
pub mod thumbnails;
pub mod universes;
pub mod url;
pub mod users;
pub mod utilities;

// Top-level re-exports for ergonomic use
pub use client::Client;
pub use error::{Result, RobloxError};
