//! CSRF double-submit cookie middleware.
//!
//! State-changing requests (`POST`, `PUT`, `DELETE`, `PATCH`) that were
//! authenticated via an HttpOnly cookie (i.e. browser sessions) **must**
//! include an `X-CSRF-Token` header whose value matches the `oxicloud_csrf`
//! cookie.  Requests authenticated via `Bearer` or `Basic` headers are
//! exempt because they are not vulnerable to CSRF — the browser never
//! attaches those automatically.
//!
//! Safe methods (`GET`, `HEAD`, `OPTIONS`) are always allowed through.

use axum::{
    extract::Request,
    http::{Method, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::interfaces::api::cookie_auth;
use crate::interfaces::middleware::auth::CookieAuthenticated;

/// Methods considered safe (no side-effects) — CSRF check is skipped.
const SAFE_METHODS: [Method; 3] = [Method::GET, Method::HEAD, Method::OPTIONS];

/// Middleware that enforces CSRF protection for cookie-authenticated browser
/// sessions using the **double-submit cookie** pattern.
///
/// Must be applied **after** `auth_middleware` so that the
/// `CookieAuthenticated` marker is available in extensions.
pub async fn csrf_middleware(request: Request, next: Next) -> Result<Response, Response> {
    // Safe methods never need CSRF validation.
    if SAFE_METHODS.contains(request.method()) {
        return Ok(next.run(request).await);
    }

    // Only enforce for cookie-authenticated sessions.
    let is_cookie_auth = request.extensions().get::<CookieAuthenticated>().is_some();
    if !is_cookie_auth {
        return Ok(next.run(request).await);
    }

    // Extract the CSRF token from the cookie.
    let cookie_token = cookie_auth::extract_cookie_value(
        request.headers(),
        cookie_auth::CSRF_COOKIE,
    );

    // Extract the CSRF token from the request header.
    let header_token = request
        .headers()
        .get(cookie_auth::CSRF_HEADER)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    match (cookie_token, header_token) {
        (Some(c), Some(h)) if !c.is_empty() && c == h => {
            // Tokens match — allow the request through.
            Ok(next.run(request).await)
        }
        _ => {
            tracing::warn!(
                method = %request.method(),
                uri = %request.uri(),
                "CSRF validation failed: missing or mismatched token"
            );
            Err((
                StatusCode::FORBIDDEN,
                axum::Json(serde_json::json!({
                    "error": "CSRF token missing or invalid"
                })),
            )
                .into_response())
        }
    }
}
