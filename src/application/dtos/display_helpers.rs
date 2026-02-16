/// Shared display helpers for DTOs.
///
/// These functions centralise the mime→icon / mime→category / size→human-string
/// logic so that every API response carries pre-computed display fields and the
/// frontend does **not** need to duplicate these mappings.

/// Returns the FontAwesome icon class for a given MIME type.
///
/// Examples: `"fas fa-file-image"`, `"fas fa-file-pdf"`, `"fas fa-file"` (default).
pub fn mime_to_icon_class(mime: &str) -> &'static str {
    if mime.starts_with("image/") {
        "fas fa-file-image"
    } else if mime.starts_with("text/") {
        "fas fa-file-alt"
    } else if mime.starts_with("video/") {
        "fas fa-file-video"
    } else if mime.starts_with("audio/") {
        "fas fa-file-audio"
    } else if mime == "application/pdf" {
        "fas fa-file-pdf"
    } else {
        "fas fa-file"
    }
}

/// Returns the CSS class used to colour/style the icon container.
///
/// Examples: `"image-icon"`, `"pdf-icon"`, `""` (default).
pub fn mime_to_icon_special_class(mime: &str) -> &'static str {
    if mime.starts_with("image/") {
        "image-icon"
    } else if mime.starts_with("text/") {
        "text-icon"
    } else if mime.starts_with("video/") {
        "video-icon"
    } else if mime.starts_with("audio/") {
        "audio-icon"
    } else if mime == "application/pdf" {
        "pdf-icon"
    } else {
        ""
    }
}

/// Returns a human-readable category label for a MIME type.
///
/// Examples: `"Image"`, `"Text"`, `"Document"` (default).
pub fn mime_to_category(mime: &str) -> &'static str {
    if mime.starts_with("image/") {
        "Image"
    } else if mime.starts_with("text/") {
        "Text"
    } else if mime.starts_with("video/") {
        "Video"
    } else if mime.starts_with("audio/") {
        "Audio"
    } else if mime == "application/pdf" {
        "PDF"
    } else {
        "Document"
    }
}

/// Formats a byte count into a human-readable string (1024-based).
///
/// Matches the JavaScript `formatFileSize()` output exactly so the frontend
/// does not need its own per-file formatting.
///
/// Examples: `"0 Bytes"`, `"1.5 KB"`, `"3.27 MB"`.
pub fn format_file_size(bytes: u64) -> String {
    if bytes == 0 {
        return "0 Bytes".to_string();
    }

    const K: f64 = 1024.0;
    const SIZES: [&str; 5] = ["Bytes", "KB", "MB", "GB", "TB"];

    let i = ((bytes as f64).ln() / K.ln()).floor() as usize;
    let i = i.min(SIZES.len() - 1);

    let value = bytes as f64 / K.powi(i as i32);

    // Two decimal places, then strip trailing zeros (matches JS parseFloat behaviour)
    let formatted = format!("{:.2}", value);
    let formatted = formatted
        .trim_end_matches('0')
        .trim_end_matches('.');

    format!("{} {}", formatted, SIZES[i])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_file_size() {
        assert_eq!(format_file_size(0), "0 Bytes");
        assert_eq!(format_file_size(500), "500 Bytes");
        assert_eq!(format_file_size(1024), "1 KB");
        assert_eq!(format_file_size(1536), "1.5 KB");
        assert_eq!(format_file_size(1_048_576), "1 MB");
        assert_eq!(format_file_size(3_423_744), "3.27 MB");
        assert_eq!(format_file_size(1_073_741_824), "1 GB");
    }

    #[test]
    fn test_mime_to_icon_class() {
        assert_eq!(mime_to_icon_class("image/png"), "fas fa-file-image");
        assert_eq!(mime_to_icon_class("text/plain"), "fas fa-file-alt");
        assert_eq!(mime_to_icon_class("video/mp4"), "fas fa-file-video");
        assert_eq!(mime_to_icon_class("audio/mpeg"), "fas fa-file-audio");
        assert_eq!(mime_to_icon_class("application/pdf"), "fas fa-file-pdf");
        assert_eq!(mime_to_icon_class("application/octet-stream"), "fas fa-file");
    }

    #[test]
    fn test_mime_to_category() {
        assert_eq!(mime_to_category("image/jpeg"), "Image");
        assert_eq!(mime_to_category("text/html"), "Text");
        assert_eq!(mime_to_category("video/webm"), "Video");
        assert_eq!(mime_to_category("audio/ogg"), "Audio");
        assert_eq!(mime_to_category("application/pdf"), "PDF");
        assert_eq!(mime_to_category("application/zip"), "Document");
    }
}
