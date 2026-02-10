//! OpenID Connect (OIDC) service implementation.
//!
//! Handles OIDC discovery, authorization URL generation, code exchange,
//! ID token validation (RS256 via JWKS), and UserInfo fetching.
//! Compatible with Authentik, Keycloak, and any standard OIDC provider.

use std::sync::RwLock;
use async_trait::async_trait;
use serde::Deserialize;

use crate::application::ports::auth_ports::{OidcServicePort, OidcTokenSet, OidcIdClaims};
use crate::common::config::OidcConfig;
use crate::common::errors::{DomainError, ErrorKind};

// ============================================================================
// OIDC Discovery Document
// ============================================================================

#[derive(Debug, Clone, Deserialize)]
struct OidcDiscovery {
    issuer: String,
    authorization_endpoint: String,
    token_endpoint: String,
    userinfo_endpoint: Option<String>,
    jwks_uri: String,
}

// ============================================================================
// JWKS structures for RS256 validation
// ============================================================================

#[derive(Debug, Clone, Deserialize)]
struct JwksDocument {
    keys: Vec<JwkKey>,
}

#[derive(Debug, Clone, Deserialize)]
struct JwkKey {
    kty: String,
    #[serde(rename = "use")]
    key_use: Option<String>,
    kid: Option<String>,
    alg: Option<String>,
    n: Option<String>,      // RSA modulus (base64url)
    e: Option<String>,      // RSA exponent (base64url)
}

// ============================================================================
// Token exchange response
// ============================================================================

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    id_token: Option<String>,
    refresh_token: Option<String>,
    #[allow(dead_code)]
    token_type: Option<String>,
    #[allow(dead_code)]
    expires_in: Option<i64>,
}

// ============================================================================
// ID token claims (standard OIDC)
// ============================================================================

#[derive(Debug, Deserialize)]
struct IdTokenClaims {
    sub: String,
    email: Option<String>,
    preferred_username: Option<String>,
    name: Option<String>,
    groups: Option<Vec<String>>,
    nonce: Option<String>,
    // Standard JWT fields
    #[allow(dead_code)]
    iss: Option<String>,
    #[allow(dead_code)]
    aud: Option<serde_json::Value>,
    #[allow(dead_code)]
    exp: Option<i64>,
    #[allow(dead_code)]
    iat: Option<i64>,
}

// ============================================================================
// UserInfo response
// ============================================================================

#[derive(Debug, Deserialize)]
struct UserInfoResponse {
    sub: String,
    email: Option<String>,
    preferred_username: Option<String>,
    name: Option<String>,
    groups: Option<Vec<String>>,
}

// ============================================================================
// OIDC Service
// ============================================================================

pub struct OidcService {
    config: OidcConfig,
    http_client: reqwest::Client,
    /// Cached discovery document
    discovery: RwLock<Option<OidcDiscovery>>,
    /// Cached JWKS
    jwks: RwLock<Option<JwksDocument>>,
}

impl OidcService {
    pub fn new(config: OidcConfig) -> Self {
        let http_client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .expect("Failed to build HTTP client for OIDC");

        Self {
            config,
            http_client,
            discovery: RwLock::new(None),
            jwks: RwLock::new(None),
        }
    }

    /// Fetch and cache the OIDC discovery document
    async fn get_discovery(&self) -> Result<OidcDiscovery, DomainError> {
        // Check cache first
        {
            let cache = self.discovery.read().map_err(|_| DomainError::new(
                ErrorKind::InternalError, "OIDC", "Lock poisoned",
            ))?;
            if let Some(ref disc) = *cache {
                return Ok(disc.clone());
            }
        }

        // Fetch discovery document
        let issuer = self.config.issuer_url.trim_end_matches('/');
        let discovery_url = format!("{}/.well-known/openid-configuration", issuer);
        
        tracing::info!("Fetching OIDC discovery from: {}", discovery_url);

        let resp = self.http_client.get(&discovery_url)
            .send()
            .await
            .map_err(|e| DomainError::new(
                ErrorKind::InternalError, "OIDC",
                format!("Failed to fetch OIDC discovery: {}", e),
            ))?;

        if !resp.status().is_success() {
            return Err(DomainError::new(
                ErrorKind::InternalError, "OIDC",
                format!("OIDC discovery returned status {}", resp.status()),
            ));
        }

        let discovery: OidcDiscovery = resp.json().await.map_err(|e| DomainError::new(
            ErrorKind::InternalError, "OIDC",
            format!("Failed to parse OIDC discovery: {}", e),
        ))?;

        // Cache it
        {
            let mut cache = self.discovery.write().map_err(|_| DomainError::new(
                ErrorKind::InternalError, "OIDC", "Lock poisoned",
            ))?;
            *cache = Some(discovery.clone());
        }

        Ok(discovery)
    }

