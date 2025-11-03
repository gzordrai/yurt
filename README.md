# orda

Rust client library for the [aoe4guides.com](https://aoe4guides.com) API.

Orda provides a simple and type-safe interface for fetching build orders,
favorites, and other data from aoe4guides.com.

Internally, it wraps the public REST API using `reqwest` and `serde`.

## Features

- Optional civilization and sort filters
- Async-first API powered by `reqwest`
- Lightweight - no global runtime, compatible with any Tokio environment

## Install

Add **Orda** to your project using Cargo:

```bash
cargo add orda
```

Or manually in your `Cargo.toml`:

```toml
[dependencies]
orda = "0.1"
```

## Usage

```rust
use orda::{OrdaClient, Civilization, SortBy};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = OrdaClient::new();

    // Fetch API status
    let status = client.get_status().await?;

    println!("API status: {}", status.status);

    // Fetch the 10 most popular French builds
    let builds = client
        .get_builds(Civilization::Fre, Some(SortBy::Score), false)
        .await?;

    println!("Fetched {} builds", builds.len());

    // Fetch a single build by ID
    let build = client.get_build("00I7J47dv26cPbKmXYkO", false).await?;

    println!("Build title: {:?}", build.title);

    Ok(())
}

```

## Api overview

- `get_status()` => Returns the API status (always "running")
- `get_builds(civ, order_by, overlay)` => Fetches up to 10 build orders
- `get_build(id, overlay)` => Fetches a single build order by ID
- `get_favorites(user_id, civ, order_by, overlay)` => Fetches user favorites

## License

Licensed under the Apache 2.0 license.

See the [LICENSE](./LICENSE) file for details
