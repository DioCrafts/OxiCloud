# Deployment & Docker

## Docker Image

OxiCloud uses a multi-stage Alpine build producing a **~40 MB** image:

1. **Base** — shared build dependencies (`musl-dev`, `pkgconfig`, `openssl-dev`, `libpq-dev`)
2. **Cacher** — pre-builds the dependency layer for fast rebuilds
3. **Builder** — compiles OxiCloud (`rust:1.94.0-alpine3.23`)
4. **Runtime** — minimal Alpine (`alpine:3.23.3`) with `libgcc`, `ca-certificates`, `libpq`, `tzdata`, `su-exec`

The final image runs as non-root user `oxicloud` (UID/GID 1001). Exposed port: **8086**.

## Docker Compose

```yaml
services:
  postgres:
    image: postgres:17.4-alpine
    environment:
      POSTGRES_DB: oxicloud
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: postgres       # change in production!
    volumes:
      - pg_data:/var/lib/postgresql/data
      - ./db/schema.sql:/docker-entrypoint-initdb.d/schema.sql
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U postgres"]
      interval: 5s
      timeout: 5s
      retries: 5

  oxicloud:
    image: ghcr.io/diocrafts/oxicloud:latest
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

## Kubernetes (Helm)

### Prerequisites
- Kubernetes cluster
- Default StorageClass
- Ingress Controller
- Helm 3+

### Install

```bash
helm upgrade --install oxicloud charts/oxicloud \
  -f charts/oxicloud/values.yaml
```

### Verify

```bash
kubectl get pods -n oxicloud
kubectl logs statefulset/oxicloud -n oxicloud
```

### WOPI Verification

If Collabora/OnlyOffice is enabled:

```bash
kubectl logs statefulset/oxicloud -n oxicloud | grep "WOPI discovery loaded"
```

## Feature Dependency Matrix

| Feature | Requires DB | Requires Auth | Feature Flag |
|---|---|---|---|
| File storage | Yes | No | Always on |
| Authentication | Yes | — | `OXICLOUD_ENABLE_AUTH` |
| OIDC / SSO | Yes | Yes | `OXICLOUD_OIDC_ENABLED` |
| File sharing | Yes | Yes | `OXICLOUD_ENABLE_FILE_SHARING` |
| Trash | Yes | No | `OXICLOUD_ENABLE_TRASH` |
| Search | Yes | No | `OXICLOUD_ENABLE_SEARCH` |
| Favorites | Yes | Yes | Always on |
| Storage quotas | Yes | Yes | `OXICLOUD_ENABLE_USER_STORAGE_QUOTAS` |
| WebDAV | Yes | Optional | Always on |
| CalDAV / CardDAV | Yes | Yes | Always on |
| Deduplication | No | No | Always on |
| Thumbnails | No | No | Always on |
| Chunked uploads | No | No | Always on |
