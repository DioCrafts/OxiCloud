# Quick Start

## Docker (recommended)

```bash
git clone https://github.com/DioCrafts/OxiCloud.git
cd OxiCloud
cp example.env .env
docker compose up -d
```

Open **http://localhost:8086**. That's it.

### Docker Compose

```yaml
services:
  postgres:
    image: postgres:18.2-alpine3.23
    environment:
      POSTGRES_DB: oxicloud
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: postgres
    ports:
      - "5432:5432"
    volumes:
      - pg_data:/var/lib/postgresql/
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U postgres"]
      interval: 5s
      timeout: 5s
      retries: 5

  oxicloud:
    image: diocrafts/oxicloud:latest
    build:
      context: .
      dockerfile: Dockerfile
    ports:
      - "8086:8086"
    env_file:
      - .env
    volumes:
      - storage_data:/app/storage
    depends_on:
      postgres:
        condition: service_healthy

volumes:
  pg_data:
  storage_data:
```

## From Source

Requires **Rust 1.93+** and **PostgreSQL 13+**.

```bash
git clone https://github.com/DioCrafts/OxiCloud.git
cd OxiCloud
cp example.env .env

# Edit .env and set OXICLOUD_DB_CONNECTION_STRING for runtime
export DATABASE_URL=postgres://user:pass@localhost:5432/oxicloud

cargo build --release
cargo run --release
```

`OXICLOUD_DB_CONNECTION_STRING` is the runtime setting read by OxiCloud. `DATABASE_URL` is only needed for SQLx build-time checks.

## Kubernetes (Helm)

```bash
helm upgrade --install oxicloud charts/oxicloud \
  -f charts/oxicloud/values.yaml
```

Verify:

```bash
kubectl get pods -n oxicloud
kubectl logs statefulset/oxicloud -n oxicloud
```

## Client Setup

| Client | Protocol | URL |
|--------|----------|-----|
| Windows Explorer | WebDAV | `https://host/webdav/` |
| macOS Finder | WebDAV | `https://host/webdav/` |
| Nautilus / Dolphin | WebDAV | `davs://host/webdav/` |
| Thunderbird (calendar) | CalDAV | `https://host/caldav/` |
| Thunderbird (contacts) | CardDAV | `https://host/carddav/` |
| DAVx⁵ (Android) | CalDAV + CardDAV | `https://host/` |
| GNOME Calendar | CalDAV | `https://host/caldav/` |
| GNOME Contacts | CardDAV | `https://host/carddav/` |
| Collabora / OnlyOffice | WOPI | See [WOPI configuration](/config/wopi) |

## What's Next?

- [Environment Variables →](/config/env)
- [OIDC / SSO Setup →](/config/oidc)
- [WebDAV Guide →](/guide/webdav)
- [DAV Client Setup →](/guide/dav-client-setup)
