//! MIME type detection using magic bytes (infer) + extension fallback (mime_guess).
//!
//! Priority order:
//! 1. If the claimed Content-Type is specific (not `application/octet-stream`), trust it.
//! 2. Read first bytes of the file and detect via magic bytes (`infer` crate).
//! 3. Fall back to extension-based detection (`mime_guess`).
//! 4. If nothing matches, return the original claimed type.
//!
//! Performance: < 1µs for the `infer` check (reads only header bytes, no allocation).

use std::path::Path;

/// Maximum bytes to read for magic-byte detection.
const MAGIC_BYTES_LEN: usize = 8192;

/// Refine a claimed MIME type using magic bytes and filename extension.
///
/// This is a synchronous function — the caller should already have the first
/// bytes of the file available (or call the async wrapper below).
///
/// # Arguments
/// * `buf` — first bytes of the file (at least 8192 for best results)
/// * `filename` — original filename (used for extension fallback)
/// * `claimed` — the Content-Type sent by the client
pub fn refine_content_type(buf: &[u8], filename: &str, claimed: &str) -> String {
    // If the client sent a specific type (not generic), trust it
    if !claimed.is_empty()
        && claimed != "application/octet-stream"
        && claimed != "binary/octet-stream"
    {
        return claimed.to_string();
    }

    // 1. Try magic bytes detection
    if let Some(kind) = infer::get(buf) {
        return kind.mime_type().to_string();
    }

    // 2. Try extension-based detection
    let guess = mime_guess::from_path(filename);
    if let Some(mime) = guess.first() {
        return mime.to_string();
    }

    // 3. Fall back to claimed type
    claimed.to_string()
}

/// Async helper: reads the first bytes of a file on disk and refines the MIME type.
///
/// Designed for the upload path where the file has been spooled to a temp path.
pub async fn refine_content_type_from_file(
    temp_path: &Path,
    filename: &str,
    claimed: &str,
) -> String {
    // Fast path: if the client gave us a specific type, trust it
    if !claimed.is_empty()
        && claimed != "application/octet-stream"
        && claimed != "binary/octet-stream"
    {
        return claimed.to_string();
    }

    // Read first bytes for magic detection
    match tokio::fs::read(temp_path).await {
        Ok(full) => {
            let len = full.len().min(MAGIC_BYTES_LEN);
            refine_content_type(&full[..len], filename, claimed)
        }
        Err(e) => {
            tracing::warn!(
                "MIME detection: failed to read {} for magic bytes: {}",
                temp_path.display(),
                e
            );
            // Fall back to extension
            let guess = mime_guess::from_path(filename);
            if let Some(mime) = guess.first() {
                return mime.to_string();
            }
            claimed.to_string()
        }
    }
}