    /// Fetch and cache JWKS document for ID token validation
    async fn get_jwks(&self) -> Result<JwksDocument, DomainError> {
        // Check cache first
        {
            let cache = self.jwks.read().map_err(|_| DomainError::new(
                ErrorKind::InternalError, "OIDC", "Lock poisoned",
            ))?;
            if let Some(ref jwks) = *cache {
                return Ok(jwks.clone());
            }
        }

        let discovery = self.get_discovery().await?;

        tracing::debug!("Fetching JWKS from: {}", discovery.jwks_uri);

        let resp = self.http_client.get(&discovery.jwks_uri)
            .send()
            .await
            .map_err(|e| DomainError::new(
                ErrorKind::InternalError, "OIDC",
                format!("Failed to fetch JWKS: {}", e),
            ))?;

        let jwks: JwksDocument = resp.json().await.map_err(|e| DomainError::new(
            ErrorKind::InternalError, "OIDC",
            format!("Failed to parse JWKS: {}", e),
        ))?;

        // Cache it
        {
            let mut cache = self.jwks.write().map_err(|_| DomainError::new(
                ErrorKind::InternalError, "OIDC", "Lock poisoned",
            ))?;
            *cache = Some(jwks.clone());
        }

        Ok(jwks)
    }

    /// Find the right RSA key from JWKS by kid header
    fn find_rsa_key<'a>(jwks: &'a JwksDocument, kid: Option<&str>) -> Option<&'a JwkKey> {
        jwks.keys.iter().find(|k| {
            k.kty == "RSA"
                && k.key_use.as_deref() != Some("enc") // exclude encryption keys
                && (kid.is_none() || k.kid.as_deref() == kid)
        })
    }

    /// Extract the `kid` from a JWT header without full validation
    fn extract_jwt_kid(token: &str) -> Option<String> {
        let parts: Vec<&str> = token.splitn(3, '.').collect();
        if parts.len() < 2 {
            return None;
        }
        use base64::Engine;
        let engine = base64::engine::general_purpose::URL_SAFE_NO_PAD;
        let header_bytes = engine.decode(parts[0]).ok()?;
        let header: serde_json::Value = serde_json::from_slice(&header_bytes).ok()?;
        header.get("kid").and_then(|v| v.as_str()).map(|s| s.to_string())
    }
}

#[async_trait]
impl OidcServicePort for OidcService {
    fn get_authorize_url(&self, state: &str, nonce: &str, pkce_challenge: &str) -> Result<String, DomainError> {
        // We need the authorization_endpoint. If not cached, we'll construct it from issuer.
        // In practice, the discovery should be pre-fetched during startup.
        let auth_endpoint = {
            let cache = self.discovery.read().map_err(|_| DomainError::new(
                ErrorKind::InternalError, "OIDC", "Lock poisoned",
            ))?;
            match &*cache {
                Some(disc) => disc.authorization_endpoint.clone(),
                None => {
                    // Fallback: construct typical endpoint
                    let issuer = self.config.issuer_url.trim_end_matches('/');
                    format!("{}/authorize", issuer)
                }
            }
        };

        let scopes = self.config.scopes.replace(',', " ");
        let url = format!(
            "{}?response_type=code&client_id={}&redirect_uri={}&scope={}&state={}&nonce={}&code_challenge={}&code_challenge_method=S256",
            auth_endpoint,
            urlencoding::encode(&self.config.client_id),
            urlencoding::encode(&self.config.redirect_uri),
            urlencoding::encode(&scopes),
            urlencoding::encode(state),
            urlencoding::encode(nonce),
            urlencoding::encode(pkce_challenge),
        );

        Ok(url)
    }

    async fn exchange_code(&self, code: &str, pkce_verifier: &str) -> Result<OidcTokenSet, DomainError> {
        let discovery = self.get_discovery().await?;

        tracing::debug!("Exchanging authorization code at: {}", discovery.token_endpoint);

        let resp = self.http_client.post(&discovery.token_endpoint)
            .form(&[
                ("grant_type", "authorization_code"),
                ("code", code),
                ("redirect_uri", &self.config.redirect_uri),
                ("client_id", &self.config.client_id),
                ("client_secret", &self.config.client_secret),
                ("code_verifier", pkce_verifier),
            ])
            .send()
            .await
            .map_err(|e| DomainError::new(
                ErrorKind::InternalError, "OIDC",
                format!("Token exchange failed: {}", e),
            ))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            tracing::error!("OIDC token exchange error: status={}, body={}", status, body);
            return Err(DomainError::new(
                ErrorKind::AccessDenied, "OIDC",
                format!("Token exchange failed with status {}", status),
            ));
        }

        let token_resp: TokenResponse = resp.json().await.map_err(|e| DomainError::new(
            ErrorKind::InternalError, "OIDC",
            format!("Failed to parse token response: {}", e),
        ))?;

        let id_token = token_resp.id_token.ok_or_else(|| DomainError::new(
            ErrorKind::InternalError, "OIDC",
            "No id_token in token response",
        ))?;

