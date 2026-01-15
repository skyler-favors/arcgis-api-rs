# Contributing to arcgis-sharing-rs

## API Client Pattern Guide

This document explains the architectural patterns used in this codebase. Following these patterns ensures consistency and maintainability across all API endpoints.

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [The Handler Pattern](#the-handler-pattern)
3. [The Builder Pattern](#the-builder-pattern)
4. [Request Execution](#request-execution)
5. [Response Models](#response-models)
6. [File Organization](#file-organization)
7. [Complete Example](#complete-example)
8. [Common Mistakes to Avoid](#common-mistakes-to-avoid)

## Architecture Overview

The codebase follows a three-tier pattern:

```
Client → Handler → Builder → HTTP Request → Response
```

1. **Client** (`ArcGISSharingClient`) - Entry point, manages authentication and HTTP
2. **Handler** - Represents a resource or collection (e.g., feature service, group)
3. **Builder** - Constructs and executes specific operations on that resource
4. **Response Models** - Typed responses deserialized from JSON

## The Handler Pattern

Handlers represent resources in the ArcGIS API. They hold references to the client and any resource-specific state (like IDs or URLs).

### Handler Structure

```rust
pub struct FeatureServiceHandler<'a> {
    client: &'a ArcGISSharingClient,
    url: Url,
}

impl<'a> FeatureServiceHandler<'a> {
    pub(crate) fn new(client: &'a ArcGISSharingClient, url: impl Into<String>) -> Self {
        let url = Url::parse(&url.into()).context(UrlParseSnafu).unwrap();
        Self { client, url }
    }

    pub fn query(&self) -> FeatureServiceQueryBuilder<'_, '_> {
        FeatureServiceQueryBuilder::new(self)
    }

    pub fn apply_edits(&self) -> FeatureServiceApplyEditsBuilder<'_, '_> {
        FeatureServiceApplyEditsBuilder::new(self)
    }
}
```

### Handler Guidelines

- **Keep it simple** - Handlers should only create builders
- **Store resource identifiers** - URL, ID, or other identifiers
- **Use `pub(crate)` for constructors** - Only the client should create handlers
- **Return builders** - Each operation gets a builder method

### Registering Handlers in the Client

Add handler creation methods to `ArcGISSharingClient`:

```rust
impl ArcGISSharingClient {
    pub fn feature_service(&self, url: impl Into<String>) -> FeatureServiceHandler<'_> {
        FeatureServiceHandler::new(self, url.into())
    }

    pub fn groups(&self, id: impl Into<String>) -> GroupsHandler<'_> {
        GroupsHandler::new(self, id.into())
    }
}
```

## The Builder Pattern

Builders construct and execute API requests. They hold all parameters for a specific operation.

### Builder Structure

```rust
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FeatureServiceQueryBuilder<'a, 'r> {
    // ALWAYS skip the handler - it's not part of the API request
    #[serde(skip)]
    handler: &'r FeatureServiceHandler<'a>,

    // Optional fields use Option and skip_serializing_if
    #[serde(skip_serializing_if = "Option::is_none")]
    return_count_only: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    result_offset: Option<i32>,

    // Required fields can be plain types
    out_fields: String,

    // Vec fields skip serialization when empty
    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "Vec::is_empty"
    )]
    tags: Vec<String>,
}
```

### Builder Implementation

```rust
impl<'a, 'r> FeatureServiceQueryBuilder<'a, 'r> {
    pub fn new(handler: &'r FeatureServiceHandler<'a>) -> Self {
        Self {
            handler,
            // Initialize all fields with sensible defaults
            return_count_only: Some(false),
            result_offset: Some(0),
            out_fields: "*".to_string(),
            tags: Vec::new(),
        }
    }

    // Setter methods consume self and return Self for chaining
    pub fn set_count_only(mut self, count_only: bool) -> Self {
        self.return_count_only = Some(count_only);
        self
    }

    pub fn set_offset(mut self, offset: i32) -> Self {
        self.result_offset = Some(offset);
        self
    }

    pub fn set_tags(mut self, tags: Vec<impl Into<String>>) -> Self {
        self.tags = tags.into_iter().map(|t| t.into()).collect();
        self
    }

    // send() executes the request
    pub async fn send(&self) -> Result<FeatureServiceQueryResponse> {
        let url = format!("{}/query", self.handler.url);
        self.handler.client.get(url, Some(self)).await
    }
}
```

### Builder Guidelines

#### ✅ DO

- **Use direct references** - `handler: &'r Handler<'a>`, not `Option<&'r Handler<'a>>`
- **No `Default` derive** - Explicitly initialize all fields in `new()`
- **Use `#[serde(skip)]`** - Always skip the handler field
- **Chain-friendly setters** - Consume `self` and return `Self`
- **Sensible defaults** - Initialize fields in `new()` with appropriate defaults
- **Use `impl Into<T>`** - For string and convertible parameters
- **Skip serialization** - Use `skip_serializing_if` for `Option` and empty `Vec`

#### ❌ DON'T

- **Don't wrap handler in `Option`** - It's always required
- **Don't use `.as_ref().unwrap()`** - This indicates you're using `Option` unnecessarily
- **Don't create separate query structs** - The builder itself can execute the request
- **Don't use `..Default::default()`** - Explicitly list all fields for clarity
- **Don't add a `build()` method** - Just use `send()` directly

### Method Naming Conventions

- **Setters**: `set_*` (e.g., `set_count_only`, `set_where`)
- **Boolean flags**: `set_*` (e.g., `set_return_geometry`)
- **Collections**: `set_*` or plural noun (e.g., `set_tags`, `tags`)
- **Execution**: `send()` for all operations

## Request Execution

The `send()` method constructs the URL and executes the HTTP request.

### GET Requests

```rust
pub async fn send(&self) -> Result<FeatureServiceQueryResponse> {
    let url = format!("{}/query", self.handler.url);
    self.handler.client.get(url, Some(self)).await
}
```

### POST Requests

```rust
pub async fn send(&self) -> Result<CreateGroupResponse> {
    let url = self
        .handler
        .client
        .portal
        .join("sharing/rest/community/createGroup")
        .context(UrlParseSnafu)?;

    self.handler.client.post(url, Some(self), None).await
}
```

### URL Construction

- **Appending paths**: Use `format!("{}/path", base_url)` for simple cases
- **Complex URLs**: Use `Url::join()` when working with `portal` URLs
- **Query parameters**: Pass the builder as `Some(self)` - serde will serialize it

### Client Methods

The client provides these HTTP methods:

```rust
// GET request with query parameters
client.get(url, Some(params)).await

// POST request with form body
client.post(url, Some(body), None).await
```

Parameters are automatically:
- Serialized to query string (GET) or form data (POST)
- Filtered based on `skip_serializing_if` attributes
- Converted using serde field naming (camelCase)
- Appended with `f=json` (automatic)

## Response Models

Response models are defined in `src/models/` and use serde for deserialization.

### Response Structure

```rust
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FeatureServiceQueryResponse {
    // Use plain types for fields that are always present
    #[serde(default)]
    pub count: i32,

    // Use Option for fields that might be absent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<EsriFeature>>,

    // Capture unknown fields with flatten
    #[serde(flatten)]
    pub extra_fields: HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EsriFeature {
    pub attributes: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geometry: Option<serde_json::Value>,
}
```

### Response Guidelines

- **Match API structure** - Mirror the JSON structure from ArcGIS
- **Use `camelCase`** - Add `#[serde(rename_all = "camelCase")]` to match API
- **Use `Option` for optional fields** - Fields that might not exist
- **Use `#[serde(default)]`** - For fields that should have defaults when missing
- **Capture extras** - Use `#[serde(flatten)]` with `HashMap` for unknown fields
- **Public fields** - All response fields should be `pub`
- **Derive traits** - Always include `Serialize, Deserialize, Debug, Clone`

## File Organization

```
src/
├── api/
│   ├── mod.rs                           # Re-export all API modules
│   ├── community/
│   │   ├── mod.rs                       # Re-export community APIs
│   │   ├── create_group.rs              # Handler + Builder
│   │   └── groups/
│   │       ├── mod.rs                   # GroupsHandler
│   │       └── delete.rs                # DeleteGroupBuilder
│   └── feature_service/
│       ├── mod.rs                       # FeatureServiceHandler
│       ├── query.rs                     # QueryBuilder
│       └── apply_edits.rs               # ApplyEditsBuilder
├── models/
│   ├── mod.rs                           # Re-export all models
│   ├── group.rs                         # Group-related models
│   └── feature_service.rs               # Feature service models
├── client.rs                            # ArcGISSharingClient
├── error.rs                             # Error types
└── lib.rs                               # Public API

tests/
└── integration_tests.rs                 # Integration tests
```

### File Naming

- **Handler files**: Match the resource name (e.g., `create_group.rs`, `query.rs`)
- **One operation per file**: Each builder gets its own file
- **Group related operations**: Use subdirectories (e.g., `groups/delete.rs`)

### Module Structure

Each module should follow this pattern:

```rust
// src/api/feature_service/query.rs
use serde::Serialize;
use crate::api::FeatureServiceHandler;
use crate::{error::Result, models::*};

// Builder implementation here

// src/api/feature_service/mod.rs
mod query;
mod apply_edits;

pub use query::*;
pub use apply_edits::*;

// Handler definition
pub struct FeatureServiceHandler<'a> { /* ... */ }
```

## Complete Example

Here's a complete example implementing a new API endpoint:

### Step 1: Define Response Models

```rust
// src/models/item.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SearchItemsResponse {
    pub total: i32,
    pub start: i32,
    pub num: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<Item>>,
    #[serde(flatten)]
    pub extra_fields: HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    pub id: String,
    pub title: String,
    pub r#type: String,
    #[serde(flatten)]
    pub extra_fields: HashMap<String, serde_json::Value>,
}
```

### Step 2: Export Models

```rust
// src/models/mod.rs
pub mod auth;
pub mod feature_service;
pub mod group;
pub mod item;  // Add this line

pub use auth::*;
pub use feature_service::*;
pub use group::*;
pub use item::*;  // Add this line
```

### Step 3: Create Handler and Builder

```rust
// src/api/content/search.rs
use serde::Serialize;
use crate::api::ContentHandler;
use crate::{error::Result, models::*};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchItemsBuilder<'a, 'r> {
    #[serde(skip)]
    handler: &'r ContentHandler<'a>,

    #[serde(skip_serializing_if = "Option::is_none")]
    q: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    num: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sort_field: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sort_order: Option<String>,
}

impl<'a, 'r> SearchItemsBuilder<'a, 'r> {
    pub fn new(handler: &'r ContentHandler<'a>) -> Self {
        Self {
            handler,
            q: None,
            start: Some(1),
            num: Some(10),
            sort_field: None,
            sort_order: None,
        }
    }

    pub fn query(mut self, q: impl Into<String>) -> Self {
        self.q = Some(q.into());
        self
    }

    pub fn start(mut self, start: i32) -> Self {
        self.start = Some(start);
        self
    }

    pub fn num(mut self, num: i32) -> Self {
        self.num = Some(num);
        self
    }

    pub fn sort_by(mut self, field: impl Into<String>, order: impl Into<String>) -> Self {
        self.sort_field = Some(field.into());
        self.sort_order = Some(order.into());
        self
    }

    pub async fn send(&self) -> Result<SearchItemsResponse> {
        let url = format!("{}/sharing/rest/search", self.handler.client.portal);
        self.handler.client.get(url, Some(self)).await
    }
}
```

### Step 4: Create Handler Module

```rust
// src/api/content/mod.rs
mod search;

pub use search::*;

use crate::ArcGISSharingClient;

pub struct ContentHandler<'a> {
    client: &'a ArcGISSharingClient,
}

impl<'a> ContentHandler<'a> {
    pub(crate) fn new(client: &'a ArcGISSharingClient) -> Self {
        Self { client }
    }

    pub fn search(&self) -> SearchItemsBuilder<'_, '_> {
        SearchItemsBuilder::new(self)
    }
}
```

### Step 5: Register in Main API Module

```rust
// src/api/mod.rs
mod community;
mod content;  // Add this
mod feature_service;

pub use community::*;
pub use content::*;  // Add this
pub use feature_service::*;
```

### Step 6: Add Client Method

```rust
// src/client.rs (in the impl ArcGISSharingClient block)
pub fn content(&self) -> ContentHandler<'_> {
    ContentHandler::new(self)
}
```

### Step 7: Write Tests

```rust
// tests/integration_tests.rs
#[tokio::test]
async fn test_search_items() {
    Lazy::force(&SETUP);
    let client = arcgis_sharing_rs::instance();
    
    let response = client
        .content()
        .search()
        .query("type:Feature Service")
        .num(5)
        .send()
        .await
        .unwrap();
    
    assert!(response.total >= 0);
    assert_eq!(response.num, 5);
}
```

### Usage Example

```rust
// Using the new API
let client = ArcGISSharingClient::builder()
    .portal("https://my.arcgis.com".to_string())
    .legacy_auth(username, password, referer, expiration)
    .build();

let results = client
    .content()
    .search()
    .query("owner:myusername AND type:Web Map")
    .start(1)
    .num(25)
    .sort_by("title", "asc")
    .send()
    .await?;

println!("Found {} items", results.total);
```

## Common Mistakes to Avoid

### ❌ Mistake 1: Wrapping Handler in Option

```rust
// WRONG
#[serde(skip)]
handler: Option<&'r Handler<'a>>,

// In new():
handler: Some(handler),

// In send():
let handler = self.handler.as_ref().unwrap();
```

```rust
// CORRECT
#[serde(skip)]
handler: &'r Handler<'a>,

// In new():
handler,

// In send():
self.handler.client.get(...)
```

### ❌ Mistake 2: Using Default Derive

```rust
// WRONG
#[derive(Default, Serialize)]
pub struct MyBuilder<'a, 'r> {
    handler: Option<&'r Handler<'a>>,
    field1: String,
}

impl<'a, 'r> MyBuilder<'a, 'r> {
    pub fn new(handler: &'r Handler<'a>) -> Self {
        Self {
            handler: Some(handler),
            ..Default::default()  // Hides field initialization
        }
    }
}
```

```rust
// CORRECT
#[derive(Serialize)]
pub struct MyBuilder<'a, 'r> {
    handler: &'r Handler<'a>,
    field1: String,
}

impl<'a, 'r> MyBuilder<'a, 'r> {
    pub fn new(handler: &'r Handler<'a>) -> Self {
        Self {
            handler,
            field1: "default".to_string(),  // Explicit initialization
        }
    }
}
```

### ❌ Mistake 3: Creating Separate Query Structs

```rust
// WRONG - unnecessary complexity
pub struct Builder<'a, 'r> {
    handler: &'r Handler<'a>,
    field: String,
}

impl Builder {
    pub fn build(self) -> Query<'a, 'r> {
        Query { /* ... */ }
    }
}

pub struct Query<'a, 'r> {
    handler: &'r Handler<'a>,
    field: String,
}

impl Query {
    pub async fn send(&self) -> Result<Response> { /* ... */ }
}
```

```rust
// CORRECT - builder is the query
pub struct Builder<'a, 'r> {
    handler: &'r Handler<'a>,
    field: String,
}

impl Builder {
    pub async fn send(&self) -> Result<Response> {
        // Execute directly
    }
}
```

### ❌ Mistake 4: Not Skipping Handler Serialization

```rust
// WRONG - handler gets serialized to JSON
#[derive(Serialize)]
pub struct Builder<'a, 'r> {
    handler: &'r Handler<'a>,  // Missing #[serde(skip)]
    field: String,
}
```

```rust
// CORRECT
#[derive(Serialize)]
pub struct Builder<'a, 'r> {
    #[serde(skip)]
    handler: &'r Handler<'a>,
    field: String,
}
```

### ❌ Mistake 5: Using Url::join() Incorrectly

```rust
// WRONG - join() may replace the last segment
let url = handler.url.join("query").context(UrlParseSnafu)?;
// If handler.url = "http://host/service/FeatureServer/0"
// Result might be "http://host/service/FeatureServer/query" (wrong!)
```

```rust
// CORRECT - use format! for appending
let url = format!("{}/query", handler.url);
// Result: "http://host/service/FeatureServer/0/query" (correct!)
```

## Testing Your Implementation

Always add integration tests:

```rust
#[tokio::test]
async fn test_my_new_endpoint() {
    Lazy::force(&SETUP);
    let client = arcgis_sharing_rs::instance();
    
    let response = client
        .my_handler()
        .my_operation()
        .set_param("value")
        .send()
        .await
        .unwrap();
    
    assert!(response.is_valid());
}
```

Run tests with:
```bash
cargo test --test integration_tests test_my_new_endpoint
```

## Summary

The pattern is simple:

1. **Handler** - Represents a resource, creates builders
2. **Builder** - Holds parameters, has setters, executes request via `send()`
3. **Direct references** - No `Option` wrappers, no unnecessary abstractions
4. **Explicit initialization** - No `Default` derive, list all fields
5. **Clean execution** - Use `self.handler.client.get/post()` directly

Follow these patterns for consistency, and your code will integrate seamlessly with the existing codebase!
