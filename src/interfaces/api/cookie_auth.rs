//! HttpOnly cookie helpers for secure token transport.
//!
//! Tokens are set as `HttpOnly; SameSite=Lax` cookies so that
//! browser-based JavaScript cannot read them (mitigates XSS token theft).
//! The `Secure` flag is controlled by the `OXICLOUD_COOKIE_SECURE` env var
//! (default: auto-detect from `OXICLOUD_BASE_URL`).
//!
//! A companion **non-HttpOnly** CSRF cookie (`oxicloud_csrf`) is set
//! alongside the auth cookies.  The frontend must read it and echo its
//! value back as `X-CSRF-Token` on every state-changing request.
//! A middleware (`csrf_middleware`) validates the match.
//!
//! DAV clients continue to use `Authorization: Basic` with app passwords
//! and are completely unaffected by this mechanism.

use axum::http::header::SET_COOKIE;
use axum::http::{HeaderMap, HeaderValue};

/// Cookie name for the JWT access token.
pub const ACCESS_COOKIE: &str = "oxicloud_access";
/// Cookie name for the opaque refresh token.
pub const REFRESH_COOKIE: &str = "oxicloud_refresh";
/// Cookie name for the CSRF double-submit token (readable by JS).
pub const CSRF_COOKIE: &str = "oxicloud_csrf";
/// Header the frontend must send with the CSRF token value.
pub const CSRF_HEADER: &str = "x-csrf-token";

/// Whether the `Secure` flag should be set on cookies.
///
/// Resolution order:
/// 1. `OXICLOUD_COOKIE_SECURE=true|false` — explicit override.
/// 2. `OXICLOUD_BASE_URL` starts with `https` → `true`.
/// 3. `OXICLOUD_BASE_URL` starts with `http` → `false`.
/// 4. **Default: `false`** for compatibility with HTTP deployments
///    (Docker, local development). Set `OXICLOUD_COOKIE_SECURE=true`
///    explicitly for production HTTPS environments.
pub fn is_cookie_secure() -> bool {
    cookie_secure()
}

fn cookie_secure() -> bool {
    if let Ok(v) = std::env::var("OXICLOUD_COOKIE_SECURE") {
        let secure = v == "true" || v == "1";
        if !secure {
            tracing::warn!(
                "OXICLOUD_COOKIE_SECURE is explicitly disabled — \
                 cookies will be sent over plain HTTP. \
                 Do NOT use this in production."
            );
        }
        return secure;
    }
    // Auto-detect from base URL, defaulting to insecure for compatibility
    match std::env::var("OXICLOUD_BASE_URL") {
        Ok(url) if url.starts_with("https") => true,
        Ok(url) if url.starts_with("http://") => {
            tracing::info!(
                "OXICLOUD_BASE_URL is HTTP — cookie Secure flag is OFF. \
                 Set OXICLOUD_COOKIE_SECURE=true to override if your proxy terminates TLS."
            );
            false
        }
        _ => {
            // Default to false for compatibility with HTTP deployments
            tracing::info!(
                "OXICLOUD_BASE_URL not set — defaulting to non-secure cookies \
                 for HTTP compatibility. Set OXICLOUD_COOKIE_SECURE=true for HTTPS deployments."
            );
            false
        }
    }
}

/// Build a `Set-Cookie` header value.
fn build_cookie(name: &str, value: &str, path: &str, max_age_secs: i64) -> String {
    let secure = if cookie_secure() { "; Secure" } else { "" };
    format!("{name}={value}; HttpOnly; SameSite=Lax; Path={path}; Max-Age={max_age_secs}{secure}",)
}

/// Append `Set-Cookie` headers for both access and refresh tokens.
///
/// The access cookie covers all paths (`/`) because the API lives under
/// `/api`, CalDAV under `/caldav`, WebDAV under `/webdav`, etc.
///
/// The refresh cookie is restricted to `/api/auth` so it is only sent
/// when the client explicitly calls the refresh or logout endpoints.
pub fn append_auth_cookies(
    headers: &mut HeaderMap,
    access_token: &str,
    refresh_token: &str,
    access_expiry_secs: i64,
    refresh_expiry_secs: i64,
) {
    if let Ok(val) = HeaderValue::from_str(&build_cookie(
        ACCESS_COOKIE,
        access_token,
        "/",
        access_expiry_secs,
    )) {
        headers.append(SET_COOKIE, val);
    }
    if let Ok(val) = HeaderValue::from_str(&build_cookie(
        REFRESH_COOKIE,
        refresh_token,
        "/api/auth",
        refresh_expiry_secs,
    )) {
        headers.append(SET_COOKIE, val);
    }
}

/// Append `Set-Cookie` headers that immediately expire both auth cookies,
/// effectively logging the user out on the browser side.
pub fn append_clear_cookies(headers: &mut HeaderMap) {
    for (name, path) in [(ACCESS_COOKIE, "/"), (REFRESH_COOKIE, "/api/auth")] {
        let secure = if cookie_secure() { "; Secure" } else { "" };
        let val = format!("{name}=; HttpOnly; SameSite=Lax; Path={path}; Max-Age=0{secure}",);
        if let Ok(hv) = HeaderValue::from_str(&val) {
            headers.append(SET_COOKIE, hv);
        }
    }
}

/// Extract a named cookie value from the `Cookie` request header.
pub fn extract_cookie_value(headers: &HeaderMap, name: &str) -> Option<String> {
    let cookie_header = headers.get(axum::http::header::COOKIE)?;
    let cookie_str = cookie_header.to_str().ok()?;

    for pair in cookie_str.split(';') {
        let pair = pair.trim();
        if let Some(val) = pair.strip_prefix(name) {
            let val = val.strip_prefix('=')?;
            if !val.is_empty() {
                return Some(val.to_string());
            }
        }
    }
    None
}

// ────────────────────────────────────────────────────────────
// CSRF double-submit cookie helpers
// ────────────────────────────────────────────────────────────

/// Generate a cryptographically random CSRF token (128-bit UUIDv4, hex-like).
pub fn generate_csrf_token() -> String {
    uuid::Uuid::new_v4().to_string()
}

/// Build a **non-HttpOnly** CSRF cookie so that frontend JS can read it
/// via `document.cookie` and echo it back in the `X-CSRF-Token` header.
fn build_csrf_cookie(value: &str, max_age_secs: i64) -> String {
    let secure = if cookie_secure() { "; Secure" } else { "" };
    format!("{CSRF_COOKIE}={value}; SameSite=Lax; Path=/; Max-Age={max_age_secs}{secure}",)
}

/// Append a CSRF double-submit cookie alongside the auth cookies.
/// Should be called in every endpoint that also sets auth cookies.
pub fn append_csrf_cookie(headers: &mut HeaderMap, access_expiry_secs: i64) {
    let token = generate_csrf_token();
    if let Ok(val) = HeaderValue::from_str(&build_csrf_cookie(&token, access_expiry_secs)) {
        headers.append(SET_COOKIE, val);
    }
}

/// Clear the CSRF cookie (on logout).
pub fn append_clear_csrf_cookie(headers: &mut HeaderMap) {
    let secure = if cookie_secure() { "; Secure" } else { "" };
    let val = format!("{CSRF_COOKIE}=; SameSite=Lax; Path=/; Max-Age=0{secure}",);
    if let Ok(hv) = HeaderValue::from_str(&val) {
        headers.append(SET_COOKIE, hv);
    }
}
