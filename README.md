<p align="center">
  <img src="static/oxicloud-logo.svg" alt="OxiCloud" width="375" />
</p>

<h3 align="center">Self-hosted cloud storage, calendar &amp; contacts — blazingly fast.</h3>

<div align="center">

  [![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)
  [![Latest Release](https://img.shields.io/github/release/diocrafts/OxiCloud.svg?style=for-the-badge)](https://github.com/diocrafts/OxiCloud/releases)
  [![CI](https://img.shields.io/github/actions/workflow/status/diocrafts/OxiCloud/ci.yml?branch=main&style=for-the-badge&label=CI)](https://github.com/diocrafts/OxiCloud/actions/workflows/ci.yml)
  [![Rust](https://img.shields.io/badge/Rust-1.93+-orange?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
  [![Docker Image Size](https://img.shields.io/docker/image-size/diocrafts/oxicloud?style=for-the-badge&logo=docker)](https://hub.docker.com/r/diocrafts/oxicloud)
  [![GitHub Stars](https://img.shields.io/github/stars/diocrafts/OxiCloud?style=for-the-badge&logo=github)](https://github.com/diocrafts/OxiCloud/stargazers)
  [![GitHub Issues](https://img.shields.io/github/issues/diocrafts/OxiCloud?style=for-the-badge)](https://github.com/diocrafts/OxiCloud/issues)
  [![Last Commit](https://img.shields.io/github/last-commit/diocrafts/OxiCloud?style=for-the-badge)](https://github.com/diocrafts/OxiCloud/commits/main)

</div>

<br/>

NextCloud was too slow on my home server. So I built OxiCloud — a complete cloud platform written in Rust that runs on minimal hardware and stays out of your way.

![OxiCloud Dashboard](doc/images/Captura%20de%20pantalla%202025-03-23%20230739.png)

<!-- TODO: add animated GIF/demo here -->

---

## OxiCloud vs NextCloud

| Metric | OxiCloud | NextCloud |
|--------|----------|-----------|
| **Language** | Rust (compiled, zero-cost abstractions) | PHP (interpreted) |
| **Docker image** | ~40 MB (Alpine, static binary) | ~1 GB+ (Apache + PHP + modules) |
| **Idle RAM** | ~30–50 MB | ~250–512 MB |
| **Cold start** | < 1 s | 5–15 s |
| **CPU at idle** | ~0 % | 1–5 % (cron, background jobs) |
| **Min. hardware** | 1 vCPU / 512 MB RAM | 2 vCPU / 2 GB RAM (recommended) |
| **Concurrent uploads** | Parallel chunked (TUS-like), async I/O | Sequential PHP workers |
| **File dedup** | SHA-256 content-addressable, ref-counting | None (each user = full copy) |
| **DB connections** | Dual pool (user + maintenance) | Single pool, background jobs compete |
| **LTO + PGO** | Fat LTO, codegen-units=1, opt-level=3 | N/A (interpreted) |
| **Dependencies** | Single binary + PostgreSQL | PHP, Apache/Nginx, Redis, Cron, …  |
| **WebDAV** | Built-in (RFC 4918, PROPFIND streaming) | Built-in |
| **CalDAV / CardDAV** | Built-in | Via apps |
| **WOPI (Office editing)** | Built-in (Collabora / OnlyOffice) | Via apps |
| **OIDC / SSO** | Built-in (Keycloak, Authentik, …) | Via apps |

> **Note:** NextCloud is a mature, feature-rich ecosystem. OxiCloud targets users who prioritise raw performance, simplicity, and low resource usage over plugin breadth.

---

## Features

### Storage & Files
- **Upload / download / organise** — drag-and-drop, multi-file, grid & list views
- **Chunked uploads** — TUS-like protocol, parallel chunks, resumable, MD5 integrity
- **File deduplication** — SHA-256 content-addressable blobs with automatic ref-counting
- **Adaptive compression** — zstd / gzip selected per MIME type
- **Trash bin** — soft-delete, restore, auto-purge by retention policy
- **Favourites & recent files**
- **Full-text search** — by name, type, date range, size, recursive subtree (ltree)
- **MIME magic-byte detection** — `infer` crate, not just extension guessing
- **Inline preview** — images, PDF, text, audio & video player modal
- **Thumbnails & transcoding** — WebP / AVIF on-the-fly via `image` crate

### Protocols
- **WebDAV** — RFC 4918, streaming PROPFIND, locking, compatible with all major clients
- **CalDAV** — calendar sync (Thunderbird, GNOME Calendar, iOS, DAVx⁵, …)
- **CardDAV** — contacts sync with vCard support
- **WOPI** — edit Office docs in Collabora Online or OnlyOffice
- **REST API** — complete JSON API for all operations

### Security & Auth
- **JWT authentication** with refresh tokens
- **Argon2id** password hashing
- **OIDC / SSO** — Keycloak, Authentik, Authelia, Google, Azure AD…
- **Role-based access** — admin / user, per-folder permissions
- **Storage quotas** per user
- **Shared links** with optional password protection

### Infrastructure
- **Single binary** — no runtime, no interpreter, no framework overhead
- **~40 MB Docker image** (Alpine)
- **Dual DB pool** — dedicated maintenance pool so background tasks never starve user queries
- **LTO-optimised release** — fat LTO, 1 codegen-unit, `opt-level = 3`, stripped
- **Write-behind caching** (moka) — sub-millisecond hot reads
- **112 automated tests** — `cargo test` on every push (CI)
- **9 languages** — EN, ES, DE, FR, IT, PT, NL, ZH, FA

---

## Quick Start

### Docker (recommended)

```bash
git clone https://github.com/DioCrafts/oxicloud.git
cd oxicloud

# Copy and optionally edit environment
cp example.env .env

docker compose up -d
```

Open **http://localhost:8086**. That's it.

### From source

Requires **Rust 1.93+** and **PostgreSQL 13+**.

```bash
git clone https://github.com/DioCrafts/oxicloud.git
cd oxicloud

# Configure database
echo "DATABASE_URL=postgres://user:pass@localhost/oxicloud" > .env

# Build optimised binary
cargo build --release

# Run migrations and start
cargo run --bin migrate --features migrations
cargo run --release
```

---

## Client Setup

OxiCloud speaks standard protocols — any WebDAV / CalDAV / CardDAV client works:

| Client | Protocol | URL |
|--------|----------|-----|
| Windows Explorer | WebDAV | `http://host:8086/webdav/` |
| macOS Finder | WebDAV | `http://host:8086/webdav/` |
| Nautilus / Dolphin | WebDAV | `dav://host:8086/webdav/` |
| Thunderbird (calendar) | CalDAV | `http://host:8086/caldav/` |
| Thunderbird (contacts) | CardDAV | `http://host:8086/carddav/` |
| DAVx⁵ (Android) | CalDAV + CardDAV | `http://host:8086/` |
| GNOME Calendar | CalDAV | `http://host:8086/caldav/` |
| GNOME Contacts | CardDAV | `http://host:8086/carddav/` |
| Collabora / OnlyOffice | WOPI | See [WOPI docs](doc/wopi-integration.md) |

For detailed setup guides: [WebDAV](doc/webdav-integration-guide.md) · [CalDAV](doc/caldav-technical-spec.md) · [CardDAV](doc/carddav-technical-spec.md) · [OIDC/SSO](doc/oidc-integration.md)

---

## Architecture

Clean / Hexagonal architecture — each layer depends only on the one below:

```
┌───────────────────────────────────────────────────────────────┐
│  Interfaces    │ REST API, WebDAV, CalDAV, CardDAV, WOPI     │
├───────────────────────────────────────────────────────────────┤
│  Application   │ Use cases, DTOs, port definitions           │
├───────────────────────────────────────────────────────────────┤
│  Domain        │ Entities, business rules, repository traits │
├───────────────────────────────────────────────────────────────┤
│  Infrastructure│ PostgreSQL, filesystem, caching, auth       │
└───────────────────────────────────────────────────────────────┘
```

Swap the database, add a new protocol, or change auth — without touching business logic.

For a deep dive: [Internal Architecture](doc/internal-architecture.md) · [Caching](doc/caching-architecture.md) · [DB Transactions](doc/database-transactions.md)

---

## Configuration

All config via environment variables (see [`example.env`](example.env)):

| Variable | Default | Description |
|----------|---------|-------------|
| `OXICLOUD_STORAGE_PATH` | `./storage` | Root directory for file storage |
| `OXICLOUD_SERVER_PORT` | `8086` | HTTP listen port |
| `OXICLOUD_DB_CONNECTION_STRING` | — | PostgreSQL connection string |
| `OXICLOUD_JWT_SECRET` | random | Token signing key (set in production!) |
| `OXICLOUD_OIDC_ENABLED` | `false` | Enable OpenID Connect SSO |
| `OXICLOUD_WOPI_ENABLED` | `false` | Enable Collabora / OnlyOffice editing |
| `OXICLOUD_ENABLE_AUTH` | `true` | Toggle authentication |
| `OXICLOUD_ENABLE_TRASH` | `true` | Toggle trash / recycle bin |

Full reference: [`example.env`](example.env) · [Deployment guide](doc/deployment.md) · [OIDC examples](doc/oidc-config-examples.md)

---

## Development

```bash
cargo build                 # Dev build
cargo run                   # Run locally
cargo test --workspace      # 112 tests
cargo clippy -- -D warnings # Lint (zero warnings policy)
cargo fmt --all --check     # Format check
RUST_LOG=debug cargo run    # Debug logging
```

### Project stats

| Metric | Value |
|--------|-------|
| Rust source files | 170 |
| Lines of code | ~50 000 |
| Automated tests | 112 |
| Documentation pages | 35 |

---

## Documentation

Extensive docs live in [`doc/`](doc/):

| Topic | Link |
|-------|------|
| Deployment & Docker | [deployment.md](doc/deployment.md) |
| WebDAV integration | [webdav-integration-guide.md](doc/webdav-integration-guide.md) |
| CalDAV / CardDAV | [caldav-technical-spec.md](doc/caldav-technical-spec.md) · [carddav-technical-spec.md](doc/carddav-technical-spec.md) |
| OIDC / SSO setup | [oidc-integration.md](doc/oidc-integration.md) · [oidc-config-examples.md](doc/oidc-config-examples.md) |
| WOPI (Office editing) | [wopi-integration.md](doc/wopi-integration.md) |
| Chunked uploads | [chunked-uploads.md](doc/chunked-uploads.md) |
| Deduplication | [deduplication.md](doc/deduplication.md) |
| Search | [search.md](doc/search.md) |
| Caching architecture | [caching-architecture.md](doc/caching-architecture.md) |
| Storage quotas | [storage-quotas.md](doc/storage-quotas.md) |
| Trash / recycle bin | [trash-feature-summary.md](doc/trash-feature-summary.md) |
| Internationalisation | [i18n.md](doc/i18n.md) |
| Internal architecture | [internal-architecture.md](doc/internal-architecture.md) |

---

## Roadmap

Check [TODO-LIST.md](TODO-LIST.md) for the full roadmap. Highlights:

- [ ] File versioning & diff viewer
- [ ] End-to-end encryption
- [ ] Desktop sync client (Rust)
- [ ] Android / iOS app
- [ ] OCR & intelligent tagging
- [ ] Automated workflows

---

## Contributing

The project is actively developed. Contributions welcome!

Read [CONTRIBUTING.md](CONTRIBUTING.md) before submitting a PR. Follow the [Code of Conduct](CODE_OF_CONDUCT.md).

## License

MIT — see [LICENSE](LICENSE).

---

## Star History

<div align="center">
  <a href="https://star-history.com/#DioCrafts/OxiCloud&Date">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/svg?repos=DioCrafts/OxiCloud&type=Date&theme=dark" />
      <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/svg?repos=DioCrafts/OxiCloud&type=Date" />
      <img alt="Star History Chart" src="https://api.star-history.com/svg?repos=DioCrafts/OxiCloud&type=Date" style="border-radius: 15px; box-shadow: 0 0 30px rgba(0, 217, 255, 0.3);" />
    </picture>
  </a>
</div>

---

Questions? [Open an issue](https://github.com/DioCrafts/OxiCloud/issues). Want to help? PRs welcome.
