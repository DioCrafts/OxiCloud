# 02 - Deployment

OxiCloud is deployed as a containerized application with PostgreSQL.

---

## Docker Setup

### Docker Compose

```yaml
# docker-compose.yml
version: '3'
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
    environment:
      OXICLOUD_DB_CONNECTION_STRING: "postgres://postgres:postgres@postgres/oxicloud"
    volumes:
      - storage_data:/app/storage
    depends_on:
      postgres:
        condition: service_healthy

volumes:
  pg_data:
  storage_data:
```

### Dockerfile

3-stage Alpine-based build:
1. **Cacher** -- pre-builds dependency layer
2. **Builder** -- compiles OxiCloud (`rust:1.93.0-alpine3.23`)
3. **Runtime** -- minimal Alpine image (`alpine:3.23.3`) with `libgcc`, `ca-certificates`, `libpq`, `tzdata`, `su-exec`

Non-root user: `oxicloud` (UID/GID 1001). Exposed port: `8086`. Entrypoint: `entrypoint.sh` (chown storage + drop privileges via `su-exec`).

---

## Environment Variables

### Server

| Variable | Default | Description |
|---|---|---|
| `OXICLOUD_STORAGE_PATH` | `./storage` | Root storage directory |
| `OXICLOUD_STATIC_PATH` | `./static` | Static files directory |
| `OXICLOUD_SERVER_PORT` | `8085` | Server port (note: `main.rs` hardcodes `8086`) |
| `OXICLOUD_SERVER_HOST` | `127.0.0.1` | Server bind address |

### Database

| Variable | Default | Description |
|---|---|---|
| `OXICLOUD_DB_CONNECTION_STRING` | `postgres://postgres:postgres@localhost:5432/oxicloud` | PostgreSQL connection string |
| `OXICLOUD_DB_MAX_CONNECTIONS` | `20` | Max pool connections |
| `OXICLOUD_DB_MIN_CONNECTIONS` | `5` | Min pool connections |

### Authentication

| Variable | Default | Description |
|---|---|---|
| `OXICLOUD_JWT_SECRET` | (auto-generated) | JWT signing secret. If empty, a random 32-byte hex secret is generated per session |
| `OXICLOUD_ACCESS_TOKEN_EXPIRY_SECS` | `3600` (1h) | Access token lifetime |
| `OXICLOUD_REFRESH_TOKEN_EXPIRY_SECS` | `2592000` (30d) | Refresh token lifetime |

### Feature Flags

| Variable | Default | Description |
|---|---|---|
| `OXICLOUD_ENABLE_AUTH` | `true` | Enable authentication system |
| `OXICLOUD_ENABLE_USER_STORAGE_QUOTAS` | `false` | Enable per-user storage quotas |
| `OXICLOUD_ENABLE_FILE_SHARING` | `true` | Enable file/folder sharing |
| `OXICLOUD_ENABLE_TRASH` | `true` | Enable trash/recycle bin |
| `OXICLOUD_ENABLE_SEARCH` | `true` | Enable search functionality |

### OIDC / SSO

| Variable | Default | Description |
|---|---|---|
| `OXICLOUD_OIDC_ENABLED` | `false` | Enable OIDC authentication |
| `OXICLOUD_OIDC_ISSUER_URL` | (empty) | OIDC provider issuer URL |
| `OXICLOUD_OIDC_CLIENT_ID` | (empty) | OIDC client ID |
| `OXICLOUD_OIDC_CLIENT_SECRET` | (empty) | OIDC client secret |
| `OXICLOUD_OIDC_REDIRECT_URI` | `http://localhost:8086/api/auth/oidc/callback` | Callback URL |
| `OXICLOUD_OIDC_SCOPES` | `openid profile email` | Requested OIDC scopes |
| `OXICLOUD_OIDC_FRONTEND_URL` | `http://localhost:8086` | Frontend URL for redirects |
| `OXICLOUD_OIDC_AUTO_PROVISION` | `true` | Auto-create users on first login |
| `OXICLOUD_OIDC_ADMIN_GROUPS` | (empty) | OIDC groups that grant admin role |
| `OXICLOUD_OIDC_DISABLE_PASSWORD_LOGIN` | `false` | Disable password login when OIDC is active |
| `OXICLOUD_OIDC_PROVIDER_NAME` | `SSO` | Display name for the OIDC provider |

### OIDC Validation

If **OXICLOUD_OIDC_ENABLED** is `true` but **issuer_url**, **client_id**, or **client_secret** are empty, OIDC is automatically disabled with an error log.

---

## Internal Configuration (Not Environment-Configurable)

Hardcoded defaults in `src/common/config.rs`:

### Cache

| Parameter | Default |
|---|---|
| File cache TTL | 60,000 ms (1 min) |
| Directory cache TTL | 120,000 ms (2 min) |
| Max cache entries | 10,000 |

### Timeouts

| Parameter | Default |
|---|---|
| File operation | 10,000 ms |
| Directory operation | 30,000 ms |
| Lock acquisition | 5,000 ms |
| Network operation | 15,000 ms |

### Resources

| Parameter | Default |
|---|---|
| Large file threshold | 100 MB |
| Large directory threshold | 1,000 entries |
| Streaming chunk size | 1 MB |
| Max in-memory file size | 50 MB |

### Concurrency

| Parameter | Default |
|---|---|
| Max concurrent files | 10 |
| Max concurrent dirs | 5 |
| Max concurrent I/O | 20 |
| Max parallel chunks | 8 |
| Min size for parallel chunks | 200 MB |
| Parallel chunk size | 8 MB |

### Storage

| Parameter | Default |
|---|---|
| Trash retention | 30 days |

### Auth Hashing (Argon2id)

| Parameter | Default |
|---|---|
| Memory cost | 65,536 KB (64 MB) |
| Time cost | 3 iterations |

---

## Feature Dependency Matrix

| Feature | Requires DB | Requires Auth | Feature Flag |
|---|---|---|---|
| File storage | Yes | No | Always on |
| Authentication | Yes | -- | `OXICLOUD_ENABLE_AUTH` |
| OIDC / SSO | Yes | Yes | `OXICLOUD_OIDC_ENABLED` |
| File sharing | Yes | Yes | `OXICLOUD_ENABLE_FILE_SHARING` |
| Trash | Yes | No | `OXICLOUD_ENABLE_TRASH` |
| Search | Yes | No | `OXICLOUD_ENABLE_SEARCH` |
| Favorites | Yes | Yes | Always on (when DB available) |
| Recent items | Yes | Yes | Always on (when DB available) |
| Storage quotas | Yes | Yes | `OXICLOUD_ENABLE_USER_STORAGE_QUOTAS` |
| Admin panel | Yes | Yes | Always on (when auth enabled) |
| WebDAV | Yes | Optional | Always on |
| CalDAV | Yes | Yes | Always on (when DB available) |
| CardDAV | Yes | Yes | Always on (when DB available) |
| Deduplication | No | No | Always on |
| Thumbnails | No | No | Always on |
| Chunked uploads | No | No | Always on |
