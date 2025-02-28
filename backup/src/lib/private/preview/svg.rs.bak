// Copyright (c) 2013 Georg Ehrke georg@ownCloud.com
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use crate::preview::Provider;
use crate::preview::{self, register_provider};
use crate::util::image::Image;
use crate::util::log;
use anyhow::{bail, Context, Result};
use imagick::{MagickWand, PixelWand};
use regex::Regex;
use std::io::Read;

/// SVG preview provider
pub struct Svg;

impl Provider for Svg {
    fn mime_type(&self) -> Regex {
        Regex::new(r"image/svg\+xml").unwrap()
    }

    async fn thumbnail(&self, path: &str, max_x: u32, max_y: u32, scaling_up: bool, file_view: &dyn FileView) -> Result<Option<Image>> {
        // Initialize Imagick
        let mut svg = MagickWand::new();
        let mut bg = PixelWand::new();
        
        // Set background color to transparent
        bg.set_color("transparent")?;
        svg.set_background_color(&bg)?;
        
        // Read the file content
        let mut content = String::new();
        let mut file = file_view.fopen(path, "r").await
            .context("Failed to open SVG file")?;
        file.read_to_string(&mut content)
            .context("Failed to read SVG content")?;
        
        // Add XML header if missing
        if !content.starts_with("<?xml") {
            content = format!("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?>{}", content);
        }
        
        // Read the SVG blob and convert to PNG
        match svg.read_image_blob(content.as_bytes()) {
            Ok(_) => {
                svg.set_image_format("png32")?;
                
                // Create new image object
                let image_data = svg.write_image_blob("png32")?;
                let image = Image::load_from_data(&image_data)
                    .context("Failed to create image from SVG")?;
                
                // Check if image is valid
                if image.is_valid() {
                    Ok(Some(image))
                } else {
                    Ok(None)
                }
            },
            Err(e) => {
                log::error("core", &format!("SVG preview error: {}", e));
                Ok(None)
            }
        }
    }
}

// Register the provider if imagick is available
#[cfg(feature = "imagick")]
pub fn register() {
    register_provider(Box::new(Svg));
}