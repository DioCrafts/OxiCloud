// Copyright (c) 2013 Georg Ehrke georg@ownCloud.com
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use std::fs;
use std::path::Path;
use regex::Regex;
use async_trait::async_trait;
use anyhow::{Result, Context, bail};
use image::DynamicImage;
use log::error;
use imagick::{MagickWand, PixelWand};

use crate::preview::{Provider, FileView, register_provider};

pub struct Pdf;

#[async_trait]
impl Provider for Pdf {
    fn mime_type(&self) -> Regex {
        Regex::new(r"application/pdf").unwrap()
    }

    async fn get_thumbnail(&self, path: &str, max_x: u32, max_y: u32, scaling_up: bool, fileview: &dyn FileView) -> Result<Option<DynamicImage>> {
        // Only proceed if imagick is available
        if !has_imagick() {
            return Ok(None);
        }

        let tmp_path = fileview.to_tmp_file(path).await?;

        // Create imagick object from pdf
        let result = try_get_pdf_preview(&tmp_path);

        // Clean up temp file
        if let Err(e) = fs::remove_file(&tmp_path) {
            error!("Failed to remove temporary file {}: {}", tmp_path.display(), e);
        }

        match result {
            Ok(image) => {
                if image.is_valid() {
                    Ok(Some(image))
                } else {
                    Ok(None)
                }
            },
            Err(e) => {
                error!("{}", e);
                Ok(None)
            }
        }
    }
}

fn has_imagick() -> bool {
    // In a real implementation, this would check if the imagick library is available
    // For now, we'll assume it is
    true
}

fn try_get_pdf_preview(tmp_path: &Path) -> Result<DynamicImage> {
    // Use imagick to convert PDF's first page to an image
    let mut wand = MagickWand::new();
    
    // Append [0] to get the first page
    let path_str = format!("{}[0]", tmp_path.to_string_lossy());
    
    wand.read_image(&path_str)
        .context("Failed to read PDF with imagick")?;
    
    wand.set_image_format("jpg")
        .context("Failed to set image format")?;
    
    // Convert imagick result to DynamicImage
    let blob = wand.write_image_blob("jpg")
        .context("Failed to write image blob")?;
    
    let img = image::load_from_memory(&blob)
        .context("Failed to load image from memory")?;
    
    Ok(img)
}

pub fn register() {
    if has_imagick() {
        register_provider(Box::new(Pdf));
    }
}