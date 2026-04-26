//! Custom [`MakeSpan`] that records the client IP in every request span.
//!
//! IP resolution delegates to [`super::trusted_proxy::client_ip`]:
//! 1. TCP peer in `OXICLOUD_TRUST_PROXY_CIDR` → `X-Forwarded-For` / `X-Real-Ip`
//! 2. Otherwise → raw TCP peer address (with port, e.g. `127.0.0.1:12345`)

use axum::http::Request;
use tower_http::trace::MakeSpan;
use tracing::Span;

/// Implements [`MakeSpan`] so that every HTTP request span carries a
/// `client_ip` field visible in every log line produced inside that span.
#[derive(Clone, Debug, Default)]
pub struct ClientIpMakeSpan;

impl<B> MakeSpan<B> for ClientIpMakeSpan {
    fn make_span(&mut self, request: &Request<B>) -> Span {
        let ip = super::trusted_proxy::client_ip(request, true);
        tracing::info_span!(
            "req",
            client_ip = %ip,
            user_id = tracing::field::Empty,
        )
    }
}
