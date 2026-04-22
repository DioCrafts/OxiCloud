# Environment Variables

Most runtime variables use the `OXICLOUD_` prefix. A few build-time or allocator variables do not.

## Server

| Variable | Default | Description |
|---|---|---|
| `OXICLOUD_STORAGE_PATH` | `./storage` | Root storage directory |
| `OXICLOUD_STATIC_PATH` | `./static` | Static files directory |
| `OXICLOUD_SERVER_PORT` | `8086` | Server port |
| `OXICLOUD_SERVER_HOST` | `127.0.0.1` | Server bind address |
| `OXICLOUD_BASE_URL` | (auto) | Public base URL for share links |

## Database

| Variable | Default | Description |
|---|---|---|
| `OXICLOUD_DB_CONNECTION_STRING` | `postgres://postgres:postgres@localhost:5432/oxicloud` | PostgreSQL connection string |
| `OXICLOUD_DB_MAX_CONNECTIONS` | `20` | Max pool connections |
| `OXICLOUD_DB_MIN_CONNECTIONS` | `5` | Min pool connections |
| `OXICLOUD_DB_MAINTENANCE_MAX_CONNECTIONS` | `5` | Max connections in the isolated maintenance pool |
| `OXICLOUD_DB_MAINTENANCE_MIN_CONNECTIONS` | `1` | Min connections in the isolated maintenance pool |

## Build-Time SQLx

| Variable | Default | Description |
|---|---|---|
| `DATABASE_URL` | — | Build-time database URL for SQLx compile-time checks |

## Authentication

| Variable | Default | Description |
|---|---|---|
| `OXICLOUD_JWT_SECRET` | (random) | JWT signing secret |
| `OXICLOUD_ACCESS_TOKEN_EXPIRY_SECS` | `3600` | Access token lifetime (seconds) |
| `OXICLOUD_REFRESH_TOKEN_EXPIRY_SECS` | `2592000` | Refresh token lifetime (seconds) |

## Feature Flags

| Variable | Default | Description |
|---|---|---|
| `OXICLOUD_ENABLE_AUTH` | `true` | Enable authentication |
| `OXICLOUD_ENABLE_USER_STORAGE_QUOTAS` | `false` | Per-user storage quotas |
| `OXICLOUD_ENABLE_FILE_SHARING` | `true` | File/folder sharing |
| `OXICLOUD_ENABLE_TRASH` | `true` | Trash / recycle bin |
| `OXICLOUD_ENABLE_SEARCH` | `true` | Search |

## OIDC / SSO

See the [OIDC configuration guide](/config/oidc) for details.

| Variable | Default | Description |
|---|---|---|
| `OXICLOUD_OIDC_ENABLED` | `false` | Enable OIDC |
| `OXICLOUD_OIDC_ISSUER_URL` | — | OIDC issuer URL |
| `OXICLOUD_OIDC_CLIENT_ID` | — | Client ID |
| `OXICLOUD_OIDC_CLIENT_SECRET` | — | Client secret |
| `OXICLOUD_OIDC_REDIRECT_URI` | `http://localhost:8086/api/auth/oidc/callback` | Callback URL |
| `OXICLOUD_OIDC_SCOPES` | `openid profile email` | Requested scopes |
| `OXICLOUD_OIDC_FRONTEND_URL` | `http://localhost:8086` | Frontend URL |
| `OXICLOUD_OIDC_AUTO_PROVISION` | `true` | Auto-create users on first SSO login |
| `OXICLOUD_OIDC_ADMIN_GROUPS` | — | Groups that grant admin role |
| `OXICLOUD_OIDC_DISABLE_PASSWORD_LOGIN` | `false` | Hide password form when OIDC enabled |
| `OXICLOUD_OIDC_PROVIDER_NAME` | `SSO` | Display name for the provider |

## WOPI (Office Editing)

See the [WOPI configuration guide](/config/wopi) for details.

| Variable | Default | Description |
|---|---|---|
| `OXICLOUD_WOPI_ENABLED` | `false` | Enable WOPI |
| `OXICLOUD_WOPI_DISCOVERY_URL` | — | Collabora/OnlyOffice discovery URL |
| `OXICLOUD_WOPI_SECRET` | (JWT secret) | WOPI token signing key |
| `OXICLOUD_WOPI_TOKEN_TTL_SECS` | `86400` | Token lifetime |
| `OXICLOUD_WOPI_LOCK_TTL_SECS` | `1800` | Lock expiration |

## Allocator Tuning

These variables are read directly by **mimalloc**, not by OxiCloud's config parser.

| Variable | Default | Description |
|---|---|---|
| `MIMALLOC_PURGE_DELAY` | `0` | Delay in ms before freed memory is returned to the OS |
| `MIMALLOC_ALLOW_LARGE_OS_PAGES` | `0` | Enable or disable large OS pages for allocations |

## Internal Defaults (not configurable via env)

| Parameter | Default |
|---|---|
| File cache TTL | 60 s |
| Directory cache TTL | 120 s |
| Max cache entries | 10 000 |
| Large file threshold | 100 MB |
| Streaming chunk size | 1 MB |
| Max parallel chunks | 8 |
| Trash retention | 30 days |
| Argon2id memory cost | 64 MB |
| Argon2id time cost | 3 iterations |
