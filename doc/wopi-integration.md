# WOPI Integration Technical Report for OxiCloud

## Table of Contents

1. [WOPI Protocol Overview](#1-wopi-protocol-overview)
2. [Required WOPI Endpoints](#2-required-wopi-endpoints)
3. [CheckFileInfo Response JSON Structure](#3-checkfileinfo-response-json-structure)
4. [WOPI Discovery XML](#4-wopi-discovery-xml)
5. [Collabora Online vs OnlyOffice WOPI Differences](#5-collabora-online-vs-onlyoffice-wopi-differences)
6. [OxiCloud Backend Changes](#6-oxicloud-backend-changes)
7. [OxiCloud Frontend Changes](#7-oxicloud-frontend-changes)
8. [Docker-Compose Additions](#8-docker-compose-additions)
9. [Implementation Roadmap](#9-implementation-roadmap)

---

## 1. WOPI Protocol Overview

WOPI (Web Application Open Platform Interface) is a REST-based protocol that enables cloud storage platforms (the **WOPI Host**) to expose files to external document editors (the **WOPI Client**) such as Collabora Online or OnlyOffice Document Server.

### Architecture Diagram

```
┌──────────────────────┐         WOPI REST API           ┌──────────────────────┐
│                      │◄──────────────────────────────► │                      │
│  OxiCloud (WOPI Host)│  CheckFileInfo, GetFile,        │  Collabora / Only-   │
│  Rust / Axum         │  PutFile, Lock, Unlock          │  Office (WOPI Client)│
│                      │                                 │                      │
│  /wopi/files/{id}    │                                 │  Document Editor     │
│  /wopi/files/{id}/   │                                 │  iframe UI           │
│    contents          │                                 │                      │
└──────────┬───────────┘                                 └──────────┬───────────┘
           │                                                        │
           │  Host Page (iframe)                                    │
           └────────────────────────────────────────────────────────┘
                      Browser loads host page with iframe
                      pointing to WOPI Client action URL
```

**Flow Summary:**

1. User clicks "Edit in Office" on a file in OxiCloud's frontend.
2. OxiCloud generates a **WOPI access token** and constructs a **host page URL**.
3. The host page contains an iframe whose `src` points to the WOPI Client's **action URL** (obtained from WOPI discovery), with the `WOPISrc` pointing back to OxiCloud's WOPI endpoints.
4. The WOPI Client (Collabora/OnlyOffice) calls OxiCloud's WOPI endpoints using the access token to:
   - Get file metadata (`CheckFileInfo`)
   - Download the file (`GetFile`)
   - Lock the file (`Lock`)
   - Save changes (`PutFile`)
   - Unlock the file (`Unlock`)

---

## 2. Required WOPI Endpoints

All WOPI host endpoints **must** be located at URLs starting with `/wopi`. The following endpoints are required for a functional editing integration:

### 2.1 Files Endpoint: `/wopi/files/{file_id}`

| Operation           | HTTP Method | URL                              | X-WOPI-Override  | Required For      |
|---------------------|-------------|----------------------------------|------------------|-------------------|
| **CheckFileInfo**   | `GET`       | `/wopi/files/{file_id}`          | —                | All actions       |
| **Lock**            | `POST`      | `/wopi/files/{file_id}`          | `LOCK`           | Edit              |
| **Unlock**          | `POST`      | `/wopi/files/{file_id}`          | `UNLOCK`         | Edit              |
| **RefreshLock**     | `POST`      | `/wopi/files/{file_id}`          | `REFRESH_LOCK`   | Edit              |
| **UnlockAndRelock** | `POST`      | `/wopi/files/{file_id}`          | `LOCK` (with `X-WOPI-OldLock`) | Edit |
| **PutRelativeFile** | `POST`      | `/wopi/files/{file_id}`          | `PUT_RELATIVE`   | Save As           |
| **RenameFile**      | `POST`      | `/wopi/files/{file_id}`          | `RENAME_FILE`    | Rename from editor|
| **DeleteFile**      | `POST`      | `/wopi/files/{file_id}`          | `DELETE`         | Delete from editor|

### 2.2 File Contents Endpoint: `/wopi/files/{file_id}/contents`

| Operation   | HTTP Method | URL                                    | X-WOPI-Override | Required For |
|-------------|-------------|----------------------------------------|-----------------|--------------|
| **GetFile** | `GET`       | `/wopi/files/{file_id}/contents`       | —               | All actions  |
| **PutFile** | `POST`      | `/wopi/files/{file_id}/contents`       | `PUT`           | Save/Edit    |

### 2.3 Detailed Endpoint Specifications

#### CheckFileInfo — `GET /wopi/files/{file_id}?access_token=TOKEN`

Returns JSON metadata about the file and user permissions. **Required for all WOPI actions.**

- **Query Params:** `access_token` (string)
- **Request Headers:** `X-WOPI-SessionContext` (optional)
- **Response:** `200 OK` with JSON body (see Section 3)
- **Error codes:** `401 Unauthorized`, `404 Not Found`, `500 Server Error`

#### GetFile — `GET /wopi/files/{file_id}/contents?access_token=TOKEN`

Returns the full binary contents of the file.

- **Query Params:** `access_token` (string)
- **Request Headers:** `X-WOPI-MaxExpectedSize` (optional integer)
- **Response:** `200 OK` with binary body
- **Response Headers:** `X-WOPI-ItemVersion` (optional)
- **Error codes:** `401`, `404`, `412 Precondition Failed` (file too large), `500`

#### PutFile — `POST /wopi/files/{file_id}/contents?access_token=TOKEN`

Saves updated file contents. The request body is the full binary content.

- **Request Headers:**
  - `X-WOPI-Override: PUT` (required)
  - `X-WOPI-Lock: <lock_id>` (required if file is locked)
  - `X-WOPI-Editors: <comma-separated user IDs>` (optional)
- **Response Headers:** `X-WOPI-ItemVersion`, `X-WOPI-Lock` (on 409)
- **Error codes:** `200 OK`, `401`, `404`, `409 Conflict` (lock mismatch), `413 Too Large`, `500`

**Special rule:** If the file is unlocked and its size is 0 bytes, PutFile should succeed (supports new file creation).

#### Lock — `POST /wopi/files/{file_id}?access_token=TOKEN`

Locks the file for editing.

- **Request Headers:**
  - `X-WOPI-Override: LOCK` (required)
  - `X-WOPI-Lock: <lock_id>` (required)
- **Behavior:**
  - If unlocked → lock file, return `200 OK`
  - If locked with same lock → refresh timer, return `200 OK`
  - Otherwise → `409 Conflict` with `X-WOPI-Lock` response header
- **Error codes:** `200`, `400 Bad Request`, `401`, `404`, `409`, `500`

#### Unlock — `POST /wopi/files/{file_id}?access_token=TOKEN`

- **Request Headers:**
  - `X-WOPI-Override: UNLOCK`
  - `X-WOPI-Lock: <lock_id>`
- **Behavior:** If lock matches → unlock, `200 OK`. Otherwise → `409 Conflict`.

#### RefreshLock — `POST /wopi/files/{file_id}?access_token=TOKEN`

- **Request Headers:**
  - `X-WOPI-Override: REFRESH_LOCK`
  - `X-WOPI-Lock: <lock_id>`
- **Behavior:** Resets the lock timer. Same conflict rules as Lock.

---

## 3. CheckFileInfo Response JSON Structure

```json
{
  // ─── REQUIRED ─────────────────────────────────────────
  "BaseFileName": "document.docx",
  "OwnerId": "user123",
  "Size": 45678,
  "UserId": "user456",
  "Version": "v1706123456",

  // ─── HOST CAPABILITIES ────────────────────────────────
  "SupportsLocks": true,
  "SupportsUpdate": true,
  "SupportsRename": true,
  "SupportsDeleteFile": false,
  "SupportsExtendedLockLength": true,
  "SupportsGetLock": true,

  // ─── USER PERMISSIONS ─────────────────────────────────
  "UserCanWrite": true,
  "UserCanRename": true,
  "UserCanNotWriteRelative": false,
  "ReadOnly": false,

  // ─── USER METADATA ────────────────────────────────────
  "UserFriendlyName": "John Doe",
  "IsAnonymousUser": false,

  // ─── FILE URLs ────────────────────────────────────────
  "CloseUrl": "https://cloud.example.com/files",
  "DownloadUrl": "https://cloud.example.com/api/files/abc123",
  "HostEditUrl": "https://cloud.example.com/wopi/edit/abc123",
  "HostViewUrl": "https://cloud.example.com/wopi/view/abc123",
  "FileSharingUrl": "https://cloud.example.com/share/abc123",
  "SignoutUrl": "https://cloud.example.com/logout",

  // ─── COLLABORA-SPECIFIC (optional) ───────────────────
  "PostMessageOrigin": "https://cloud.example.com",
  "HideSaveOption": false,
  "HidePrintOption": false,
  "DisablePrint": false,
  "DisableExport": false,
  "EnableOwnerTermination": true,
  "LastModifiedTime": "2026-02-13T10:00:00Z"
}
```

### Property Details

| Property | Type | Required | Description |
|----------|------|----------|-------------|
| `BaseFileName` | string | **Yes** | File name with extension, no path |
| `OwnerId` | string | **Yes** | Unique ID of the file owner (alphanumeric) |
| `Size` | long | **Yes** | File size in bytes |
| `UserId` | string | **Yes** | Unique ID of the current user (alphanumeric) |
| `Version` | string | **Yes** | Version string; must change on every file change |
| `SupportsLocks` | bool | No | Host supports Lock/Unlock/RefreshLock/UnlockAndRelock |
| `SupportsUpdate` | bool | No | Host supports PutFile and PutRelativeFile |
| `SupportsRename` | bool | No | Host supports RenameFile |
| `UserCanWrite` | bool | No | User has write permission |
| `UserFriendlyName` | string | No | Display name for the user |
| `PostMessageOrigin` | string | No | Origin for PostMessage communication |
| `LastModifiedTime` | string | No | ISO 8601 formatted last-modified time (Collabora uses this) |

---

## 4. WOPI Discovery XML

### 4.1 What is WOPI Discovery?

WOPI Discovery is the process by which OxiCloud (the WOPI host) learns the capabilities of the WOPI client (Collabora/OnlyOffice). The WOPI client exposes an XML document at a well-known URL that describes:

- **Supported file types** (extensions)
- **Available actions** (view, edit, editnew, etc.)
- **Action URLs** (the iframe URLs to load the editor)
- **Proof keys** (for request signing verification)

### 4.2 Discovery URLs

| WOPI Client | Discovery URL |
|-------------|---------------|
| **Collabora Online** | `https://<collabora-host>:<port>/hosting/discovery` |
| **OnlyOffice Document Server** | `https://<onlyoffice-host>/hosting/discovery` |

### 4.3 Discovery XML Structure

```xml
<?xml version="1.0" encoding="utf-8"?>
<wopi-discovery>
  <net-zone name="external-https">
    <app name="Word"
         favIconUrl="https://collabora.example.com/favicon.ico">
      <action name="view" ext="docx" default="true"
              urlsrc="https://collabora.example.com/cool/word/view?
                      WOPISrc=WOPI_SOURCE&amp;lang=UI_LLCC"/>
      <action name="edit" ext="docx" default="true"
              requires="locks,update"
              urlsrc="https://collabora.example.com/cool/word/edit?
                      WOPISrc=WOPI_SOURCE&amp;lang=UI_LLCC"/>
    </app>
    <app name="Excel"
         favIconUrl="https://collabora.example.com/favicon_calc.ico">
      <action name="view" ext="xlsx"
              urlsrc="https://collabora.example.com/cool/calc/view?
                      WOPISrc=WOPI_SOURCE"/>
      <action name="edit" ext="xlsx" requires="locks,update"
              urlsrc="https://collabora.example.com/cool/calc/edit?
                      WOPISrc=WOPI_SOURCE"/>
    </app>
    <!-- ... more apps for Impress, Draw, etc. -->
  </net-zone>
  <proof-key oldvalue="..." value="..."
             oldmodulus="..." modulus="..."
             oldexponent="..." exponent="..."/>
</wopi-discovery>
```

### 4.4 How OxiCloud Uses Discovery

1. **Fetch and cache** the discovery XML from the configured WOPI client URL (refresh every 12–24 hours or on proof key validation failure).
2. **Parse** the XML to build a map of `extension → { action_name → urlsrc }`.
3. When a user wants to edit/view a file:
   - Look up the file extension in the map.
   - Get the `urlsrc` for the desired action (e.g., `edit`).
   - **Transform** the URL: replace `WOPI_SOURCE` with OxiCloud's `WOPISrc` (the URL-encoded `CheckFileInfo` endpoint), and optionally replace `UI_LLCC` with the user's locale.
4. The transformed URL becomes the iframe `src` on the host page.

### 4.5 Action Requirements

| Requirement | WOPI Operations Needed |
|-------------|------------------------|
| `update`    | PutFile, PutRelativeFile |
| `locks`     | Lock, RefreshLock, Unlock, UnlockAndRelock |

---

## 5. Collabora Online vs OnlyOffice WOPI Differences

### 5.1 Integration Modes

| Aspect | Collabora Online (CODE) | OnlyOffice Document Server |
|--------|------------------------|---------------------------|
| **Primary API** | WOPI (native, first-class) | Document Server API (callback-based); WOPI support added later |
| **WOPI Support** | Full, native implementation | Supported since v7.2+, enabled via `wopi.enable: true` in config |
| **Discovery URL** | `/hosting/discovery` | `/hosting/discovery` (when WOPI enabled) |
| **Docker Image** | `collabora/code` | `onlyoffice/documentserver` |
| **License** | MPL 2.0 (CODE edition) | AGPL v3 (Community), commercial for >20 users |

### 5.2 WOPI Endpoint Differences

| Feature | Collabora Online | OnlyOffice (WOPI mode) |
|---------|-----------------|------------------------|
| **CheckFileInfo** | Standard WOPI spec + extra properties (`PostMessageOrigin`, `LastModifiedTime`, `HideSaveOption`, `HideExportOption`, `EnableOwnerTermination`) | Standard WOPI spec |
| **GetFile** | Standard | Standard |
| **PutFile** | Standard with `X-LOOL-WOPI-Timestamp` header (optional, for conflict detection) | Standard |
| **Lock/Unlock** | Standard (WOPI locks) | Standard (WOPI locks); also has internal lock via `refreshLockInterval` config |
| **Proof Keys** | Uses WOPI proof key validation from discovery XML | Has its own WOPI proof key system (`wopi.publicKey`, `wopi.modulus`, `wopi.exponent` in config) |
| **PutRelativeFile** | Supported | Supported |
| **RenameFile** | Supported via WOPI | Supported via WOPI |

### 5.3 Collabora-Specific Extras

- **PostMessageOrigin:** Collabora heavily uses PostMessage for UI integration (close, save status, etc.). Must be set in CheckFileInfo.
- **`X-LOOL-WOPI-Timestamp`:** Collabora sends the last-known modification timestamp in PutFile to detect conflicts. If the host's file is newer, it should return `409 Conflict` with `X-LOOL-WOPI-Timestamp` header.
- **COOL protocol:** Collabora's internal name is "COOL" (Collabora Online). URLs follow the pattern `/cool/<app>/edit`.

### 5.4 OnlyOffice-Specific Extras

- **Dual mode:** OnlyOffice can operate in either its native Document Server API mode (with callback URLs and JWT) or WOPI mode. When `wopi.enable = true`, it switches to WOPI and its `/hosting/discovery` endpoint becomes available.
- **File type mapping:** OnlyOffice has explicit configuration arrays for which extensions map to which editor: `wopi.wordEdit`, `wopi.cellEdit`, `wopi.slideEdit`, `wopi.pdfEdit`, etc.
- **JWT/Secret keys:** OnlyOffice uses `services.CoAuthoring.secret.inbox` for WOPI request validation alongside WOPI proof keys.
- **Browser token:** OnlyOffice has a separate `services.CoAuthoring.token.enable.browser` setting.

### 5.5 Recommendation

**Use Collabora Online (CODE)** as the primary editor for self-hosted deployments:
- Native WOPI support with no configuration mode switching.
- Better open-source license (MPL 2.0).
- More straightforward integration.
- Widely used by Nextcloud, ownCloud, Seafile, etc.

**Support OnlyOffice as secondary** for users who prefer its UI:
- The WOPI interface is the same, so a single WOPI host implementation supports both.
- Only docker-compose and frontend discovery URL config differ.

---

## 6. OxiCloud Backend Changes

### 6.1 New Configuration (`src/common/config.rs`)

Add a `WopiConfig` struct following the existing pattern:

```rust
/// WOPI (Web Application Open Platform Interface) configuration
#[derive(Debug, Clone)]
pub struct WopiConfig {
    /// Whether WOPI integration is enabled
    pub enabled: bool,
    /// URL to the WOPI client's discovery endpoint
    /// e.g., "http://collabora:9980/hosting/discovery"
    pub discovery_url: String,
    /// The public-facing base URL of OxiCloud (used for WOPISrc)
    /// e.g., "https://cloud.example.com"
    pub public_base_url: String,
    /// Secret key for signing WOPI access tokens (HMAC-SHA256)
    pub secret: String,
    /// Access token TTL in seconds (default: 86400 = 24 hours)
    pub token_ttl_seconds: u64,
    /// Lock expiration in seconds (default: 1800 = 30 minutes)
    pub lock_expiration_seconds: u64,
    /// Discovery XML cache TTL in seconds (default: 86400 = 24 hours)
    pub discovery_cache_ttl_seconds: u64,
}

impl Default for WopiConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            discovery_url: String::new(),
            public_base_url: String::new(),
            secret: String::new(),
            token_ttl_seconds: 86400,
            lock_expiration_seconds: 1800,
            discovery_cache_ttl_seconds: 86400,
        }
    }
}
```

**Environment variables** (following existing OxiCloud env pattern):

| Variable | Purpose | Example |
|----------|---------|---------|
| `WOPI_ENABLED` | Enable/disable WOPI | `true` |
| `WOPI_DISCOVERY_URL` | Discovery endpoint of the editor | `http://collabora:9980/hosting/discovery` |
| `WOPI_PUBLIC_BASE_URL` | Public URL of OxiCloud | `https://cloud.example.com` |
| `WOPI_SECRET` | HMAC secret for access tokens | `supersecretkey123` |
| `WOPI_TOKEN_TTL` | Token lifetime in seconds | `86400` |

### 6.2 New WOPI Handler (`src/interfaces/api/handlers/wopi_handler.rs`)

```rust
use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};

/// WOPI access token query parameter
#[derive(Deserialize)]
pub struct WopiTokenQuery {
    pub access_token: String,
}

/// CheckFileInfo response
#[derive(Serialize)]
pub struct CheckFileInfoResponse {
    #[serde(rename = "BaseFileName")]
    pub base_file_name: String,
    #[serde(rename = "OwnerId")]
    pub owner_id: String,
    #[serde(rename = "Size")]
    pub size: i64,
    #[serde(rename = "UserId")]
    pub user_id: String,
    #[serde(rename = "Version")]
    pub version: String,
    #[serde(rename = "SupportsLocks")]
    pub supports_locks: bool,
    #[serde(rename = "SupportsUpdate")]
    pub supports_update: bool,
    #[serde(rename = "SupportsRename")]
    pub supports_rename: bool,
    #[serde(rename = "UserCanWrite")]
    pub user_can_write: bool,
    #[serde(rename = "UserCanRename")]
    pub user_can_rename: bool,
    #[serde(rename = "UserFriendlyName")]
    pub user_friendly_name: String,
    #[serde(rename = "PostMessageOrigin")]
    pub post_message_origin: String,
    #[serde(rename = "CloseUrl")]
    pub close_url: String,
    #[serde(rename = "LastModifiedTime")]
    pub last_modified_time: String,
}

pub struct WopiHandler;

impl WopiHandler {
    /// GET /wopi/files/{file_id}?access_token=TOKEN
    pub async fn check_file_info(
        Path(file_id): Path<String>,
        Query(token): Query<WopiTokenQuery>,
        State(state): State<AppState>,
    ) -> impl IntoResponse {
        // 1. Validate access_token (HMAC verification)
        // 2. Extract user_id and file_id from token claims
        // 3. Fetch file metadata from FileRetrievalService
        // 4. Return CheckFileInfoResponse JSON
    }

    /// GET /wopi/files/{file_id}/contents?access_token=TOKEN
    pub async fn get_file(
        Path(file_id): Path<String>,
        Query(token): Query<WopiTokenQuery>,
        State(state): State<AppState>,
    ) -> impl IntoResponse {
        // 1. Validate access_token
        // 2. Stream file contents from storage
        // 3. Return binary body with Content-Type
    }

    /// POST /wopi/files/{file_id}/contents?access_token=TOKEN
    /// X-WOPI-Override: PUT
    pub async fn put_file(
        Path(file_id): Path<String>,
        Query(token): Query<WopiTokenQuery>,
        headers: HeaderMap,
        State(state): State<AppState>,
        body: axum::body::Bytes,
    ) -> impl IntoResponse {
        // 1. Validate access_token
        // 2. Check X-WOPI-Lock header against stored lock
        // 3. If locked with different lock → 409 Conflict
        // 4. If unlocked and file size > 0 → 409 Conflict
        // 5. Write file contents via FileManagementService
        // 6. Return 200 OK with X-WOPI-ItemVersion
    }

    /// POST /wopi/files/{file_id}?access_token=TOKEN
    /// Dispatches based on X-WOPI-Override header
    pub async fn file_operations(
        Path(file_id): Path<String>,
        Query(token): Query<WopiTokenQuery>,
        headers: HeaderMap,
        State(state): State<AppState>,
    ) -> impl IntoResponse {
        let override_header = headers
            .get("X-WOPI-Override")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        match override_header {
            "LOCK" => Self::handle_lock(file_id, headers, state).await,
            "UNLOCK" => Self::handle_unlock(file_id, headers, state).await,
            "REFRESH_LOCK" => Self::handle_refresh_lock(file_id, headers, state).await,
            "GET_LOCK" => Self::handle_get_lock(file_id, state).await,
            "RENAME_FILE" => Self::handle_rename(file_id, headers, state).await,
            "PUT_RELATIVE" => Self::handle_put_relative(file_id, headers, state).await,
            "DELETE" => Self::handle_delete(file_id, state).await,
            _ => (StatusCode::NOT_IMPLEMENTED, "Unknown override").into_response(),
        }
    }
}
```

### 6.3 WOPI Token Service (`src/application/services/wopi_token_service.rs`)

```rust
use hmac::{Hmac, Mac};
use sha2::Sha256;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct WopiTokenClaims {
    pub file_id: String,
    pub user_id: String,
    pub permissions: WopiPermissions,
    pub expires_at: u64,  // Unix timestamp
}

#[derive(Serialize, Deserialize)]
pub struct WopiPermissions {
    pub can_write: bool,
    pub can_rename: bool,
}

pub struct WopiTokenService {
    secret: Vec<u8>,
    ttl_seconds: u64,
}

impl WopiTokenService {
    /// Generate an access token for a file+user pair
    pub fn generate_token(&self, file_id: &str, user_id: &str, can_write: bool) -> (String, u64) {
        // Create claims, serialize to JSON, HMAC-sign, base64url encode
        // Return (token_string, ttl_in_milliseconds)
    }

    /// Validate and decode an access token
    pub fn validate_token(&self, token: &str) -> Result<WopiTokenClaims, WopiError> {
        // Decode, verify HMAC, check expiry
    }
}
```

### 6.4 WOPI Lock Service (`src/application/services/wopi_lock_service.rs`)

```rust
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::{Instant, Duration};

pub struct WopiLockEntry {
    pub lock_id: String,
    pub expires_at: Instant,
}

/// In-memory lock store (for single-instance deployments)
/// For multi-instance, replace with Redis or database-backed locks
pub struct WopiLockService {
    locks: Arc<RwLock<HashMap<String, WopiLockEntry>>>,
    lock_duration: Duration,
}

impl WopiLockService {
    pub async fn lock(&self, file_id: &str, lock_id: &str) -> Result<(), LockConflict>;
    pub async fn unlock(&self, file_id: &str, lock_id: &str) -> Result<(), LockConflict>;
    pub async fn refresh_lock(&self, file_id: &str, lock_id: &str) -> Result<(), LockConflict>;
    pub async fn get_lock(&self, file_id: &str) -> Option<String>;
    pub async fn is_locked(&self, file_id: &str) -> bool;
}
```

### 6.5 WOPI Discovery Service (`src/infrastructure/services/wopi_discovery_service.rs`)

```rust
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct WopiAction {
    pub name: String,       // "view", "edit", "editnew"
    pub ext: String,        // "docx", "xlsx", etc.
    pub urlsrc: String,     // Template URL
    pub requires: String,   // "locks,update"
}

/// Caches parsed WOPI discovery data
pub struct WopiDiscoveryService {
    discovery_url: String,
    /// Map: extension → Vec<WopiAction>
    actions: Arc<RwLock<HashMap<String, Vec<WopiAction>>>>,
    last_fetched: Arc<RwLock<Option<Instant>>>,
    cache_ttl: Duration,
}

impl WopiDiscoveryService {
    /// Fetch and parse the discovery XML from the WOPI client
    pub async fn refresh_discovery(&self) -> Result<(), Error>;

    /// Get the editor URL for a given file extension and action
    pub async fn get_action_url(
        &self,
        extension: &str,
        action: &str,  // "view" or "edit"
        wopi_src: &str, // URL-encoded WOPISrc
    ) -> Option<String>;

    /// Check if an extension is supported for editing
    pub async fn supports_edit(&self, extension: &str) -> bool;

    /// Check if an extension is supported for viewing
    pub async fn supports_view(&self, extension: &str) -> bool;

    /// Get list of all supported extensions
    pub async fn get_supported_extensions(&self) -> Vec<String>;
}
```

### 6.6 Routes Integration (`src/interfaces/api/routes.rs` and `src/main.rs`)

**New routes to add:**

```rust
// In routes.rs or a new wopi_routes function:
pub fn create_wopi_routes(app_state: &AppState) -> Router<AppState> {
    use crate::interfaces::api::handlers::wopi_handler::WopiHandler;

    Router::new()
        // CheckFileInfo
        .route("/files/{file_id}", get(WopiHandler::check_file_info))
        // Lock/Unlock/RefreshLock/Rename/Delete (dispatched by X-WOPI-Override)
        .route("/files/{file_id}", post(WopiHandler::file_operations))
        // GetFile
        .route("/files/{file_id}/contents", get(WopiHandler::get_file))
        // PutFile
        .route("/files/{file_id}/contents", post(WopiHandler::put_file))
        .with_state(app_state.clone())
}
```

**In `main.rs`, mount at `/wopi`** (outside of `/api` for WOPI spec compliance):

```rust
// In main.rs, after building other routers:
if config.wopi.enabled {
    let wopi_router = create_wopi_routes(&app_state);
    // WOPI routes are NOT behind auth middleware —
    // they use their own access_token validation
    app = app.nest("/wopi", wopi_router);
    tracing::info!("WOPI integration enabled");
}
```

### 6.7 API Endpoint for Frontend: WOPI Editor URL

Add a new API endpoint for the frontend to request an editor URL:

```rust
/// GET /api/wopi/editor-url?file_id=X&action=edit
/// Returns the URL to open the WOPI editor for a file
pub async fn get_editor_url(
    Query(params): Query<EditorUrlParams>,
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> impl IntoResponse {
    // 1. Generate WOPI access token for (file_id, user_id)
    // 2. Build WOPISrc = "{public_base_url}/wopi/files/{file_id}"
    // 3. Look up action URL from discovery for file extension + action
    // 4. Transform urlsrc: replace WOPI_SOURCE, UI_LLCC
    // 5. Return JSON:
    //    {
    //      "editor_url": "https://collabora:9980/cool/word/edit?WOPISrc=...",
    //      "access_token": "...",
    //      "access_token_ttl": 86400000,
    //      "favicon_url": "..."
    //    }
}
```

### 6.8 New Files Summary

| File | Purpose |
|------|---------|
| `src/interfaces/api/handlers/wopi_handler.rs` | WOPI HTTP handlers (CheckFileInfo, GetFile, PutFile, Lock, etc.) |
| `src/application/services/wopi_token_service.rs` | WOPI access token generation and validation |
| `src/application/services/wopi_lock_service.rs` | In-memory file lock management |
| `src/infrastructure/services/wopi_discovery_service.rs` | Discovery XML fetching, parsing, caching |
| `src/application/ports/wopi_ports.rs` | Port interfaces for WOPI services |
| `src/common/config.rs` | WopiConfig struct (added to AppConfig) |
| `src/common/di.rs` | Wire up WOPI services in AppState |
| `src/interfaces/api/routes.rs` | WOPI route definitions |

### 6.9 Dependencies to Add (`Cargo.toml`)

```toml
# WOPI support
hmac = "0.12"
sha2 = "0.10"
quick-xml = "0.36"     # XML parsing for WOPI discovery
urlencoding = "2.1"     # URL encoding for WOPISrc
base64 = "0.22"         # Token encoding
```

---

## 7. OxiCloud Frontend Changes

### 7.1 New JavaScript Module: `static/js/wopiEditor.js`

```javascript
/**
 * OxiCloud WOPI Editor Integration
 * Opens files in Collabora Online / OnlyOffice via WOPI protocol
 */
class WopiEditor {
  constructor() {
    this.supportedExtensions = null;
    this.editorModal = null;
  }

  /**
   * Check if a file can be opened in the WOPI editor
   */
  async canEdit(filename) {
    const ext = filename.split('.').pop().toLowerCase();
    const editableExtensions = [
      // Documents
      'docx', 'doc', 'odt', 'rtf', 'txt', 'dotx', 'docm',
      // Spreadsheets
      'xlsx', 'xls', 'ods', 'csv', 'xltx', 'xlsm',
      // Presentations
      'pptx', 'ppt', 'odp', 'ppsx', 'potx', 'pptm',
      // PDF
      'pdf'
    ];
    return editableExtensions.includes(ext);
  }

  /**
   * Open a file in the WOPI editor
   * @param {string} fileId - The file ID
   * @param {string} fileName - The file name
   * @param {string} action - "edit" or "view"
   */
  async openEditor(fileId, fileName, action = 'edit') {
    try {
      // 1. Request editor URL from OxiCloud API
      const response = await fetch(
        `/api/wopi/editor-url?file_id=${encodeURIComponent(fileId)}&action=${action}`,
        {
          headers: {
            'Authorization': `Bearer ${this.getAuthToken()}`
          }
        }
      );

      if (!response.ok) {
        throw new Error('Failed to get editor URL');
      }

      const data = await response.json();
      // data = { editor_url, access_token, access_token_ttl, favicon_url }

      // 2. Open the editor in a modal with iframe
      this.showEditorModal(data, fileName);

    } catch (error) {
      console.error('Failed to open WOPI editor:', error);
      alert('Could not open the document editor. Please try again.');
    }
  }

  /**
   * Create and show the editor modal with WOPI iframe
   */
  showEditorModal(editorData, fileName) {
    // Remove existing modal if any
    this.closeEditor();

    // Create modal overlay
    const modal = document.createElement('div');
    modal.id = 'wopi-editor-modal';
    modal.style.cssText = `
      position: fixed; top: 0; left: 0; width: 100%; height: 100%;
      z-index: 10000; background: #fff;
    `;

    // Create header bar
    const header = document.createElement('div');
    header.style.cssText = `
      height: 40px; background: #333; color: #fff; display: flex;
      align-items: center; justify-content: space-between; padding: 0 16px;
    `;
    header.innerHTML = `
      <span>${this.escapeHtml(fileName)}</span>
      <button id="wopi-close-btn" style="background:none;border:none;
        color:#fff;cursor:pointer;font-size:18px;">✕</button>
    `;

    // Create form + iframe (WOPI host page pattern)
    const frameholder = document.createElement('div');
    frameholder.style.cssText = `
      position: absolute; top: 40px; left: 0; right: 0; bottom: 0;
    `;

    // Form to POST access_token to the editor iframe
    const form = document.createElement('form');
    form.id = 'wopi_form';
    form.name = 'wopi_form';
    form.target = 'wopi_frame';
    form.action = editorData.editor_url;
    form.method = 'post';
    form.innerHTML = `
      <input name="access_token" value="${editorData.access_token}" type="hidden"/>
      <input name="access_token_ttl" value="${editorData.access_token_ttl}" type="hidden"/>
    `;

    // Create iframe dynamically (WOPI best practice)
    const iframe = document.createElement('iframe');
    iframe.name = 'wopi_frame';
    iframe.id = 'wopi_frame';
    iframe.title = 'Document Editor';
    iframe.style.cssText = 'width:100%;height:100%;border:none;';
    iframe.setAttribute('allowfullscreen', 'true');
    iframe.setAttribute('sandbox',
      'allow-scripts allow-same-origin allow-forms allow-popups ' +
      'allow-top-navigation allow-popups-to-escape-sandbox');
    iframe.setAttribute('allow',
      "clipboard-read 'src'; clipboard-write 'src'");

    frameholder.appendChild(iframe);

    modal.appendChild(header);
    modal.appendChild(form);
    modal.appendChild(frameholder);
    document.body.appendChild(modal);

    // Close button handler
    document.getElementById('wopi-close-btn').onclick = () => this.closeEditor();

    // ESC key to close
    this._escHandler = (e) => {
      if (e.key === 'Escape') this.closeEditor();
    };
    document.addEventListener('keydown', this._escHandler);

    // Submit the form to POST token to iframe
    form.submit();

    this.editorModal = modal;
  }

  closeEditor() {
    const modal = document.getElementById('wopi-editor-modal');
    if (modal) {
      modal.remove();
    }
    if (this._escHandler) {
      document.removeEventListener('keydown', this._escHandler);
    }
    this.editorModal = null;
    // Refresh file list to show any changes
    if (typeof loadFiles === 'function') {
      loadFiles();
    }
  }

  getAuthToken() {
    return localStorage.getItem('auth_token') || '';
  }

  escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
  }
}

// Global instance
window.wopiEditor = new WopiEditor();
```

### 7.2 Changes to Existing Frontend Files

#### `static/js/inlineViewer.js`

Add WOPI editor integration to the `openFile` method — when a user opens a document file (docx, xlsx, pptx, etc.), redirect to the WOPI editor instead of the inline viewer:

```javascript
// In openFile(file) method, add at the beginning:
if (window.wopiEditor && await window.wopiEditor.canEdit(file.name)) {
  window.wopiEditor.openEditor(file.id, file.name, 'edit');
  return;
}
```

#### `static/js/contextMenus.js`

Add "Edit in Office" context menu option for supported file types:

```javascript
// Add to file context menu items:
{
  label: 'Edit in Office',
  icon: 'fas fa-file-word',
  condition: (file) => window.wopiEditor?.canEdit(file.name),
  action: (file) => window.wopiEditor.openEditor(file.id, file.name, 'edit')
},
{
  label: 'View in Office',
  icon: 'fas fa-eye',
  condition: (file) => window.wopiEditor?.canEdit(file.name),
  action: (file) => window.wopiEditor.openEditor(file.id, file.name, 'view')
}
```

#### `static/index.html`

Add the WOPI editor script:

```html
<script src="/js/wopiEditor.js"></script>
```

### 7.3 PostMessage Integration (Optional Enhancement)

For deeper UI integration, listen for PostMessage events from the editor iframe:

```javascript
window.addEventListener('message', (event) => {
  // Verify origin matches the editor URL
  const data = JSON.parse(event.data);

  switch (data.MessageId) {
    case 'App_LoadingStatus':
      if (data.Values?.Status === 'Document_Loaded') {
        console.log('Document loaded in editor');
      }
      break;
    case 'UI_Close':
      window.wopiEditor.closeEditor();
      break;
    case 'UI_FileVersions':
      // Show version history UI
      break;
    case 'UI_Sharing':
      // Show sharing UI
      break;
  }
});
```

---

## 8. Docker-Compose Additions

### 8.1 Collabora Online (CODE)

```yaml
services:
  # ... existing postgres and oxicloud services ...

  collabora:
    image: collabora/code:latest
    restart: always
    cap_add:
      - MKNOD
    environment:
      # Allow OxiCloud domain to use Collabora
      - "aliasgroup1=http://oxicloud:8086"
      # Alternative: use domain= for regex pattern
      # - "domain=oxicloud\\.example\\.com"
      # Disable SSL (handled by reverse proxy)
      - "extra_params=--o:ssl.enable=false --o:ssl.termination=true"
      # Admin console credentials
      - "username=admin"
      - "password=admin_password"
      # Server name for discovery
      - "server_name=collabora.example.com"
    ports:
      - "9980:9980"
    networks:
      - oxicloud
    depends_on:
      - oxicloud

  # Update oxicloud service with WOPI env vars:
  oxicloud:
    # ... existing config ...
    environment:
      - "OXICLOUD_DB_CONNECTION_STRING=postgres://postgres:postgres@postgres/oxicloud"
      - "DATABASE_URL=postgres://postgres:postgres@postgres/oxicloud"
      # WOPI configuration
      - "WOPI_ENABLED=true"
      - "WOPI_DISCOVERY_URL=http://collabora:9980/hosting/discovery"
      - "WOPI_PUBLIC_BASE_URL=http://localhost:8086"
      - "WOPI_SECRET=change-me-to-a-random-secret-key"
```

### 8.2 OnlyOffice Document Server (alternative)

```yaml
services:
  # ... existing services ...

  onlyoffice:
    image: onlyoffice/documentserver:latest
    restart: always
    environment:
      # Enable WOPI mode
      - "WOPI_ENABLED=true"
      # JWT secret (must match OxiCloud's WOPI secret)
      - "JWT_SECRET=change-me-to-a-random-secret-key"
      - "JWT_ENABLED=true"
    ports:
      - "8088:80"
    networks:
      - oxicloud
    volumes:
      - onlyoffice_data:/var/www/onlyoffice/Data
      - onlyoffice_logs:/var/log/onlyoffice
    depends_on:
      - oxicloud

volumes:
  # ... existing volumes ...
  onlyoffice_data:
  onlyoffice_logs:
```

### 8.3 Full Docker-Compose with Both Editors

```yaml
services:
  postgres:
    image: postgres:17.4-alpine
    restart: always
    environment:
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: postgres
      POSTGRES_DB: oxicloud
    networks:
      - oxicloud
    volumes:
      - pg_data:/var/lib/postgresql/data
      - ./db/schema.sql:/docker-entrypoint-initdb.d/10-schema.sql
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U postgres"]
      interval: 5s
      timeout: 5s
      retries: 5

  oxicloud:
    image: oxicloud
    restart: always
    build:
      context: .
      dockerfile: Dockerfile
    ports:
      - "8086:8086"
    networks:
      - oxicloud
    depends_on:
      postgres:
        condition: service_healthy
    environment:
      - "OXICLOUD_DB_CONNECTION_STRING=postgres://postgres:postgres@postgres/oxicloud"
      - "DATABASE_URL=postgres://postgres:postgres@postgres/oxicloud"
      # WOPI configuration
      - "WOPI_ENABLED=true"
      - "WOPI_DISCOVERY_URL=http://collabora:9980/hosting/discovery"
      - "WOPI_PUBLIC_BASE_URL=http://localhost:8086"
      - "WOPI_SECRET=change-me-to-a-random-secret-key"
    volumes:
      - storage_data:/app/storage

  collabora:
    image: collabora/code:latest
    restart: always
    cap_add:
      - MKNOD
    environment:
      - "aliasgroup1=http://oxicloud:8086"
      - "extra_params=--o:ssl.enable=false --o:ssl.termination=true"
      - "username=admin"
      - "password=admin_password"
    ports:
      - "9980:9980"
    networks:
      - oxicloud

  # Optional: OnlyOffice alternative
  # onlyoffice:
  #   image: onlyoffice/documentserver:latest
  #   restart: always
  #   environment:
  #     - "WOPI_ENABLED=true"
  #     - "JWT_SECRET=change-me-to-a-random-secret-key"
  #     - "JWT_ENABLED=true"
  #   ports:
  #     - "8088:80"
  #   networks:
  #     - oxicloud
  #   volumes:
  #     - onlyoffice_data:/var/www/onlyoffice/Data

networks:
  oxicloud:
    driver: bridge

volumes:
  pg_data:
  storage_data:
  # onlyoffice_data:
```

### 8.4 Reverse Proxy Considerations

In production, a reverse proxy (Nginx/Traefik) is typically needed:

```nginx
# Collabora Online proxy
location ^~ /cool/ {
    proxy_pass http://collabora:9980;
    proxy_set_header Host $http_host;
    proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    proxy_set_header X-Forwarded-Proto $scheme;

    # WebSocket support
    proxy_http_version 1.1;
    proxy_set_header Upgrade $http_upgrade;
    proxy_set_header Connection "Upgrade";
    proxy_read_timeout 36000s;
}

location ^~ /hosting/ {
    proxy_pass http://collabora:9980;
    proxy_set_header Host $http_host;
}

# WOPI endpoints (OxiCloud)
location ^~ /wopi/ {
    proxy_pass http://oxicloud:8086;
    proxy_set_header Host $http_host;
    proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    proxy_set_header X-Forwarded-Proto $scheme;
}
```

---

## 9. Implementation Roadmap

### Phase 1: Core WOPI Host (Backend)
1. Add `WopiConfig` to `AppConfig` and environment variable parsing
2. Implement `WopiTokenService` (token generation/validation)
3. Implement `WopiLockService` (in-memory lock store)
4. Implement `WopiDiscoveryService` (discovery XML parsing)
5. Implement `WopiHandler` with `CheckFileInfo`, `GetFile`, `PutFile`
6. Implement lock operations (`Lock`, `Unlock`, `RefreshLock`)
7. Wire up in `di.rs` and mount routes in `main.rs`

### Phase 2: Frontend Integration
1. Create `wopiEditor.js` module
2. Add "Edit in Office" to context menu and file viewer
3. Implement WOPI host page (iframe + form POST)
4. Add PostMessage event handling

### Phase 3: Docker & Deployment
1. Add Collabora CODE to docker-compose
2. Configure networking and environment variables
3. Test end-to-end flow
4. Add OnlyOffice as alternative option

### Phase 4: Enhancements
1. Proof key validation (verify requests come from legitimate WOPI client)
2. `PutRelativeFile` ("Save As" support)
3. `RenameFile` integration
4. Multi-instance lock storage (Redis/DB-backed)
5. Co-editing status indicators in the file list
6. File version tracking for WOPI saves

---

## Key Architectural Notes for OxiCloud

1. **WOPI routes go at `/wopi/` (top-level)**, following the same pattern as WebDAV/CalDAV/CardDAV which are merged at top-level in `main.rs` for protocol compliance.

2. **WOPI uses its own authentication** (access tokens), not OxiCloud's JWT auth middleware. The WOPI routes should NOT be wrapped in `auth_middleware`. Instead, each WOPI handler validates the `access_token` query parameter internally.

3. **The existing `FileRetrievalService` and `FileManagementService`** in `AppState` already provide all the file operations needed (download, upload, rename, delete). WOPI handlers are thin adapters that call these existing services.

4. **File IDs** in OxiCloud are already URL-safe strings (UUID-style), which satisfies the WOPI file ID requirement.

5. **The `InlineViewer`** already has a modal pattern that can be extended. The WOPI editor uses a similar full-screen modal approach but with an iframe instead of direct content rendering.
