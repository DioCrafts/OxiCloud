# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

# Architecture

This project is split into two parts:
- `/src` — OxiCloud Backend server in **Rust**
- `/static` — Oxicloud Frontend in **vanilla CSS & vanilla JavaScript**

# Backend part

## Backend Build & Dev Commands

```bash
cargo build                          # Dev build
cargo build --release                # Optimized release build
cargo run                            # Run server (port 8086)
cargo test --workspace               # Run all tests (~208)
cargo test <test_name>               # Run a single test by name
cargo test --features test_utils     # Run tests that use mockall mocks
cargo clippy -- -D warnings          # Lint (zero warnings policy)
cargo fmt --all --check              # Format check
cargo fmt --all                      # Auto-format
RUST_LOG=debug cargo run             # Run with debug logging
cargo run --bin generate-openapi     # Regenerate resources/gen/openapi.json
```

A `justfile` is available for common tasks (`just --list` to see all). Key recipes: `just check` (fmt + clippy), `just test`, `just openapi`.

Requires **Rust 1.93+** (edition 2024) and **PostgreSQL 13+** (with `pg_trgm` and `ltree` extensions).

Database setup: `docker compose up -d postgres` — schema is applied automatically via sqlx migrations on app startup. Migration files live in `migrations/`. For local dev, set `DATABASE_URL` in `.env` (see `example.env`).

## Backend Pre-commit checks

Always run these before committing, in this order:

```bash
cargo fmt --all                                              # Auto-format
cargo clippy --all-features --all-targets -- -D warnings     # Lint (must pass with zero warnings)
```

CI enforces both — commits that fail either check will not merge.

## Backend Architecture

Hexagonal / Clean Architecture with four layers. Dependencies point inward only.

### Layer structure (`src/`)

- **`domain/`** — Core business entities (`entities/`) and repository trait definitions (`repositories/`). Pure Rust, no framework dependencies. Entity types: `File`, `Folder`, `User`, `Calendar`, `CalendarEvent`, `Contact`, `Share`, `TrashedItem`, `Session`, `DeviceCode`, `AppPassword`.

- **`application/`** — Use cases and orchestration.
  - `ports/` — Trait definitions (inbound/outbound) for storage, auth, caching, compression, dedup, thumbnails, chunked uploads, CalDAV/CardDAV, etc. This is the hexagonal "ports" layer.
  - `services/` — Use case implementations (`FileManagementService`, `FolderService`, `ShareService`, `TrashService`, `CalendarService`, `ContactService`, `SearchService`, `BatchOperations`, etc.).
  - `adapters/` — CalDAV/CardDAV protocol adapters (iCalendar/vCard parsing).
  - `dtos/` — Data transfer objects for API boundaries.

- **`infrastructure/`** — Concrete implementations of ports.
  - `repositories/pg/` — All PostgreSQL repository implementations (via `sqlx`). Uses `auth` schema for users/sessions, `storage` schema for files/folders/blobs (content-addressable dedup with ltree paths).
  - `services/` — JWT, password hashing (Argon2), OIDC, compression, thumbnails, chunked uploads, WOPI discovery, WebDAV locking, file content caching (moka).
  - `adapters/` — CalDAV/CardDAV storage adapters bridging domain traits to PG.
  - `db.rs` — Dual connection pool setup (user pool + maintenance pool).

- **`interfaces/`** — HTTP layer (Axum).
  - `api/handlers/` — REST API handlers for files, folders, auth, admin, search, shares, WebDAV, CalDAV, CardDAV, WOPI, chunked uploads, batch operations.
  - `api/routes.rs` — Route registration, splits protected vs public routes.
  - `nextcloud/` — NextCloud-compatible API (WebDAV, OCS, login flow v2, trashbin) with Basic Auth middleware.
  - `middleware/` — Auth (JWT validation), CSRF, rate limiting.
  - `web/` — Static file serving.

