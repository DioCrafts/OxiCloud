# OIDC Config Examples

This page collects provider-specific OpenID Connect examples for OxiCloud. Use it together with the base reference in [OIDC / SSO](/config/oidc).

## Base Environment Variables

```bash
OXICLOUD_OIDC_ENABLED=true
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

1. Create an OAuth2 or OpenID Connect application for OxiCloud
2. Register `https://your-oxicloud.example.com/api/auth/oidc/callback` as the callback URL
3. Copy the generated client ID and client secret

```yaml
services:
  oxicloud:
    image: diocrafts/oxicloud:latest
    environment:
      OXICLOUD_OIDC_ENABLED: "true"
      OXICLOUD_OIDC_PROVIDER_NAME: "Authentik"
      OXICLOUD_OIDC_ISSUER_URL: "https://authentik.example.com/application/o/oxicloud"
      OXICLOUD_OIDC_CLIENT_ID: "your-authentik-client-id"
      OXICLOUD_OIDC_CLIENT_SECRET: "your-authentik-client-secret"
      OXICLOUD_OIDC_REDIRECT_URI: "https://oxicloud.example.com/api/auth/oidc/callback"
      OXICLOUD_OIDC_SCOPES: "openid profile email"
      OXICLOUD_OIDC_FRONTEND_URL: "https://oxicloud.example.com"
```

## Authelia

Configure an OIDC client in Authelia and allow OxiCloud's redirect URI.

```yaml
identity_providers:
  oidc:
    clients:
      - id: oxicloud
        description: OxiCloud
        public: false
        redirect_uris:
          - https://oxicloud.example.com/api/auth/oidc/callback
        scopes: [openid, profile, email, groups]
```

```yaml
services:
  oxicloud:
    image: diocrafts/oxicloud:latest
    environment:
      OXICLOUD_OIDC_PROVIDER_NAME: "Authelia"
      OXICLOUD_OIDC_ISSUER_URL: "https://authelia.example.com"
      OXICLOUD_OIDC_CLIENT_ID: "oxicloud"
      OXICLOUD_OIDC_CLIENT_SECRET: "your-client-secret"
```

## Keycloak

1. Create a confidential client named `oxicloud`
2. Set the valid redirect URI to `https://oxicloud.example.com/api/auth/oidc/callback`
3. Copy the generated client secret

```yaml
services:
  oxicloud:
    image: diocrafts/oxicloud:latest
    environment:
      OXICLOUD_OIDC_PROVIDER_NAME: "Keycloak"
      OXICLOUD_OIDC_ISSUER_URL: "https://keycloak.example.com/realms/your-realm"
      OXICLOUD_OIDC_CLIENT_ID: "oxicloud"
      OXICLOUD_OIDC_CLIENT_SECRET: "your-keycloak-client-secret"
      OXICLOUD_OIDC_ADMIN_GROUPS: "oxicloud-admins"
```

## Troubleshooting

### Failed to discover the provider

- Verify the issuer URL exactly matches the provider's discovery URL base
- Check DNS, TLS, and network reachability from the OxiCloud container or host

### Invalid redirect URI

- Make the configured callback match exactly on scheme, host, port, and path
- Check for `http` versus `https` mismatches

### Auto-provisioning problems

- Enable `OXICLOUD_OIDC_AUTO_PROVISION=true` if first-login account creation is expected
- Make sure `openid` is included in the configured scopes

## Related Pages

- [OIDC / SSO](/config/oidc)
- [Admin Settings](/config/admin-settings)