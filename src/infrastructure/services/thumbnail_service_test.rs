use std::sync::Arc;

use super::thumbnail_service::{ThumbnailService, ThumbnailSize};

/// Minimal valid 1x1 red PNG (68 bytes).
fn tiny_png() -> Vec<u8> {
    // Generated from a real 1×1 PNG — smallest valid RGBA image.
    let mut img = image::RgbaImage::new(1, 1);
    img.put_pixel(0, 0, image::Rgba([255, 0, 0, 255]));
    let mut buf = Vec::new();
    img.write_to(
        &mut std::io::Cursor::new(&mut buf),
        image::ImageFormat::Png,
    )
    .expect("encode test PNG");
    buf
}

/// Regression test: thumbnail generation must work when the source file lives
/// at a blob-style path (`.blobs/ab/ab1234…`) rather than a logical path
/// (`folder/image.png`).  This broke after the blob storage migration
/// (commit 3c7c16f) because the handler passed the logical path — which
/// doesn't exist on disk — to the thumbnail service.
#[tokio::test]
async fn generate_thumbnail_from_blob_path() {
    let tmp = tempfile::tempdir().expect("create temp dir");
    let storage_root = tmp.path();

    // Simulate a blob-store layout: .blobs/ab/<hash>.blob
    let blob_dir = storage_root.join(".blobs").join("ab");
    std::fs::create_dir_all(&blob_dir).expect("create blob dir");
    let blob_path = blob_dir.join("ab1234567890.blob");
    std::fs::write(&blob_path, tiny_png()).expect("write test blob");

    let svc = Arc::new(ThumbnailService::new(storage_root, 100, 10 * 1024 * 1024));
    svc.initialize().await.expect("init thumbnail dirs");

    // The key assertion: the service can read from a blob path (not a logical path)
    let result = svc
        .get_thumbnail("test-file-id", ThumbnailSize::Icon, &blob_path)
        .await;

    let thumb_bytes = result.expect("thumbnail generation should succeed from blob path");
    assert!(!thumb_bytes.is_empty(), "thumbnail bytes must not be empty");

    // Verify it's valid WebP (starts with "RIFF" magic)
    assert!(
        thumb_bytes.len() > 12 && &thumb_bytes[0..4] == b"RIFF",
        "output should be WebP format"
    );
}

/// Verify that a non-existent path produces an error, not a panic.
#[tokio::test]
async fn generate_thumbnail_nonexistent_path_returns_error() {
    let tmp = tempfile::tempdir().expect("create temp dir");
    let svc = Arc::new(ThumbnailService::new(tmp.path(), 100, 10 * 1024 * 1024));
    svc.initialize().await.expect("init thumbnail dirs");

    let bad_path = tmp.path().join("does-not-exist.png");
    let result = svc
        .get_thumbnail("missing-id", ThumbnailSize::Icon, &bad_path)
        .await;

    assert!(result.is_err(), "should fail for nonexistent file");
}
