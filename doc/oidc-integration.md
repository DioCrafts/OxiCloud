# 30 - OIDC Integration

OpenID Connect (OIDC) is an identity layer on top of OAuth 2.0. It lets clients verify user identity based on authentication performed by an authorization server and obtain basic profile information. Adding OIDC enables SSO with providers like Authentik, Authelia, and KeyCloak.

What it gives us:
1. Users authenticate with their existing IdP credentials
2. No need for separate username/password management
3. Modern auth best practices baked in
4. Seamless experience for users already on SSO

## OIDC Configuration

OIDC is configured separately from **AuthConfig** via **OidcConfig** in `src/common/config.rs`. This is a single-provider model -- one OIDC provider per instance:

```rust
/// OpenID Connect (OIDC) configuration
pub struct OidcConfig {
    pub enabled: bool,                   // Whether OIDC is enabled
    pub issuer_url: String,              // OIDC Issuer URL
    pub client_id: String,               // OIDC Client ID
    pub client_secret: String,           // OIDC Client Secret
    pub redirect_uri: String,            // Redirect URI (default: http://localhost:8086/api/auth/oidc/callback)
    pub scopes: String,                  // Scopes to request (default: "openid profile email")
    pub frontend_url: String,            // Frontend URL for post-login redirect
    pub auto_provision: bool,            // Auto-create users on first login (JIT provisioning)
    pub admin_groups: String,            // Comma-separated OIDC groups that map to admin role
    pub disable_password_login: bool,    // Disable password-based login entirely
    pub provider_name: String,           // Display name (default: "SSO")
}
```

Environment variables use the **OXICLOUD_OIDC_*** prefix:

```bash
OXICLOUD_OIDC_ENABLED=true
OXICLOUD_OIDC_ISSUER_URL="https://authentik.example.com/application/o/oxicloud/"
OXICLOUD_OIDC_CLIENT_ID="your-client-id"
OXICLOUD_OIDC_CLIENT_SECRET="your-client-secret"
OXICLOUD_OIDC_REDIRECT_URI="https://oxicloud.example.com/api/auth/oidc/callback"
OXICLOUD_OIDC_SCOPES="openid profile email"
OXICLOUD_OIDC_FRONTEND_URL="https://oxicloud.example.com"
OXICLOUD_OIDC_AUTO_PROVISION=true
OXICLOUD_OIDC_ADMIN_GROUPS="oxicloud-admins"
OXICLOUD_OIDC_DISABLE_PASSWORD_LOGIN=false
OXICLOUD_OIDC_PROVIDER_NAME="Authentik"
```

## OIDC Service Implementation

The OIDC service lives in the infrastructure layer at `src/infrastructure/services/oidc_service.rs` and implements the **OidcServicePort** trait defined in `src/application/ports/auth_ports.rs`:

```rust
// src/application/ports/auth_ports.rs — Port trait
#[async_trait]
pub trait OidcServicePort: Send + Sync + 'static {
    fn enabled(&self) -> bool;
    fn provider_name(&self) -> &str;
    fn generate_auth_url(&self, state: &str) -> Result<String, DomainError>;
    async fn exchange_code(&self, code: &str) -> Result<OidcTokenSet, DomainError>;
    async fn get_user_info(&self, token_set: &OidcTokenSet) -> Result<OidcIdClaims, DomainError>;
}

// src/infrastructure/services/oidc_service.rs — Implementation
pub struct OidcService {
    config: OidcConfig,
    http_client: reqwest::Client,
    // Discovery metadata cached after initialization
}

impl OidcService {
    pub async fn new(config: OidcConfig) -> Result<Self, DomainError> {
        // Discovers OIDC endpoints from issuer_url
        // ...
    }
}
```

Follows hexagonal architecture: the port (**OidcServicePort**) is in the application layer, and the implementation (**OidcService**) is in the infrastructure layer.

## User Entity OIDC Support

The **User** entity in `src/domain/entities/user.rs` supports OIDC users via two fields:

```rust
#[derive(Debug, Clone)]
pub struct User {
    // ... standard fields ...
    oidc_provider: Option<String>,   // OIDC provider name (e.g., "authentik")
    oidc_subject: Option<String>,    // OIDC subject identifier (unique ID from provider)
}

impl User {
    pub fn oidc_provider(&self) -> Option<&str> {
        self.oidc_provider.as_deref()
    }

    pub fn oidc_subject(&self) -> Option<&str> {
        self.oidc_subject.as_deref()
    }

    // Constructor for OIDC users
    pub fn new_oidc(username, email, role, quota, oidc_provider, oidc_subject) -> Self;
}
```

## Database Schema

The **auth.users** table includes OIDC columns:

```sql
ALTER TABLE auth.users ADD COLUMN IF NOT EXISTS oidc_provider VARCHAR(255);
ALTER TABLE auth.users ADD COLUMN IF NOT EXISTS oidc_subject VARCHAR(255);
CREATE UNIQUE INDEX IF NOT EXISTS idx_users_oidc ON auth.users(oidc_provider, oidc_subject) WHERE oidc_provider IS NOT NULL;
```

Users are matched by **oidc_provider** + **oidc_subject** combination. The **UserRepository** trait includes `get_user_by_oidc_subject()` for lookups.

## Auth Application Service

**AuthApplicationService** in `src/application/services/auth_application_service.rs` coordinates OIDC authentication:

