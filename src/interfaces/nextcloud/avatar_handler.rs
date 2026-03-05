use axum::{
    extract::{Path, State},
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use std::sync::Arc;

use crate::common::di::AppState;

/// GET /index.php/avatar/{user}/{size}
///
/// Returns an SVG avatar with the user's initials on a colored background.
pub async fn handle_avatar(
    State(_state): State<Arc<AppState>>,
    Path((username, size)): Path<(String, u32)>,
) -> Response {
    let size = size.clamp(16, 1024);
    let initials = extract_initials(&username);
    let color = pick_color(&username);
    let font_size = (size as f32 * 0.45) as u32;

    let safe_initials = xml_escape(&initials);

    let svg = format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="{s}" height="{s}" viewBox="0 0 {s} {s}">
  <rect width="{s}" height="{s}" rx="{r}" fill="{c}"/>
  <text x="50%" y="50%" dy="0.36em" fill="#fff" font-family="-apple-system,BlinkMacSystemFont,sans-serif" font-size="{fs}" font-weight="600" text-anchor="middle">{i}</text>
</svg>"##,
        s = size,
        r = size / 2,
        c = color,
        fs = font_size,
        i = safe_initials,
    );

    (
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, "image/svg+xml"),
            (header::CACHE_CONTROL, "public, max-age=86400, immutable"),
            (
                header::CONTENT_SECURITY_POLICY,
                "default-src 'none'; style-src 'unsafe-inline'",
            ),
        ],
        svg,
    )
        .into_response()
}

/// Escape XML special characters to prevent XSS in SVG output.
fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn extract_initials(username: &str) -> String {
    let parts: Vec<&str> = username.split_whitespace().collect();
    match parts.len() {
        0 => "?".to_string(),
        1 => parts[0]
            .chars()
            .next()
            .unwrap_or('?')
            .to_uppercase()
            .to_string(),
        _ => {
            let first = parts[0].chars().next().unwrap_or('?');
            let last = parts[parts.len() - 1].chars().next().unwrap_or('?');
            format!("{}{}", first.to_uppercase(), last.to_uppercase())
        }
    }
}

fn pick_color(username: &str) -> &'static str {
    const PALETTE: [&str; 10] = [
        "#0082c9", "#e9322d", "#2d8a0f", "#c37200", "#6c2d9e", "#007a87", "#b02e7c", "#465a64",
        "#a65d00", "#3b5998",
    ];
    let hash: u32 = username
        .bytes()
        .fold(0u32, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u32));
    PALETTE[(hash as usize) % PALETTE.len()]
}
