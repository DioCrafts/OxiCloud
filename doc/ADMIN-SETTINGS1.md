# OxiCloud — Admin Settings

## Overview

OxiCloud provides an admin panel API for managing server settings, OIDC configuration, user management, and dashboard statistics. All admin endpoints require a valid JWT token with `role = "admin"`.

## Architecture

| Layer | Component | File |
|---|---|---|
| Domain Port | `SettingsRepository` trait | `src/domain/repositories/settings_repository.rs` |
| Application Service | `AdminSettingsService` | `src/application/services/admin_settings_service.rs` |
| Application DTOs | Settings and user management DTOs | `src/application/dtos/settings_dto.rs` |
| Infrastructure | `SettingsPgRepository` | `src/infrastructure/repositories/pg/settings_pg_repository.rs` |
| Interfaces | `admin_handler` functions | `src/interfaces/api/handlers/admin_handler.rs` |

## REST API

All routes under `/api/admin`, require admin JWT.

### Settings

| Method | Path | Handler | Description |
|---|---|---|---|
| `GET` | `/api/admin/settings/oidc` | `get_oidc_settings` | Get current OIDC configuration |
| `PUT` | `/api/admin/settings/oidc` | `save_oidc_settings` | Update OIDC configuration |
| `POST` | `/api/admin/settings/oidc/test` | `test_oidc_connection` | Test OIDC provider connectivity |
| `GET` | `/api/admin/settings/general` | `get_general_settings` | Get general server settings |

### Dashboard

| Method | Path | Handler | Description |
|---|---|---|---|
| `GET` | `/api/admin/dashboard` | `get_dashboard_stats` | Server dashboard statistics |

### User Management

| Method | Path | Handler | Description |
|---|---|---|---|
| `GET` | `/api/admin/users` | `list_users` | List all users (paginated) |
| `GET` | `/api/admin/users/{id}` | `get_user` | Get user details |
| `DELETE` | `/api/admin/users/{id}` | `delete_user` | Delete a user |
| `PUT` | `/api/admin/users/{id}/role` | `update_user_role` | Change user role |
| `PUT` | `/api/admin/users/{id}/active` | `update_user_active` | Activate/deactivate user |
| `PUT` | `/api/admin/users/{id}/quota` | `update_user_quota` | Set storage quota |

### Safety Guards

- **Self-deletion blocked**: Admins cannot delete their own account
- **Self-role-change blocked**: Admins cannot change their own role
- **Self-deactivation blocked**: Admins cannot deactivate themselves

## OIDC Settings Management

### Get Settings Response

```json
{
  "enabled": true,
  "issuer_url": "https://keycloak.example.com/realms/main",
  "client_id": "oxicloud",
  "client_secret_set": true,
  "scopes": "openid profile email",
  "auto_provision": true,
  "admin_groups": "oxicloud-admins",
  "disable_password_login": false,
  "provider_name": "KeyCloak",
  "callback_url": "https://oxicloud.example.com/api/auth/oidc/callback",
  "env_overrides": ["issuer_url", "client_id"]
}
```

The `env_overrides` field lists which settings are overridden by environment variables (env vars take priority over DB settings).

### Save Settings Request

```json
{
  "enabled": true,
  "issuer_url": "https://keycloak.example.com/realms/main",
  "client_id": "oxicloud",
  "client_secret": "new-secret",
  "scopes": "openid profile email",
  "auto_provision": true,
  "admin_groups": "oxicloud-admins",
  "disable_password_login": false,
  "provider_name": "KeyCloak"
}
```

After saving, the service hot-reloads OIDC via `auth_app_service.reload_oidc()` or `disable_oidc()`.

### Test OIDC Connection

```json
// Request
{ "issuer_url": "https://keycloak.example.com/realms/main" }

// Response
{
  "success": true,
  "message": "Successfully connected to OIDC provider",
  "issuer": "https://keycloak.example.com/realms/main",
  "authorization_endpoint": "https://keycloak.example.com/realms/main/protocol/openid-connect/auth",
  "token_endpoint": "https://keycloak.example.com/realms/main/protocol/openid-connect/token",
  "userinfo_endpoint": "https://keycloak.example.com/realms/main/protocol/openid-connect/userinfo",
  "provider_name_suggestion": "KeyCloak"
}
```

## Dashboard Statistics

```json
{
  "server_version": "0.3.2",
  "auth_enabled": true,
  "oidc_configured": true,
  "quotas_enabled": false,
  "total_users": 42,
  "active_users": 38,
  "admin_users": 2,
  "total_quota_bytes": 107374182400,
  "total_used_bytes": 53687091200,
  "storage_usage_percent": 50.0,
  "users_over_80_percent": 5,
  "users_over_quota": 1
}
```

## User Management DTOs

```rust
pub struct UpdateUserRoleDto { pub role: String }       // "user" | "admin"
pub struct UpdateUserActiveDto { pub active: bool }
pub struct UpdateUserQuotaDto { pub quota_bytes: i64 }
pub struct ListUsersQueryDto { pub limit: Option<i64>, pub offset: Option<i64> }
```

## Config Priority

Settings are resolved with the following priority (highest first):

1. **Environment variables** (`OXICLOUD_OIDC_*`)
2. **Database settings** (`auth.admin_settings` table)
3. **Defaults**

## Database Schema

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

## Frontend

The admin panel is served from `static/admin.html`.
