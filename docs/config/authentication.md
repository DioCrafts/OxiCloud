# Authentication

OxiCloud ships with JWT-based authentication and Argon2id password hashing for local accounts. It also exposes status and OIDC-related auth endpoints under the same `/api/auth` namespace.

## Core Endpoints

| Method | Endpoint | Description |
| --- | --- | --- |
| `POST` | `/api/auth/register` | Create a local user account |
| `POST` | `/api/auth/login` | Exchange username and password for access and refresh tokens |
| `POST` | `/api/auth/refresh` | Refresh the session tokens |
| `GET` | `/api/auth/me` | Return the current authenticated user |
| `PUT` | `/api/auth/change-password` | Change the current user's password |
| `POST` | `/api/auth/logout` | Invalidate the current session |
| `GET` | `/api/auth/status` | Return auth system state, including OIDC availability |

## OIDC Endpoints Under Auth

| Method | Endpoint | Description |
| --- | --- | --- |
| `GET` | `/api/auth/oidc/providers` | List configured OIDC provider info |
| `GET` | `/api/auth/oidc/authorize` | Build the authorization redirect URL |
| `GET` | `/api/auth/oidc/callback` | Handle provider redirect callback |
| `POST` | `/api/auth/oidc/exchange` | Exchange the auth code for OxiCloud session tokens |

## Example Flows

### Register

```json
{
  "username": "testuser",
  "email": "test@example.com",
  "password": "SecurePassword123"
}
```

### Login

```json
{
  "username": "testuser",
  "password": "SecurePassword123"
}
```

Typical successful login response:

```json
{
  "accessToken": "...",
  "refreshToken": "...",
  "expiresIn": 3600
}
```

### Current User

`GET /api/auth/me` returns the authenticated user's identity, role, and storage information.

## Security Model

- local passwords are hashed with Argon2id
- access control is role-based (`admin` and `user`)
- refresh tokens support session renewal without forcing frequent re-login
- OIDC can coexist with local auth or disable password login entirely

## Related Pages

- [OIDC / SSO](/config/oidc)
- [Admin Settings](/config/admin-settings)
- [Environment Variables](/config/env)