```rust
impl AuthApplicationService {
    // Initialize with OIDC support
    pub fn with_oidc(self, oidc_service: Arc<dyn OidcServicePort>, oidc_config: OidcConfig) -> Self;

    // Reload OIDC configuration (for admin settings changes)
    pub async fn reload_oidc(&self, config: OidcConfig) -> Result<(), DomainError>;

    // Disable OIDC
    pub fn disable_oidc(&self);

    // Check if OIDC is enabled
    pub fn oidc_enabled(&self) -> bool;

    // Get OIDC config
    pub fn oidc_config(&self) -> Option<OidcConfig>;

    // Get OIDC service reference
    pub fn oidc_service(&self) -> Option<Arc<dyn OidcServicePort>>;

    // Check if password login is disabled
    pub fn password_login_disabled(&self) -> bool;

    // Prepare OIDC authorization URL
    pub fn prepare_oidc_authorize(&self) -> Result<OidcAuthorizeResponseDto, DomainError>;

    // Handle OIDC callback (exchange code for tokens)
    pub async fn oidc_callback(&self, code: &str, state: &str) -> Result<AuthResponseDto, DomainError>;
}
```

## Auth Handler Routes

OIDC endpoints in `src/interfaces/api/handlers/auth_handler.rs`:

```rust
// Public OIDC routes (no auth required) — nested under /api/auth/
.route("/status", get(get_system_status))
.route("/oidc/providers", get(oidc_providers))
.route("/oidc/authorize", get(oidc_authorize))
.route("/oidc/callback", get(oidc_callback))
.route("/oidc/exchange", post(oidc_exchange))
```

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/auth/oidc/providers` | Returns OIDC provider info (name, enabled state) |
| GET | `/api/auth/oidc/authorize` | Returns authorization URL for redirect to IdP |
| GET | `/api/auth/oidc/callback` | Receives callback redirect from IdP with auth code |
| POST | `/api/auth/oidc/exchange` | Exchanges auth code for JWT tokens |

## DTOs

DTOs in `src/application/dtos/user_dto.rs`:

```rust
// Response with authorization URL
#[derive(Debug, Clone, Serialize)]
pub struct OidcAuthorizeResponseDto {
    pub authorize_url: String,
    pub state: String,
}

// Query params received from IdP callback
#[derive(Debug, Clone, Deserialize)]
pub struct OidcCallbackQueryDto {
    pub code: String,
    pub state: String,
}

// Request to exchange code for tokens
#[derive(Debug, Clone, Deserialize)]
pub struct OidcExchangeDto {
    pub code: String,
    pub state: String,
}

// Provider info response
#[derive(Debug, Clone, Serialize)]
pub struct OidcProviderInfoDto {
    pub enabled: bool,
    pub provider_name: String,
    pub disable_password_login: bool,
}

// User info from OIDC claims
#[derive(Debug, Clone, Serialize)]
pub struct OidcUserInfoDto {
    pub subject: String,
    pub email: Option<String>,
    pub name: Option<String>,
    pub preferred_username: Option<String>,
    pub groups: Vec<String>,
}
```

## Frontend Integration

OIDC login is built directly into `static/login.html` and handled by `static/js/auth.js`. There is no separate `oidcAuth.js` file. The login page checks the system status endpoint to see if OIDC is enabled, then shows an SSO button accordingly.

```html
<!-- In login.html - SSO login button shown when OIDC is enabled -->
<button id="oidc-login-btn" class="btn btn-oidc" style="display:none">
  Login with SSO
</button>
```

## Configuration Example

KeyCloak setup via docker-compose:

```yaml
# docker-compose.yml
version: '3'
services:
  oxicloud:
    image: oxicloud:latest
    environment:
      OXICLOUD_OIDC_ENABLED: "true"
      OXICLOUD_OIDC_ISSUER_URL: "https://keycloak.example.com/realms/your-realm"
      OXICLOUD_OIDC_CLIENT_ID: "oxicloud"
      OXICLOUD_OIDC_CLIENT_SECRET: "your-client-secret"
      OXICLOUD_OIDC_REDIRECT_URI: "https://oxicloud.example.com/api/auth/oidc/callback"
      OXICLOUD_OIDC_SCOPES: "openid profile email"
      OXICLOUD_OIDC_FRONTEND_URL: "https://oxicloud.example.com"
      OXICLOUD_OIDC_AUTO_PROVISION: "true"
      OXICLOUD_OIDC_ADMIN_GROUPS: "oxicloud-admins"
      OXICLOUD_OIDC_DISABLE_PASSWORD_LOGIN: "false"
      OXICLOUD_OIDC_PROVIDER_NAME: "KeyCloak"
    ports:
      - "8086:8086"
    volumes:
      - ./storage:/app/storage
```

See `oidc-config-examples.md` for more provider-specific configurations.

## Additional Notes

1. **Security** -- always use HTTPS for OIDC connections. Ensure proper TLS configuration.
2. **User mapping** -- OIDC users are identified by **oidc_provider** + **oidc_subject** in the **auth.users** table. Groups from OIDC can map to admin role via **OXICLOUD_OIDC_ADMIN_GROUPS**.
3. **Single provider** -- one OIDC provider per instance. Managed via admin settings UI or environment variables.
4. **Session management** -- after OIDC authentication, the backend generates its own JWT access/refresh tokens. Sessions work identically to password-based login from that point.
5. **Access control** -- OIDC users share the same permissions model as local users. Admin role can be auto-assigned based on OIDC group membership.
6. **Testing** -- use the admin settings UI (`/admin.html`) to configure and test OIDC connections.
