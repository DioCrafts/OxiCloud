# 31 - OIDC Config Examples

Configuration examples for integrating with different OIDC (OpenID Connect) providers. One OIDC provider per instance.

## Table of Contents

1. [General OIDC Configuration](#general-oidc-configuration)
2. [Authentik](#authentik)
3. [Authelia](#authelia)
4. [KeyCloak](#keycloak)
5. [Troubleshooting](#troubleshooting)

## General OIDC Configuration

To enable OIDC, set these environment variables:

```bash
# Enable OIDC
OXICLOUD_OIDC_ENABLED=true

# OIDC provider configuration
OXICLOUD_OIDC_PROVIDER_NAME="Display Name"
OXICLOUD_OIDC_ISSUER_URL="https://provider.example.com/realms/your-realm"
OXICLOUD_OIDC_CLIENT_ID="your-client-id"
OXICLOUD_OIDC_CLIENT_SECRET="your-client-secret"
OXICLOUD_OIDC_REDIRECT_URI="https://your-oxicloud.example.com/api/auth/oidc/callback"
OXICLOUD_OIDC_SCOPES="openid profile email"
OXICLOUD_OIDC_FRONTEND_URL="https://your-oxicloud.example.com"
OXICLOUD_OIDC_AUTO_PROVISION="true"
OXICLOUD_OIDC_ADMIN_GROUPS="admin-group"
OXICLOUD_OIDC_DISABLE_PASSWORD_LOGIN="false"
```

## Authentik

[Authentik](https://goauthentik.io/) is an open-source identity platform providing authentication, authorization, and user management.

### 1. Create an Application in Authentik

1. Log into the Authentik admin panel
2. Go to "Applications" -> "Create"
3. Enter a name for the application (e.g. "OxiCloud")
4. Select "OAuth2/OpenID Provider" as the provider type
5. In the OAuth2 configuration:
   - **Redirect URI/Callback URL**: `https://your-oxicloud.example.com/api/auth/oidc/callback`
   - **Client Type**: Confidential
   - **Client ID**: auto-generated (note it down)
   - **Client Secret**: auto-generated (note it down)
   - **Scopes**: openid, email, profile
6. In the UI configuration:
   - **Launch URL**: `https://your-oxicloud.example.com/`
   - **Icon**: optional

### 2. Configure for Authentik

```yaml
# docker-compose.yml
version: '3'
services:
  oxicloud:
    image: oxicloud:latest
    environment:
      OXICLOUD_OIDC_ENABLED: "true"
      OXICLOUD_OIDC_PROVIDER_NAME: "Authentik"
      OXICLOUD_OIDC_ISSUER_URL: "https://authentik.example.com/application/o/oxicloud"
      OXICLOUD_OIDC_CLIENT_ID: "your-authentik-client-id"
      OXICLOUD_OIDC_CLIENT_SECRET: "your-authentik-client-secret"
      OXICLOUD_OIDC_REDIRECT_URI: "https://oxicloud.example.com/api/auth/oidc/callback"
      OXICLOUD_OIDC_SCOPES: "openid profile email"
      OXICLOUD_OIDC_FRONTEND_URL: "https://oxicloud.example.com"
      OXICLOUD_OIDC_AUTO_PROVISION: "true"
      OXICLOUD_OIDC_ADMIN_GROUPS: ""
      OXICLOUD_OIDC_DISABLE_PASSWORD_LOGIN: "false"
    ports:
      - "8086:8086"
    volumes:
      - ./storage:/app/storage
```

## Authelia

[Authelia](https://www.authelia.com/) is an open-source multi-factor authentication solution.

### 1. Configure Authelia

Edit your Authelia configuration (`configuration.yml`):

```yaml
identity_providers:
  oidc:
    hmac_secret: your-secure-secret  # Change to a secure random value
    issuer_private_key: /config/private.pem  # Path to your private key
    cors:
      endpoints: ['authorization', 'token', 'revocation', 'introspection']
      allowed_origins:
        - https://oxicloud.example.com
    clients:
      - id: oxicloud
        description: OxiCloud
        secret: your-secure-client-secret  # Change this
        public: false
        authorization_policy: two_factor
        redirect_uris:
          - https://oxicloud.example.com/api/auth/oidc/callback
        scopes: ['openid', 'profile', 'email', 'groups']
        userinfo_signing_algorithm: none
```

### 2. Configure for Authelia

```yaml
# docker-compose.yml
version: '3'
services:
  oxicloud:
    image: oxicloud:latest
    environment:
      OXICLOUD_OIDC_ENABLED: "true"
      OXICLOUD_OIDC_PROVIDER_NAME: "Authelia"
      OXICLOUD_OIDC_ISSUER_URL: "https://authelia.example.com"
      OXICLOUD_OIDC_CLIENT_ID: "oxicloud"
      OXICLOUD_OIDC_CLIENT_SECRET: "your-secure-client-secret"
      OXICLOUD_OIDC_REDIRECT_URI: "https://oxicloud.example.com/api/auth/oidc/callback"
      OXICLOUD_OIDC_SCOPES: "openid profile email groups"
      OXICLOUD_OIDC_FRONTEND_URL: "https://oxicloud.example.com"
      OXICLOUD_OIDC_AUTO_PROVISION: "true"
      OXICLOUD_OIDC_ADMIN_GROUPS: ""
      OXICLOUD_OIDC_DISABLE_PASSWORD_LOGIN: "false"
    ports:
      - "8086:8086"
    volumes:
      - ./storage:/app/storage
```

## KeyCloak

[KeyCloak](https://www.keycloak.org/) is an open-source identity and access management solution.

### 1. Create a Client in KeyCloak

1. Log into the KeyCloak admin console
2. Select your Realm
3. Go to "Clients" -> "Create"
4. Fill in the form:
   - **Client ID**: `oxicloud`
   - **Client Protocol**: `openid-connect`
   - **Root URL**: `https://oxicloud.example.com`
5. In the client configuration:
   - **Access Type**: `confidential`
   - **Valid Redirect URIs**: `https://oxicloud.example.com/api/auth/oidc/callback`
   - **Web Origins**: `https://oxicloud.example.com` (or `+` to allow all origins)
6. Save the configuration
7. Go to the "Credentials" tab and copy the generated "Secret"

### 2. Configure for KeyCloak

```yaml
# docker-compose.yml
version: '3'
services:
  oxicloud:
    image: oxicloud:latest
    environment:
      OXICLOUD_OIDC_ENABLED: "true"
      OXICLOUD_OIDC_PROVIDER_NAME: "KeyCloak"
      OXICLOUD_OIDC_ISSUER_URL: "https://keycloak.example.com/realms/your-realm"
      OXICLOUD_OIDC_CLIENT_ID: "oxicloud"
      OXICLOUD_OIDC_CLIENT_SECRET: "your-keycloak-client-secret"
      OXICLOUD_OIDC_REDIRECT_URI: "https://oxicloud.example.com/api/auth/oidc/callback"
      OXICLOUD_OIDC_SCOPES: "openid profile email"
      OXICLOUD_OIDC_FRONTEND_URL: "https://oxicloud.example.com"
      OXICLOUD_OIDC_AUTO_PROVISION: "true"
      OXICLOUD_OIDC_ADMIN_GROUPS: "oxicloud-admins"
      OXICLOUD_OIDC_DISABLE_PASSWORD_LOGIN: "false"
    ports:
      - "8086:8086"
    volumes:
      - ./storage:/app/storage
```

## Troubleshooting

### Error: "Failed to discover OIDC provider"

The backend cannot reach the provider's discovery endpoint.

**Fixes:**
1. Verify the discovery URL is correct
2. Check that the backend can reach the URL (firewalls, DNS, etc.)
3. If using a self-signed certificate, configure the appropriate trust

### Error: "Invalid redirect URI"

The OIDC provider is rejecting the redirect URI.

**Fixes:**
1. Make sure the redirect URI configured in the backend matches exactly what is registered in the provider
2. Check for protocol differences (http vs https), port, or path mismatches

### Error: "User does not exist and auto-creation is disabled"

**Fixes:**
1. Enable auto-provisioning: `OXICLOUD_OIDC_AUTO_PROVISION="true"`
2. Or manually create the user before attempting OIDC login

### Error: "Could not extract user ID from claim"

The backend cannot find the user ID attribute in the token claims.

**Fixes:**
1. Verify the provider returns the `sub` claim in tokens
2. Make sure scopes in **OXICLOUD_OIDC_SCOPES** include `openid`
3. Configure the provider to include the required claims in tokens

See `oidc-architecture.md` and `oidc-integration.md` for deeper technical details.
