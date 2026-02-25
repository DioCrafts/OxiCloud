// Shared display helpers for DTOs.
//
// These functions centralise the mime→icon / mime→category / size→human-string
// logic so that every API response carries pre-computed display fields and the
// frontend does **not** need to duplicate these mappings.
//
// The approach is: try MIME first (specific matches beat prefix matches),
// then fall back to the file extension when the MIME is generic
// (`application/octet-stream` or empty).

// ─── Private: extract lowercase extension from a filename ────────────

fn ext_of(name: &str) -> Option<&str> {
    let name = name.rsplit('/').next().unwrap_or(name); // strip path
    let after_dot = name.rsplit('.').next()?;
    // Reject the whole name (no dot) or empty after dot
    if after_dot.len() == name.len() || after_dot.is_empty() {
        return None;
    }
    Some(after_dot)
}

// ─── Icon class (FontAwesome) ────────────────────────────────────────

/// Returns the FontAwesome icon class for a file, considering both MIME
/// and filename extension as fallback.
///
/// Use this instead of the old `mime_to_icon_class` whenever the filename
/// is available.
pub fn icon_class_for(name: &str, mime: &str) -> &'static str {
    // 1. Try specific MIME matches first
    match mime {
        "application/pdf" => return "fas fa-file-pdf",
        // MS Office & OpenDocument – Word
        "application/msword"
        | "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
        | "application/vnd.oasis.opendocument.text" => return "fas fa-file-word",
        // Excel
        "application/vnd.ms-excel"
        | "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
        | "application/vnd.oasis.opendocument.spreadsheet"
        | "text/csv" => return "fas fa-file-excel",
        // PowerPoint
        "application/vnd.ms-powerpoint"
        | "application/vnd.openxmlformats-officedocument.presentationml.presentation"
        | "application/vnd.oasis.opendocument.presentation" => return "fas fa-file-powerpoint",
        // Archives
        "application/zip"
        | "application/x-rar-compressed"
        | "application/vnd.rar"
        | "application/x-7z-compressed"
        | "application/gzip"
        | "application/x-gzip"
        | "application/x-tar"
        | "application/x-bzip2"
        | "application/x-xz"
        | "application/x-compress" => return "fas fa-file-archive",
        // JSON / JavaScript / code transported as application/*
        "application/json"
        | "application/ld+json"
        | "application/javascript"
        | "application/typescript"
        | "application/x-httpd-php"
        | "application/xml"
        | "application/xhtml+xml"
        | "application/sql"
        | "application/x-yaml"
        | "application/toml"
        | "application/x-sh"
        | "application/x-shellscript"
        | "application/x-csh" => return "fas fa-file-code",
        // Installers / disk images
        "application/x-apple-diskimage"
        | "application/x-ms-dos-executable"
        | "application/x-msdownload"
        | "application/x-msi"
        | "application/vnd.debian.binary-package"
        | "application/x-rpm"
        | "application/vnd.appimage" => return "fas fa-hdd",
        _ => {}
    }

    // 2. MIME prefix matches
    if mime.starts_with("image/") {
        return "fas fa-file-image";
    } else if mime.starts_with("video/") {
        return "fas fa-file-video";
    } else if mime.starts_with("audio/") {
        return "fas fa-file-audio";
    } else if mime.starts_with("text/x-script")
        || mime.starts_with("text/x-python")
        || mime.starts_with("text/x-java")
        || mime.starts_with("text/x-c")
        || mime.starts_with("text/x-rust")
        || mime.starts_with("text/x-go")
        || mime.starts_with("text/x-ruby")
        || mime.starts_with("text/x-shellscript")
        || mime.starts_with("text/x-php")
        || mime.contains("javascript")
        || mime.contains("typescript")
    {
        return "fas fa-file-code";
    } else if mime.starts_with("text/") {
        return "fas fa-file-alt";
    }

    // 3. Extension-based fallback (for application/octet-stream, empty, etc.)
    if let Some(ext) = ext_of(name) {
        return match ext.to_ascii_lowercase().as_str() {
            "pdf" => "fas fa-file-pdf",
            "doc" | "docx" | "odt" | "rtf" => "fas fa-file-word",
            "xls" | "xlsx" | "ods" | "csv" => "fas fa-file-excel",
            "ppt" | "pptx" | "odp" | "key" => "fas fa-file-powerpoint",
            "jpg" | "jpeg" | "png" | "gif" | "bmp" | "svg" | "webp" | "ico" | "tiff" | "tif"
            | "heic" | "heif" | "avif" => "fas fa-file-image",
            "mp4" | "avi" | "mkv" | "mov" | "wmv" | "flv" | "webm" | "m4v" => "fas fa-file-video",
            "mp3" | "wav" | "ogg" | "flac" | "aac" | "wma" | "m4a" | "opus" => "fas fa-file-audio",
            "zip" | "rar" | "7z" | "tar" | "gz" | "bz2" | "xz" | "zst" | "lz4" => {
                "fas fa-file-archive"
            }
            "exe" | "msi" | "dmg" | "deb" | "rpm" | "appimage" | "pkg" | "snap" | "flatpak" => {
                "fas fa-hdd"
            }
            "js" | "jsx" | "mjs" | "cjs" | "ts" | "tsx" | "py" | "pyw" | "rs" | "go" | "java"
            | "kt" | "kts" | "scala" | "c" | "h" | "cpp" | "hpp" | "cc" | "cxx" | "cs" | "rb"
            | "php" | "swift" | "r" | "lua" | "pl" | "pm" | "html" | "htm" | "css" | "scss"
            | "sass" | "less" | "json" | "xml" | "yaml" | "yml" | "toml" | "ini" | "cfg"
            | "conf" | "sql" | "graphql" | "proto" | "vue" | "svelte" => "fas fa-file-code",
            "sh" | "bash" | "zsh" | "fish" | "ps1" | "bat" | "cmd" => "fas fa-terminal",
            "md" | "markdown" | "rst" | "txt" => "fas fa-file-alt",
            _ => "fas fa-file",
        };
    }

    "fas fa-file"
}