        Ok(OidcTokenSet {
            access_token: token_resp.access_token,
            id_token,
            refresh_token: token_resp.refresh_token,
        })
    }

    async fn validate_id_token(&self, id_token: &str, expected_nonce: Option<&str>) -> Result<OidcIdClaims, DomainError> {
        let jwks = self.get_jwks().await?;
        let discovery = self.get_discovery().await?;

        // Extract kid from JWT header
        let kid = Self::extract_jwt_kid(id_token);

        // Find the matching RSA key
        let jwk = Self::find_rsa_key(&jwks, kid.as_deref()).ok_or_else(|| DomainError::new(
            ErrorKind::AccessDenied, "OIDC",
            "No suitable RSA key found in JWKS for ID token validation",
        ))?;

        let n = jwk.n.as_ref().ok_or_else(|| DomainError::new(
            ErrorKind::InternalError, "OIDC", "JWKS key missing 'n' component",
        ))?;
        let e = jwk.e.as_ref().ok_or_else(|| DomainError::new(
            ErrorKind::InternalError, "OIDC", "JWKS key missing 'e' component",
        ))?;

        // Build decoding key from RSA components
        let decoding_key = jsonwebtoken::DecodingKey::from_rsa_components(n, e)
            .map_err(|err| DomainError::new(
                ErrorKind::InternalError, "OIDC",
                format!("Failed to build RSA decoding key: {}", err),
            ))?;

        // Determine algorithm from JWKS (default RS256)
        let alg = match jwk.alg.as_deref() {
            Some("RS384") => jsonwebtoken::Algorithm::RS384,
            Some("RS512") => jsonwebtoken::Algorithm::RS512,
            _ => jsonwebtoken::Algorithm::RS256,
        };

        // Build validation: check expiry and issuer
        let mut validation = jsonwebtoken::Validation::new(alg);
        validation.set_issuer(&[&discovery.issuer]);
        validation.set_audience(&[&self.config.client_id]);

        let token_data = jsonwebtoken::decode::<IdTokenClaims>(
            id_token,
            &decoding_key,
            &validation,
        ).map_err(|e| {
            tracing::warn!("OIDC ID token validation failed: {}", e);
            DomainError::new(
                ErrorKind::AccessDenied, "OIDC",
                format!("ID token validation failed: {}", e),
            )
        })?;

        let claims = token_data.claims;

        // Verify nonce to prevent token replay attacks
        if let Some(expected) = expected_nonce {
            match &claims.nonce {
                Some(actual) if actual == expected => { /* OK */ }
                Some(actual) => {
                    tracing::warn!("OIDC nonce mismatch: expected={}, got={}", expected, actual);
                    return Err(DomainError::new(
                        ErrorKind::AccessDenied, "OIDC",
                        "ID token nonce mismatch — possible replay attack",
                    ));
                }
                None => {
                    tracing::warn!("OIDC nonce missing from ID token (expected={})", expected);
                    // Some providers don't include nonce; log warning but don't fail
                }
            }
        }

        Ok(OidcIdClaims {
            sub: claims.sub,
            email: claims.email,
            preferred_username: claims.preferred_username,
            name: claims.name,
            groups: claims.groups.unwrap_or_default(),
        })
    }

    async fn fetch_user_info(&self, access_token: &str) -> Result<OidcIdClaims, DomainError> {
        let discovery = self.get_discovery().await?;

        let userinfo_url = discovery.userinfo_endpoint.ok_or_else(|| DomainError::new(
            ErrorKind::InternalError, "OIDC",
            "No userinfo_endpoint in OIDC discovery",
        ))?;

        let resp = self.http_client.get(&userinfo_url)
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await
            .map_err(|e| DomainError::new(
                ErrorKind::InternalError, "OIDC",
                format!("UserInfo request failed: {}", e),
            ))?;

        if !resp.status().is_success() {
            return Err(DomainError::new(
                ErrorKind::AccessDenied, "OIDC",
                format!("UserInfo returned status {}", resp.status()),
            ));
        }

        let info: UserInfoResponse = resp.json().await.map_err(|e| DomainError::new(
            ErrorKind::InternalError, "OIDC",
            format!("Failed to parse UserInfo: {}", e),
        ))?;

        Ok(OidcIdClaims {
            sub: info.sub,
            email: info.email,
            preferred_username: info.preferred_username,
            name: info.name,
            groups: info.groups.unwrap_or_default(),
        })
    }

    fn provider_name(&self) -> &str {
        &self.config.provider_name
    }
}

// We need urlencoding — let's use a minimal inline implementation
mod urlencoding {
    pub fn encode(input: &str) -> String {
        let mut result = String::with_capacity(input.len() * 3);
        for byte in input.bytes() {
            match byte {
                b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                    result.push(byte as char);
                }
                _ => {
                    result.push('%');
                    result.push_str(&format!("{:02X}", byte));
                }
            }
        }
        result
    }
}
