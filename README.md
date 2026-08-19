# arcgis-sharing-rs

A Rust client library for the ArcGIS REST API, providing ergonomic access to feature layers, item management, and authentication.

## Features

- **Authentication** - Legacy token authentication (OAuth planned)
- **Feature Layers** - Query, update, and spatial operations
- **Item Management** - Create, update, publish, and manage ArcGIS items
- **Group Management** - Create and manage ArcGIS groups
- **Async/Await** - Built on tokio for efficient async operations

## Requirements

Rust **1.80** or newer (`rust-version` in `Cargo.toml`).

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
arcgis-sharing-rs = "0.1.0"
```

Optional features:

- `default-client` (enabled by default) - global `initialize()` / `instance()` helpers
- `error-backtrace` - capture backtraces on errors (uses Snafu's backtrace support)

## Quick Start

```rust
use arcgis_sharing_rs::{ArcGISSharingClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = ArcGISSharingClient::builder()
        .portal("https://www.arcgis.com")
        .legacy_auth("username", "password", "127.0.0.1", "60")
        .build();

    let user = client.community_self().send().await?;
    println!("Signed in as {}", user.username);

    Ok(())
}
```

## Configuration

Set environment variables or pass values directly to the builder. Integration tests expect:

```env
APP_ARCGIS_PORTAL="https://your-portal.arcgis.com"
APP_ARCGIS_USERNAME="your_username"
APP_ARCGIS_PASSWORD="your_password"
```

## Status

This library is in early development (v0.1.0). The API is subject to change.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