// ─── Icon special class (CSS styling) ────────────────────────────────

/// Returns the CSS class for styling the icon container, considering both
/// MIME and filename extension.
///
/// The returned class maps to CSS rules in `style.css` that set colours,
/// backgrounds and decorative pseudo-elements per file type.
pub fn icon_special_class_for(name: &str, mime: &str) -> &'static str {
    // 1. Specific MIME matches
    match mime {
        "application/pdf" => return "pdf-icon",
        "application/msword"
        | "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
        | "application/vnd.oasis.opendocument.text" => return "doc-icon",
        "application/vnd.ms-excel"
        | "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
        | "application/vnd.oasis.opendocument.spreadsheet"
        | "text/csv" => return "spreadsheet-icon",
        "application/vnd.ms-powerpoint"
        | "application/vnd.openxmlformats-officedocument.presentationml.presentation"
        | "application/vnd.oasis.opendocument.presentation" => return "presentation-icon",
        "application/zip"
        | "application/x-rar-compressed"
        | "application/vnd.rar"
        | "application/x-7z-compressed"
        | "application/gzip"
        | "application/x-gzip"
        | "application/x-tar"
        | "application/x-bzip2"
        | "application/x-xz"
        | "application/x-compress" => return "archive-icon",
        "application/json" | "application/ld+json" => return "code-icon json-icon",
        "application/javascript" => return "code-icon js-icon",
        "application/typescript" => return "code-icon ts-icon",
        "application/xml" | "application/xhtml+xml" => return "code-icon html-icon",
        "application/sql" => return "code-icon sql-icon",
        "application/x-yaml" | "application/toml" => return "code-icon config-icon",
        "application/x-httpd-php" => return "code-icon php-icon",
        "application/x-sh" | "application/x-shellscript" | "application/x-csh" => {
            return "script-icon";
        }
        "application/x-apple-diskimage"
        | "application/x-ms-dos-executable"
        | "application/x-msdownload"
        | "application/x-msi"
        | "application/vnd.debian.binary-package"
        | "application/x-rpm"
        | "application/vnd.appimage" => return "installer-icon",
        _ => {}
    }

    // 2. MIME prefix matches
    if mime.starts_with("image/") {
        return "image-icon";
    } else if mime.starts_with("video/") {
        return "video-icon";
    } else if mime.starts_with("audio/") {
        return "audio-icon";
    } else if mime.starts_with("text/x-python") {
        return "code-icon py-icon";
    } else if mime.starts_with("text/x-rust") {
        return "code-icon rust-icon";
    } else if mime.starts_with("text/x-java") || mime.starts_with("text/x-c") {
        return "code-icon";
    } else if mime.starts_with("text/x-go") {
        return "code-icon go-icon";
    } else if mime.starts_with("text/x-ruby") {
        return "code-icon ruby-icon";
    } else if mime.starts_with("text/x-shellscript") {
        return "script-icon";
    } else if mime.starts_with("text/x-script") || mime.starts_with("text/x-php") {
        return "code-icon";
    } else if mime.starts_with("text/markdown") {
        return "code-icon md-icon";
    } else if mime.starts_with("text/html") {
        return "code-icon html-icon";
    } else if mime.starts_with("text/css") {
        return "code-icon css-icon";
    } else if mime.contains("javascript") {
        return "code-icon js-icon";
    } else if mime.contains("typescript") {
        return "code-icon ts-icon";
    } else if mime.starts_with("text/") {
        return "doc-icon";
    }

    // 3. Extension-based fallback
    if let Some(ext) = ext_of(name) {
        return match ext.to_ascii_lowercase().as_str() {
            "pdf" => "pdf-icon",
            "doc" | "docx" | "odt" | "rtf" => "doc-icon",
            "xls" | "xlsx" | "ods" | "csv" => "spreadsheet-icon",
            "ppt" | "pptx" | "odp" | "key" => "presentation-icon",
            "jpg" | "jpeg" | "png" | "gif" | "bmp" | "svg" | "webp" | "ico" | "tiff" | "tif"
            | "heic" | "heif" | "avif" => "image-icon",
            "mp4" | "avi" | "mkv" | "mov" | "wmv" | "flv" | "webm" | "m4v" => "video-icon",
            "mp3" | "wav" | "ogg" | "flac" | "aac" | "wma" | "m4a" | "opus" => "audio-icon",
            "zip" | "rar" | "7z" | "tar" | "gz" | "bz2" | "xz" | "zst" | "lz4" => "archive-icon",
            "exe" | "msi" | "dmg" | "deb" | "rpm" | "appimage" | "pkg" | "snap" | "flatpak" => {
                "installer-icon"
            }
            "py" | "pyw" => "code-icon py-icon",
            "rs" => "code-icon rust-icon",
            "go" => "code-icon go-icon",
            "java" | "kt" | "kts" | "scala" => "code-icon java-icon",
            "js" | "jsx" | "mjs" | "cjs" => "code-icon js-icon",
            "ts" | "tsx" => "code-icon ts-icon",
            "c" | "h" | "cpp" | "hpp" | "cc" | "cxx" => "code-icon c-icon",
            "cs" => "code-icon cs-icon",
            "rb" => "code-icon ruby-icon",
            "php" => "code-icon php-icon",
            "swift" => "code-icon swift-icon",
            "r" | "lua" | "pl" | "pm" => "code-icon",
            "html" | "htm" => "code-icon html-icon",
            "css" | "scss" | "sass" | "less" => "code-icon css-icon",
            "json" => "code-icon json-icon",
            "xml" => "code-icon html-icon",
            "yaml" | "yml" | "toml" | "ini" | "cfg" | "conf" => "code-icon config-icon",
            "sql" | "graphql" | "proto" => "code-icon sql-icon",
            "vue" | "svelte" => "code-icon js-icon",
            "sh" | "bash" | "zsh" | "fish" | "ps1" | "bat" | "cmd" => "script-icon",
            "md" | "markdown" | "rst" => "code-icon md-icon",
            "txt" => "doc-icon",
            _ => "",
        };
    }

    ""
}

