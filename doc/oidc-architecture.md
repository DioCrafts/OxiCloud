# 29 - OIDC Architecture

OpenID Connect (OIDC) authentication follows the Authorization Code Flow. The system supports multiple identity providers (Authentik, Authelia, KeyCloak) through a single configurable integration point.

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────────────┐
│                                                                         │
│                          IDENTITY PROVIDER                              │
│                                                                         │
│    ┌───────────────┐      ┌───────────────┐      ┌───────────────┐     │
│    │               │      │               │      │               │     │
│    │   Authentik   │      │   Authelia    │      │   KeyCloak    │     │
│    │               │      │               │      │               │     │
│    └───────┬───────┘      └───────┬───────┘      └───────┬───────┘     │
│            │                      │                      │             │
└────────────┼──────────────────────┼──────────────────────┼─────────────┘
             │                      │                      │
             │                      │                      │
             │                      │                      │
             │                     OIDC                    │
             │                      │                      │
             │                      │                      │
┌────────────┼──────────────────────┼──────────────────────┼─────────────┐
│            │                      │                      │             │
│            ▼                      ▼                      ▼             │
│    ┌───────────────────────────────────────────────────────────────┐   │
│    │                                                               │   │
│    │                          OXICLOUD                             │   │
│    │                                                               │   │
│    │   ┌───────────────┐      ┌───────────────┐                    │   │
│    │   │               │      │               │                    │   │
│    │   │ OidcService   │◄────►│ AuthService   │                    │   │
│    │   │               │      │               │                    │   │
│    │   └───────┬───────┘      └───────┬───────┘                    │   │
│    │           │                      │                            │   │
│    │           ▼                      ▼                            │   │
│    │   ┌───────────────────────────────────────────┐               │   │
│    │   │                                           │               │   │
│    │   │       AuthApplicationService              │               │   │
│    │   │                                           │               │   │
│    │   └───────────────────┬───────────────────────┘               │   │
│    │                       │                                        │   │
│    │                       ▼                                        │   │
│    │   ┌───────────────────────────────────────────┐               │   │
│    │   │                                           │               │   │
│    │   │             Auth Handler                  │               │   │
│    │   │                                           │               │   │
│    │   └───────────────────────────────────────────┘               │   │
│    │                                                               │   │
│    └───────────────────────────────────────────────────────────────┘   │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
                           ▲
                           │
                           │ HTTP/HTTPS
                           │
                           │
┌────────────────────────────────────────────────────────────────────────┐
│                                                                        │
│                           WEB BROWSER                                  │
│                                                                        │
│    ┌───────────────────────────────────────────────────────────────┐   │
│    │                                                               │   │
│    │                    User Interface                             │   │
│    │                                                               │   │
│    │    ┌──────────────┐        ┌──────────────┐                   │   │
│    │    │              │        │              │                   │   │
│    │    │ Login.html   │        │   auth.js    │                   │   │
│    │    │              │        │              │                   │   │
│    │    └──────────────┘        └──────────────┘                   │   │
│    │                                                               │   │
│    └───────────────────────────────────────────────────────────────┘   │
│                                                                        │
└────────────────────────────────────────────────────────────────────────┘
```

## OIDC Authentication Flow

The flow follows the standard Authorization Code Flow:

1. **Authentication Start** -- the user clicks "Login with [Provider]" on the login page. The frontend generates a random state for CSRF protection and requests an authorization URL from the backend.

2. **Redirect to Identity Provider** -- the backend generates and returns the authorization URL. The browser redirects the user to the provider's login page.

3. **Authentication at the Provider** -- the user authenticates (password, 2FA, etc.). The provider redirects back with an authorization code.

4. **Authorization Code Exchange** -- the frontend sends the authorization code to the backend. The backend exchanges it for access and ID tokens with the provider, then verifies the ID token and extracts user info.

5. **User Creation/Retrieval** -- the backend looks up an existing user by the provider's external ID. If none exists and auto-provisioning is enabled, a new user is created. If disabled, an error is returned.

6. **Session Token Generation** -- the backend generates its own access and refresh tokens for the user. These tokens authenticate subsequent API requests.

7. **Response to Client** -- tokens and user info are returned to the frontend. The frontend stores them and redirects to the main page.

## Main Components

### OidcService

Handles communication with OIDC providers:
- Discovers provider OIDC endpoints
- Generates authorization URLs
- Exchanges authorization codes for tokens
- Verifies tokens and extracts user info

### AuthApplicationService

Coordinates the authentication process:
- Acts as interface between the API layer and domain services
- Manages user creation/retrieval
- Coordinates access token generation

### Auth Handler

Exposes HTTP endpoints for the OIDC auth flow:
- `GET /api/auth/oidc/providers` -- lists available OIDC providers
- `GET /api/auth/oidc/authorize` -- generates an authorization URL for the OIDC provider
- `GET /api/auth/oidc/callback` -- receives the redirect from the provider with the authorization code
- `POST /api/auth/oidc/exchange` -- exchanges the authorization code for session tokens

### Frontend (login.html + auth.js)

Handles the client-side of the auth flow:
- Shows SSO button for the configured OIDC provider in `login.html`
- Initiates the authentication flow via `auth.js`
- Handles the return redirect from the provider
- Processes and stores session tokens

## Provider Configuration

One OIDC provider is configured per instance via environment variables prefixed with **OXICLOUD_OIDC_***:

1. **Single provider** per instance.
2. **Environment variables**: **OXICLOUD_OIDC_ENABLED**, **OXICLOUD_OIDC_ISSUER_URL**, **OXICLOUD_OIDC_CLIENT_ID**, **OXICLOUD_OIDC_CLIENT_SECRET**, etc.
3. **Auto-provisioning**: users can be created automatically on first OIDC login (**OXICLOUD_OIDC_AUTO_PROVISION**).
4. **Role mapping**: admin groups are configured via **OXICLOUD_OIDC_ADMIN_GROUPS**.

See `oidc-config-examples.md` for provider-specific configuration examples.

## Security

The OIDC implementation includes several security measures:

1. **CSRF protection** -- random state parameter prevents CSRF attacks.
2. **Token validation** -- JWT signatures and expiration are verified.
3. **Authorization Code Flow** -- more secure than the implicit flow.
4. **HTTPS** -- required for all OIDC communications.
5. **Client secrets** -- stored securely, never exposed to the frontend.
