//! EXIF metadata extraction from image files.
//!
//! Uses `kamadak-exif` to parse EXIF headers from JPEG/TIFF/HEIF images.
//! Extraction is cheap — only the header bytes are read, not the full image.

use chrono::{DateTime, NaiveDateTime, Utc};
use exif::{In, Reader, Tag};
use std::io::Cursor;

/// Extracted EXIF metadata fields.
#[derive(Debug, Clone, Default)]
pub struct ExifMetadata {
    /// Photo capture time (EXIF DateTimeOriginal)
    pub captured_at: Option<DateTime<Utc>>,
    /// GPS latitude in decimal degrees (positive = North)
    pub latitude: Option<f64>,
    /// GPS longitude in decimal degrees (positive = East)
    pub longitude: Option<f64>,
    /// Camera manufacturer (EXIF Make)
    pub camera_make: Option<String>,
    /// Camera model (EXIF Model)
    pub camera_model: Option<String>,
    /// EXIF Orientation tag (1-8)
    pub orientation: Option<u16>,
    /// Original image width in pixels
    pub width: Option<u32>,
    /// Original image height in pixels
    pub height: Option<u32>,
}

/// Stateless service for extracting EXIF metadata from image bytes.
pub struct ExifService;

impl ExifService {
    /// Extract EXIF metadata from raw image bytes.
    ///
    /// Returns `None` if the file has no EXIF data (e.g. PNG, GIF, WebP)
    /// or if parsing fails entirely. Individual fields may be `None` even
    /// when the EXIF block exists (not all cameras populate every tag).
    pub fn extract(data: &[u8]) -> Option<ExifMetadata> {
        let exif = Reader::new()
            .read_from_container(&mut Cursor::new(data))
            .ok()?;

        let mut meta = ExifMetadata::default();

        // ── Capture date ──
        if let Some(field) = exif.get_field(Tag::DateTimeOriginal, In::PRIMARY) {
            meta.captured_at = parse_exif_datetime(&field.display_value().to_string());
        }
        // Fallback to DateTimeDigitized if DateTimeOriginal is missing
        if meta.captured_at.is_none() {
            if let Some(field) = exif.get_field(Tag::DateTimeDigitized, In::PRIMARY) {
                meta.captured_at = parse_exif_datetime(&field.display_value().to_string());
            }
        }

        // ── GPS coordinates ──
        meta.latitude = parse_gps_coord(&exif, Tag::GPSLatitude, Tag::GPSLatitudeRef);
        meta.longitude = parse_gps_coord(&exif, Tag::GPSLongitude, Tag::GPSLongitudeRef);

        // ── Camera info ──
        if let Some(field) = exif.get_field(Tag::Make, In::PRIMARY) {
            let val = field
                .display_value()
                .to_string()
                .trim_matches('"')
                .trim()
                .to_string();
            if !val.is_empty() {
                meta.camera_make = Some(val);
            }
        }
        if let Some(field) = exif.get_field(Tag::Model, In::PRIMARY) {
            let val = field
                .display_value()
                .to_string()
                .trim_matches('"')
                .trim()
                .to_string();
            if !val.is_empty() {
                meta.camera_model = Some(val);
            }
        }

        // ── Orientation ──
        if let Some(field) = exif.get_field(Tag::Orientation, In::PRIMARY) {
            if let exif::Value::Short(ref v) = field.value {
                if let Some(&o) = v.first() {
                    if (1..=8).contains(&o) {
                        meta.orientation = Some(o);
                    }
                }
            }
        }

        // ── Dimensions ──
        if let Some(field) = exif.get_field(Tag::PixelXDimension, In::PRIMARY) {
            meta.width = parse_u32_value(&field.value);
        }
        if let Some(field) = exif.get_field(Tag::PixelYDimension, In::PRIMARY) {
            meta.height = parse_u32_value(&field.value);
        }
        // Fallback to ImageWidth/ImageLength if PixelXDimension is missing
        if meta.width.is_none() {
            if let Some(field) = exif.get_field(Tag::ImageWidth, In::PRIMARY) {
                meta.width = parse_u32_value(&field.value);
            }
        }
        if meta.height.is_none() {
            if let Some(field) = exif.get_field(Tag::ImageLength, In::PRIMARY) {
                meta.height = parse_u32_value(&field.value);
            }
        }

        Some(meta)
    }
}

/// Parse EXIF datetime string "YYYY:MM:DD HH:MM:SS" into DateTime<Utc>.
fn parse_exif_datetime(s: &str) -> Option<DateTime<Utc>> {
    // EXIF dates use ":" as separator for date parts
    let s = s.trim().trim_matches('"');
    NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S")
        .or_else(|_| NaiveDateTime::parse_from_str(s, "%Y:%m:%d %H:%M:%S"))
        .ok()
        .map(|ndt| ndt.and_utc())
}

/// Parse GPS coordinate from EXIF rational values + reference (N/S or E/W).
fn parse_gps_coord(exif: &exif::Exif, coord_tag: Tag, ref_tag: Tag) -> Option<f64> {
    let field = exif.get_field(coord_tag, In::PRIMARY)?;
    let ref_field = exif.get_field(ref_tag, In::PRIMARY)?;

    let rationals = match &field.value {
        exif::Value::Rational(v) if v.len() >= 3 => v,
        _ => return None,
    };

    let degrees = rationals[0].to_f64();
    let minutes = rationals[1].to_f64();
    let seconds = rationals[2].to_f64();

    let mut decimal = degrees + minutes / 60.0 + seconds / 3600.0;

    // Apply hemisphere sign
    let reference = ref_field.display_value().to_string();
    let reference = reference.trim().trim_matches('"');
    if reference == "S" || reference == "W" {
        decimal = -decimal;
    }

    Some(decimal)
}

/// Extract a u32 from various EXIF value types (Short, Long).
fn parse_u32_value(value: &exif::Value) -> Option<u32> {
    match value {
        exif::Value::Short(v) => v.first().map(|&x| x as u32),
        exif::Value::Long(v) => v.first().copied(),
        _ => None,
    }
}

/// Apply EXIF orientation to a `DynamicImage`.
///
/// EXIF orientation values 1-8 describe how the stored pixels relate to
/// the intended display orientation. This function transforms the image
/// to match the intended orientation.
pub fn apply_orientation(img: image::DynamicImage, orientation: u16) -> image::DynamicImage {
    match orientation {
        1 => img,                                                               // Normal
        2 => image::DynamicImage::from(image::imageops::flip_horizontal(&img)), // Mirror horizontal
        3 => image::DynamicImage::from(image::imageops::rotate180(&img)),       // Rotate 180°
        4 => image::DynamicImage::from(image::imageops::flip_vertical(&img)),   // Mirror vertical
        5 => {
            // Transpose: flip horizontal then rotate 270° (= rotate 90° CW then flip horizontal)
            let flipped = image::imageops::flip_horizontal(&img);
            image::DynamicImage::from(image::imageops::rotate270(&flipped))
        }
        6 => image::DynamicImage::from(image::imageops::rotate90(&img)), // Rotate 90° CW
        7 => {
            // Transverse: flip horizontal then rotate 90°
            let flipped = image::imageops::flip_horizontal(&img);
            image::DynamicImage::from(image::imageops::rotate90(&flipped))
        }
        8 => image::DynamicImage::from(image::imageops::rotate270(&img)), // Rotate 270° CW
        _ => img,
    }
}
