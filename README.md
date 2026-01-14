# arcgis-sharing-rs

A Rust client library for the ArcGIS REST API, providing ergonomic access to feature layers, item management, and authentication.

## Features

- **Authentication** - OAuth2, app credentials, and token-based authentication
- **Feature Layers** - Query, update, and spatial operations
- **Item Management** - Create, update, publish, and manage ArcGIS items
- **Group Management** - Create and manage ArcGIS groups
- **Async/Await** - Built on tokio for efficient async operations

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
arcgis-api-rs = "0.1.0"
```

## Quick Start

```rust
use arcgis_api_rs::{config::get_config, auth::AuthType};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load configuration from environment
    let config = get_config()?;
    
    // Build an authenticated client
    let client = config
        .build_authorized_request_client(AuthType::AppAuth)
        .await?;
    
    // Use the client for API requests
    // ...
    
    Ok(())
}
```

## Configuration

Create a `.env` file with your ArcGIS credentials:

```env
APP_PORTAL_ROOT="https://your-portal.arcgis.com/sharing"
APP_CLIENT_ID="your_client_id"
APP_CLIENT_SECRET="your_client_secret"
APP_TOKEN_EXPIRATION="60"
```

## Status

⚠️ This library is in early development (v0.1.0). The API is subject to change.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