// ─── Category label ──────────────────────────────────────────────────

/// Returns a human-readable category label, considering MIME + extension.
pub fn category_for(name: &str, mime: &str) -> &'static str {
    // 1. Specific MIME matches
    match mime {
        "application/pdf" => return "PDF",
        "application/msword"
        | "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
        | "application/vnd.oasis.opendocument.text" => return "Document",
        "application/vnd.ms-excel"
        | "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
        | "application/vnd.oasis.opendocument.spreadsheet"
        | "text/csv" => return "Spreadsheet",
        "application/vnd.ms-powerpoint"
        | "application/vnd.openxmlformats-officedocument.presentationml.presentation"
        | "application/vnd.oasis.opendocument.presentation" => return "Presentation",
        "application/zip"
        | "application/x-rar-compressed"
        | "application/vnd.rar"
        | "application/x-7z-compressed"
        | "application/gzip"
        | "application/x-tar" => return "Archive",
        "application/json"
        | "application/javascript"
        | "application/typescript"
        | "application/xml"
        | "application/sql"
        | "application/x-sh"
        | "application/x-shellscript" => return "Code",
        "application/x-apple-diskimage"
        | "application/x-ms-dos-executable"
        | "application/x-msdownload"
        | "application/x-msi" => return "Installer",
        _ => {}
    }

    // 2. MIME prefix
    if mime.starts_with("image/") {
        return "Image";
    } else if mime.starts_with("video/") {
        return "Video";
    } else if mime.starts_with("audio/") {
        return "Audio";
    } else if mime.starts_with("text/x-") || mime.contains("script") || mime.contains("javascript")
    {
        return "Code";
    } else if mime.starts_with("text/markdown") {
        return "Markdown";
    } else if mime.starts_with("text/html") || mime.starts_with("text/css") {
        return "Code";
    } else if mime.starts_with("text/") {
        return "Text";
    }

    // 3. Extension fallback
    if let Some(ext) = ext_of(name) {
        return match ext.to_ascii_lowercase().as_str() {
            "pdf" => "PDF",
            "doc" | "docx" | "odt" | "rtf" | "txt" => "Document",
            "xls" | "xlsx" | "ods" | "csv" => "Spreadsheet",
            "ppt" | "pptx" | "odp" | "key" => "Presentation",
            "jpg" | "jpeg" | "png" | "gif" | "bmp" | "svg" | "webp" | "ico" | "tiff" | "heic"
            | "avif" => "Image",
            "mp4" | "avi" | "mkv" | "mov" | "wmv" | "flv" | "webm" | "m4v" => "Video",
            "mp3" | "wav" | "ogg" | "flac" | "aac" | "wma" | "m4a" | "opus" => "Audio",
            "zip" | "rar" | "7z" | "tar" | "gz" | "bz2" | "xz" => "Archive",
            "exe" | "msi" | "dmg" | "deb" | "rpm" | "appimage" => "Installer",
            "js" | "jsx" | "ts" | "tsx" | "py" | "rs" | "go" | "java" | "c" | "cpp" | "cs"
            | "rb" | "php" | "swift" | "kt" | "scala" | "r" | "lua" | "pl" | "html" | "htm"
            | "css" | "scss" | "json" | "xml" | "yaml" | "yml" | "toml" | "sql" | "sh" | "bash"
            | "bat" | "ps1" | "vue" | "svelte" => "Code",
            "md" | "markdown" | "rst" => "Markdown",
            _ => "Document",
        };
    }

    "Document"
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
    let formatted = formatted.trim_end_matches('0').trim_end_matches('.');

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
    fn test_icon_class_for_with_extension_fallback() {
        // Specific MIME types
        assert_eq!(
            icon_class_for("doc.pdf", "application/pdf"),
            "fas fa-file-pdf"
        );
        assert_eq!(
            icon_class_for(
                "file.docx",
                "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
            ),
            "fas fa-file-word"
        );

        // Extension fallback when MIME is generic
        assert_eq!(
            icon_class_for("script.py", "application/octet-stream"),
            "fas fa-file-code"
        );
        assert_eq!(
            icon_class_for("app.dmg", "application/octet-stream"),
            "fas fa-hdd"
        );
        assert_eq!(
            icon_class_for("archive.zip", "application/octet-stream"),
            "fas fa-file-archive"
        );
        assert_eq!(
            icon_class_for("data.xlsx", "application/octet-stream"),
            "fas fa-file-excel"
        );
        assert_eq!(
            icon_class_for("run.sh", "application/octet-stream"),
            "fas fa-terminal"
        );
    }

    #[test]
    fn test_icon_special_class_for() {
        // MIME-based
        assert_eq!(icon_special_class_for("", "image/png"), "image-icon");
        assert_eq!(icon_special_class_for("", "application/pdf"), "pdf-icon");
        assert_eq!(
            icon_special_class_for("", "application/json"),
            "code-icon json-icon"
        );

        // Extension-based fallback
        assert_eq!(
            icon_special_class_for("main.py", "application/octet-stream"),
            "code-icon py-icon"
        );
        assert_eq!(
            icon_special_class_for("lib.rs", "application/octet-stream"),
            "code-icon rust-icon"
        );
        assert_eq!(
            icon_special_class_for("style.css", "application/octet-stream"),
            "code-icon css-icon"
        );
        assert_eq!(
            icon_special_class_for("data.xlsx", "application/octet-stream"),
            "spreadsheet-icon"
        );
        assert_eq!(
            icon_special_class_for("backup.tar", "application/octet-stream"),
            "archive-icon"
        );
        assert_eq!(
            icon_special_class_for("setup.dmg", "application/octet-stream"),
            "installer-icon"
        );
    }

    #[test]
    fn test_category_for() {
        // MIME-based
        assert_eq!(category_for("", "image/jpeg"), "Image");
        assert_eq!(category_for("", "video/webm"), "Video");
        assert_eq!(category_for("", "audio/ogg"), "Audio");
        assert_eq!(category_for("", "application/pdf"), "PDF");
        assert_eq!(category_for("", "application/zip"), "Archive");

        // Extension-based fallback
        assert_eq!(category_for("main.rs", "application/octet-stream"), "Code");
        assert_eq!(
            category_for("photo.jpg", "application/octet-stream"),
            "Image"
        );
        assert_eq!(
            category_for("notes.md", "application/octet-stream"),
            "Markdown"
        );
    }

    #[test]
    fn test_ext_of() {
        assert_eq!(ext_of("file.txt"), Some("txt"));
        assert_eq!(ext_of("archive.tar.gz"), Some("gz"));
        assert_eq!(ext_of("no_extension"), None);
        assert_eq!(ext_of(".gitignore"), Some("gitignore")); // dot file treated as having extension
        assert_eq!(ext_of("path/to/file.rs"), Some("rs"));
    }
}
