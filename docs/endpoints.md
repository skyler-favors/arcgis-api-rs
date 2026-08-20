# Implemented Endpoints

A complete inventory of ArcGIS REST API endpoints currently implemented in `arcgis-sharing-rs`.

All paths are relative to the portal URL configured on the client (e.g. `https://www.arcgis.com`).

## Community

| Client access | REST endpoint | Method | Response type |
|---|---|---|---|
| `client.community_self().send()` | `/sharing/rest/community/self` | GET | `UserSelfResponse` |
| `client.create_group().send()` | `/sharing/rest/community/createGroup` | POST | `CreateGroupResponse` |
| `client.search_groups().send()` | `/sharing/rest/community/groups` | GET | `GroupSearchStream` (paginated) |
| `client.groups(id).delete().send()` | `/sharing/rest/community/groups/{id}/delete` | POST | `DeleteGroupsResponse` |

## Portals

| Client access | REST endpoint | Method | Response type |
|---|---|---|---|
| `client.portals().self_info().send()` | `/sharing/rest/portals/self` | GET | `PortalsSelfResponse` |

## Search

| Client access | REST endpoint | Method | Response type |
|---|---|---|---|
| `client.search().send()` | `/sharing/rest/search` | GET | `SearchStream` (paginated) |

## Content (user-scoped)

| Client access | REST endpoint | Method | Response type |
|---|---|---|---|
| `client.content(username).add_item().send()` | `/sharing/rest/content/users/{username}/addItem` | POST | `AddItemResponse` |
| `client.content(username).analyze().send()` | `/sharing/rest/content/features/analyze` | POST | `AnalyzeResponse` |
| `client.content(username).create_service(parameters).send()` | `/sharing/rest/content/users/{username}/createService` | POST | `CreateServiceResponse` |

## Items

| Client access | REST endpoint | Method | Response type |
|---|---|---|---|
| `client.item(id).info()` | `/sharing/rest/content/items/{id}` | GET | `Item` |
| `client.item(id).data().send()` | `/sharing/rest/content/items/{id}/data` | GET | `T: FromResponse` |
| `client.item(id).update().send()` | `/sharing/rest/content/users/{owner}/items/{id}/update` | POST | `UpdateItemResponse` |
| `client.item(id).delete().send()` | `/sharing/rest/content/users/{owner}/items/{id}/delete` | POST | `DeleteItemResponse` |
| `client.item(id).publish().send()` | `/sharing/rest/content/users/{owner}/publish` | POST | `PublishItemResponse` |
| `client.item(id).resources().send()` | `/sharing/rest/content/items/{id}/resources` | GET | `ListResourcesResponse` |
| `client.item(id).add_resources().send()` | `/sharing/rest/content/users/{owner}/items/{id}/addResources` | POST | `AddResourcesResponse` |
| `client.item(id).update_resources().send()` | `/sharing/rest/content/users/{owner}/items/{id}/updateResources` | POST | `AddResourcesResponse` |
| `client.item(id).get_resource().send(filename)` | `/sharing/rest/content/items/{id}/resources/{filename}` | GET | `String` (raw text) |

## Feature Service

| Client access | REST endpoint | Method | Response type |
|---|---|---|---|
| `client.feature_service(url).query().send()` | `{url}/query` | GET | `FeatureServiceQueryResponse` |
| `client.feature_service(url).apply_edits().send()` | `{url}/applyEdits` | POST | `ApplyEditsResponse` |

## Auth (internal)

These endpoints live in `src/auth.rs` and are used during client setup, not via the handler/builder pattern:

| Endpoint | Method | Purpose |
|---|---|---|
| `/sharing/rest/generateToken` | POST | Legacy username/password token |
| OAuth `token_url` (configurable) | POST | Authorization code exchange (PKCE) |
| OAuth `token_url` (configurable) | POST | Token refresh |

## Summary

**20 user-facing API operations** across 6 areas:

- **Community**: 4
- **Portals**: 1
- **Search**: 1
- **Content**: 2
- **Items**: 9
- **Feature Service**: 2

Plus **3 auth endpoints** used internally during client initialization.

## Notes

- `search()` and `search_groups()` return streams with automatic pagination rather than a single response.
- Mutating item operations (`update`, `delete`, `publish`, `add_resources`, `update_resources`) resolve the item owner via `GET /content/items/{id}` on first use.
- `add_item`, `update`, `analyze`, `add_resources`, `update_resources`, and `publish` support multipart uploads when files are attached.
