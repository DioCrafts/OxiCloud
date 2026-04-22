# Admin Settings

OxiCloud exposes an admin API for runtime configuration, dashboard stats, and user administration. All routes live under `/api/admin` and require an authenticated admin JWT.

## Settings Endpoints

| Method | Path | Description |
| --- | --- | --- |
| `GET` | `/api/admin/settings/oidc` | Read current OIDC settings |
| `PUT` | `/api/admin/settings/oidc` | Save OIDC settings |
| `POST` | `/api/admin/settings/oidc/test` | Test provider connectivity |
| `GET` | `/api/admin/settings/general` | Read general server settings |

The OIDC runtime UI complements the base configuration described in [OIDC / SSO](/config/oidc) and the provider samples in [OIDC Config Examples](/config/oidc-config-examples).

## Dashboard Endpoint

| Method | Path | Description |
| --- | --- | --- |
| `GET` | `/api/admin/dashboard` | Read server statistics and feature state |

Typical dashboard fields include:

- server version
- whether auth and OIDC are enabled
- whether quotas are enabled
- total, active, and admin user counts
- quota usage totals and percentage

## User Management Endpoints

| Method | Path | Description |
| --- | --- | --- |
| `GET` | `/api/admin/users` | List users |
| `GET` | `/api/admin/users/{id}` | Get one user |
| `DELETE` | `/api/admin/users/{id}` | Delete a user |
| `PUT` | `/api/admin/users/{id}/role` | Change role |
| `PUT` | `/api/admin/users/{id}/active` | Activate or deactivate a user |
| `PUT` | `/api/admin/users/{id}/quota` | Update a storage quota |

### Built-in safety guards

- Admins cannot delete their own account
- Admins cannot change their own role
- Admins cannot deactivate themselves

## OIDC Settings Priority

When the same setting exists in multiple places, OxiCloud resolves it in this order:

1. Environment variables such as `OXICLOUD_OIDC_*`
2. Values stored in the admin settings table
3. Built-in defaults

If a value is overridden by environment variables, the admin API can expose that in the response so operators know why a saved value is not taking effect.

## Test Connection Example

```json
{
  "issuer_url": "https://keycloak.example.com/realms/main"
}
```

Successful responses include discovered endpoints such as the authorization endpoint, token endpoint, and userinfo endpoint.

## Data Storage

Runtime settings are stored in `auth.admin_settings`.

```sql
CREATE TABLE IF NOT EXISTS auth.admin_settings (
    key        TEXT PRIMARY KEY,
    value      TEXT NOT NULL,
    category   TEXT NOT NULL,
    is_secret  BOOLEAN DEFAULT FALSE,
    updated_by VARCHAR(36),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
```

## Related Pages

- [OIDC / SSO](/config/oidc)
- [OIDC Config Examples](/config/oidc-config-examples)
- [Environment Variables](/config/env)