- **`common/`** — Cross-cutting concerns.
  - `di.rs` — `AppServiceFactory` builds all services and produces `AppState` (the central DI container passed to Axum). This is the composition root.
  - `config.rs` — `AppConfig::from_env()` loads all `OXICLOUD_*` env vars.

### Key patterns

- **DI via `AppState`**: All services are `Arc`-wrapped and assembled in `common/di.rs`. `AppState` is wrapped in `Arc` and passed as Axum state. Many services are `Option<Arc<T>>` because they depend on features being enabled (auth, WOPI, trash, etc.).

- **Content-addressable storage**: Files use SHA-256 blob dedup. `storage.file_blobs` stores content; `storage.file_metadata` references blobs with ref-counting. See `file_blob_write_repository.rs` and `file_blob_read_repository.rs`.

- **ltree paths**: Folder hierarchy uses PostgreSQL `ltree` for efficient subtree queries (recursive copies, moves, searches).

- **Dual DB pools**: `DbPools` in `infrastructure/db.rs` separates user-facing queries from maintenance/background tasks to prevent starvation.

- **Feature flags**: Major features (auth, trash, search, sharing, quotas) are toggled via `OXICLOUD_ENABLE_*` env vars in `FeaturesConfig`.

- **UUID columns**: All ID columns use native PostgreSQL `UUID` type. SQL queries must use `::uuid` casts when passing string parameters to UUID columns.

### Database schemas

- `auth` schema: `users`, `sessions`, `app_passwords`, `device_codes`, `admin_settings`
- `storage` schema: `folders`, `file_metadata`, `file_blobs`, `trash`, `shares`, `favorites`, `recent_items`, `nextcloud_object_ids`
- `caldav` schema: `calendars`, `calendar_events`
- `carddav` schema: `address_books`, `contacts`, `contact_groups`, `contact_group_members`

Schema definition: `migrations/` (sqlx migrations, applied on startup)

### Protocol support

The server exposes multiple protocol interfaces simultaneously:
- REST API under `/api/`
- WebDAV at `/webdav/` (RFC 4918)
- CalDAV at `/caldav/`
- CardDAV at `/carddav/`
- NextCloud-compatible API at `/remote.php/`, `/ocs/`, `/status.php`
- WOPI at `/wopi/` (when enabled)
- Well-known discovery at `/.well-known/caldav` and `/.well-known/carddav`

### Test organization

Tests are primarily `#[cfg(test)]` modules within source files (~36 files have inline tests). Dedicated test files exist at `*_test.rs` alongside their source. The `test_utils` feature flag enables `mockall` mock generation for trait-heavy testing. No separate `tests/` directory.

# Frontend part

## Code conventions

### Javascript

- ES Modules (import/export), no CommonJS
- No frameworks — vanilla JS only
- Naming: `camelCase` for variables/functions, `PascalCase` for classes
- No `var` — use `const`/`let` only
- **JSDoc required** on all public functions — `jsconfig.json` enables `checkJs` globally (equivalent to `@ts-check` on every file)
- Type parameters, return types, and complex types via `@typedef`:

```js
/**
 * @typedef {Object} User
 * @property {number} id
 * @property {string} name
 */

/**
 * @param {User} user
 * @param {string} [role="viewer"]
 * @returns {Promise}
 */
async function updateUser(user, role = 'viewer') { … }
```

### CSS
- BEM methodology for class names (`.block__element--modifier`)
- CSS custom properties in `:root` for colors and spacing
- **All colors must use `var(--*)` — no raw hex, rgb, or named colors anywhere except in `:root` declarations**
- Mobile-first: media queries expand, they don't restrict
- One CSS file per logical component in `/static/css/`
- [data-theme="dark"] is permitted only in /static/css/themes/dark.css

# What Claude must NOT do
- Edit `Cargo.lock` directly
- Use npm dependencies not listed in this file
- Introduce a JS framework (React, Vue, etc.) without explicit approval
- Leave debug `console.log` statements in code
- Use raw color values in CSS — always use CSS custom properties
- Commit without passing all linters
