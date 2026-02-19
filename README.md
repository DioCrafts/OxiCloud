<p align="center">
  <img src="static/oxicloud-logo.svg" alt="OxiCloud" width="375" />
</p>

<div align="center">
  
  [![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)
  [![Latest Release](https://img.shields.io/github/release/diocrafts/OxiCloud.svg?style=for-the-badge)](https://github.com/diocrafts/OxiCloud/releases)
  [![GitHub Stars](https://img.shields.io/github/stars/diocrafts/OxiCloud?style=for-the-badge&logo=github)](https://github.com/diocrafts/OxiCloud/stargazers)
  [![GitHub Issues](https://img.shields.io/github/issues/diocrafts/OxiCloud?style=for-the-badge)](https://github.com/diocrafts/OxiCloud/issues)
  [![GitHub Forks](https://img.shields.io/github/forks/diocrafts/OxiCloud?style=for-the-badge&logo=github)](https://github.com/diocrafts/OxiCloud/network/members)
  [![Last Commit](https://img.shields.io/github/last-commit/diocrafts/OxiCloud?style=for-the-badge)](https://github.com/diocrafts/OxiCloud/commits/main)

</div>

## A fast, simple alternative to NextCloud

NextCloud was too slow on my home server. So I built OxiCloud: a file storage system written in Rust that runs on minimal hardware and stays out of your way.

![OxiCloud Dashboard](doc/images/Captura%20de%20pantalla%202025-03-23%20230739.png)

## Why OxiCloud?

| Feature | What you get |
|---------|--------------|
| **Low resources** | Runs on 512MB RAM. No PHP, no bloat. |
| **Fast** | Rust with LTO optimization. Sub-second responses. |
| **Clean UI** | Works on desktop and mobile. No clutter. |
| **Easy setup** | One binary, one database, done. |
| **Multi-language** | English, Spanish and Persian out of the box. |

## Quick Start

You need Rust 1.70+, Cargo, and PostgreSQL 13+.

```bash
git clone https://github.com/DioCrafts/oxicloud.git
cd oxicloud

# Set up your database connection
echo "DATABASE_URL=postgres://username:password@localhost/oxicloud" > .env

# Build and run
cargo build --release
cargo run --bin migrate --features migrations
cargo run --release
```

Open `http://localhost:8085` in your browser.

### Docker (alternative)

```bash
docker compose up -d
```

That's it. The app runs on port 8086.

## Architecture

OxiCloud uses Clean Architecture with four layers:

```
┌─────────────────────────────────────────┐
│  Interfaces    │ API routes, handlers   │
├─────────────────────────────────────────┤
│  Application   │ Use cases, services    │
├─────────────────────────────────────────┤
│  Domain        │ Business logic         │
├─────────────────────────────────────────┤
│  Infrastructure│ Database, filesystem   │
└─────────────────────────────────────────┘
```

Each layer only talks to the one below it. You can swap out the database or add new API endpoints without touching business logic.

## Development

```bash
cargo build                 # Build
cargo run                   # Run locally
cargo test                  # Run tests
cargo clippy                # Lint
cargo fmt                   # Format

# For debugging
RUST_LOG=debug cargo run
```

## Frontend Asset Pipeline (No Node)

OxiCloud includes a Rust-native assets builder with `swc_ecma_minifier` for JavaScript.

```bash
# Build bundles/chunks + minified assets into static/dist (keeps HTML untouched)
cargo run --bin assets -- --no-rewrite-html

# Build bundles/chunks and rewrite HTML entries to hashed dist assets
cargo run --bin assets -- --rewrite-html
```

What it does:
- Minifies JS with SWC (`swc_ecma_minifier`)
- Bundles and splits into shared chunk + per-page chunks
- Minifies CSS (Rust pipeline)
- Emits content-hashed filenames and `static/dist/manifest.json`

## Current Features

- File upload, download, and organization
- Folder management with drag-and-drop
- Trash bin with restore functionality
- User authentication with JWT
- Personal folders per user
- File deduplication
- Write-behind cache for fast uploads
- Search across files and folders
- Favorites and recent files
- Responsive grid/list views

## What's Next

I'm working on these when I have time:

- File sharing via links
- WebDAV for desktop sync
- Basic versioning
- Mobile app improvements

Check [TODO-LIST.md](TODO-LIST.md) for the full list.

## Contributing

The project is early stage. There's plenty to improve.

Read [CONTRIBUTING.md](CONTRIBUTING.md) before submitting a PR. Follow the [Code of Conduct](CODE_OF_CONDUCT.md).

## License

MIT. See [LICENSE](LICENSE).

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

Questions? Open an issue. Want to help? PRs welcome.
