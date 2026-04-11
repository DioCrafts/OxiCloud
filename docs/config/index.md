# Configuration

OxiCloud is configured entirely via **environment variables** (no config files needed).

## Sections

- [Deployment & Docker](/config/deployment) — Docker Compose, Kubernetes Helm chart, image details
- [Environment Variables](/config/env) — complete reference of all `OXICLOUD_*` variables
- [OIDC / SSO](/config/oidc) — single sign-on with Keycloak, Authentik, Authelia, Google, Azure AD
- [WOPI (Office Editing)](/config/wopi) — Collabora Online / OnlyOffice integration

## Minimal `.env`

```bash
OXICLOUD_DB_CONNECTION_STRING=postgres://postgres:postgres@postgres:5432/oxicloud
OXICLOUD_STORAGE_PATH=/app/storage
OXICLOUD_SERVER_HOST=0.0.0.0
OXICLOUD_SERVER_PORT=8086
```

That's enough to get started. All other settings have sensible defaults.
