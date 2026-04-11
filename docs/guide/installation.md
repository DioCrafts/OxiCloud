# Quick Start

## Docker (recommended)

```bash
git clone https://github.com/DioCrafts/oxicloud.git
cd oxicloud
cp example.env .env
docker compose up -d
```

Open **http://localhost:8086**. That's it.

### Docker Compose

```yaml
services:
  postgres:
    image: postgres:17.4-alpine
    environment:
      POSTGRES_DB: oxicloud
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: postgres
    volumes:
      - pg_data:/var/lib/postgresql/data
      - ./db/schema.sql:/docker-entrypoint-initdb.d/schema.sql
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U postgres"]
      interval: 5s
      timeout: 5s
      retries: 5

  oxicloud:
    image: oxicloud:latest
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
git clone https://github.com/DioCrafts/oxicloud.git
cd oxicloud
echo "DATABASE_URL=postgres://user:pass@localhost/oxicloud" > .env
cargo build --release
cargo run --release
```

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
| Windows Explorer | WebDAV | `http://host:8086/webdav/` |
| macOS Finder | WebDAV | `http://host:8086/webdav/` |
| Nautilus / Dolphin | WebDAV | `dav://host:8086/webdav/` |
| Thunderbird (calendar) | CalDAV | `http://host:8086/caldav/` |
| Thunderbird (contacts) | CardDAV | `http://host:8086/carddav/` |
| DAVx⁵ (Android) | CalDAV + CardDAV | `http://host:8086/` |
| GNOME Calendar | CalDAV | `http://host:8086/caldav/` |
| GNOME Contacts | CardDAV | `http://host:8086/carddav/` |
| Collabora / OnlyOffice | WOPI | See [WOPI configuration](/config/wopi) |

## What's Next?

- [Environment Variables →](/config/env)
- [OIDC / SSO Setup →](/config/oidc)
- [WebDAV Guide →](/guide/webdav)
