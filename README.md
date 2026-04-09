# roblox-rs

A modern, asynchronous Rust library for the Roblox web API, a faithful port of [ro.py](https://github.com/ro-py/ro.py).

## Features

- **Async/await** via `tokio` and `reqwest`
- **Full authentication** via `.ROBLOSECURITY` cookie + automatic X-CSRF-TOKEN refresh
- **Users** — get by ID/username, search, friends, followers, followings
- **Groups** — get group, roles, members, update shout, member role lookup
- **Assets** — economy asset details, asset type names
- **Badges** — full badge info with statistics
- **Presence** — query online/in-game status for any user list
- **Thumbnails** — avatar, asset, badge, and group icon thumbnails
- **Universes** — game info, live stats, favorites, badges
- **Places** — place details
- **Plugins** — plugin details
- **Chat** — read conversations and messages, send messages
- **Account** — description, promotion channels, gender
- **Pagination** — cursor-based `PageIterator` with `.all()` or manual `.next_page()`
- **Base objects** — lightweight `BaseUser`, `BaseGroup`, etc. with lazy `.expand()`
- **Rich error types** — typed errors for each "not found" case

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
ro-rs = "0.1.1"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use roblox::Client;

#[tokio::main]
async fn main() {
    let client = Client::new();  // unauthenticated

    // Get a user by ID
    let user = client.get_user(1).await.unwrap();
    println!("User: {} ({})", user.name, user.id);

    // Get a group
    let group = client.get_group(1200769).await.unwrap();
    println!("Group: {} — {} members", group.name, group.member_count);

    // Get a badge
    let badge = client.get_badge(2124145009).await.unwrap();
    println!("Badge: {} (awarded {} times)", badge.name, badge.statistics.awarded_count);
}
```

## Authentication

```rust
use roblox::Client;

let client = Client::with_token("_|WARNING:-DO-NOT-SHARE-THIS...");

// Get authenticated user
let me = client.get_authenticated_user().await.unwrap();
println!("Logged in as {:?}", me.name);

// Account settings
let description = client.account.get_description().await.unwrap();
println!("My bio: {}", description);
```

## Presence

```rust
use roblox::Client;

let client = Client::new();
let presences = client.presence.get_user_presences(&[1, 156]).await.unwrap();
for p in presences {
    println!("User {:?}: {:?}", p.user_id, p.presence_type());
}
```

## Thumbnails

```rust
use roblox::{Client, thumbnails::{AvatarThumbnailType, ThumbnailFormat}};

let client = Client::new();
let thumbs = client.thumbnails
    .get_user_avatar_thumbnails(&[1], "420x420", ThumbnailFormat::Png, false, AvatarThumbnailType::FullBody)
    .await
    .unwrap();

println!("{:?}", thumbs[0].image_url);
```

## Pagination

All list endpoints return a `PageIterator`. Use `.all().await` to collect everything or `.next_page().await` for manual control.

```rust
use roblox::{Client, utilities::page::SortOrder};

let client = Client::new();
let user = client.get_user(1).await.unwrap();

// Collect up to 500 followers
let followers = user
    .get_followers(100, SortOrder::Ascending, Some(500))
    .all()
    .await
    .unwrap();

println!("{} followers fetched", followers.len());
```

## Group Operations

```rust
use roblox::{Client, utilities::page::SortOrder};

let client = Client::with_token("...");
let mut group = client.get_group(1200769).await.unwrap();

// Update the shout
let (old, new) = group.update_shout("Hello from roblox-rs!").await.unwrap();

// List roles
let roles = group.get_roles().await.unwrap();
for role in roles {
    println!("  Role: {} (rank {})", role.name, role.rank);
}

// Iterate members
let members = group
    .get_members(100, SortOrder::Ascending, Some(200))
    .all()
    .await
    .unwrap();

println!("{} members fetched", members.len());
```

## Architecture

This crate mirrors ro.py's structure:

| ro.py module       | Rust module           |
|--------------------|-----------------------|
| `client.py`        | `client`              |
| `users.py`         | `users`               |
| `groups.py`        | `groups`              |
| `assets.py`        | `assets`              |
| `badges.py`        | `badges`              |
| `presence.py`      | `presence`            |
| `thumbnails.py`    | `thumbnails`          |
| `universes.py`     | `universes`           |
| `places.py`        | `places`              |
| `plugins.py`       | `plugins`             |
| `chat.py`          | `chat`                |
| `gamepasses.py`    | `gamepasses`          |
| `roles.py`         | `roles`               |
| `shout.py`         | `shout`               |
| `social_links.py`  | `social_links`        |
| `account.py`       | `account`             |
| `partials/`        | `partials`            |
| `bases/`           | `bases`               |
| `utilities/iterators.py` | `utilities::page` |
| `utilities/url.py` | `url`                 |
| `utilities/exceptions.py` | `error`          |

## License

MIT — same as ro.py.
