/*
 * Copyright (c) 2013 Frank Karlitschek frank@owncloud.org
 * Copyright (c) 2013 Georg Ehrke georg@ownCloud.com
 * This file is licensed under the Affero General Public License version 3 or
 * later.
 * See the COPYING-README file.
 */

use regex::Regex;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use async_trait::async_trait;

use crate::preview::{Provider, ProviderResult};
use crate::files::FileView;
use crate::image::Image;
use crate::helpers;
use crate::server_config;

pub struct Unknown;

#[async_trait]
impl Provider for Unknown {
    fn get_mime_type(&self) -> &str {
        "/.*/}"
    }

    async fn get_thumbnail(
        &self,
        path: &str,
        max_x: u32,
        max_y: u32,
        scaling_up: bool,
        file_view: Arc<dyn FileView>,
    ) -> ProviderResult<Image> {
        let mimetype = file_view.get_mime_type(path)?;

        let icon_path = helpers::mimetype_icon(&mimetype);
        let full_path = server_config::server_root()
            .join(icon_path.strip_prefix(server_config::web_root())?);

        let svg_path = {
            let mut path_buf = PathBuf::from(&full_path);
            path_buf.set_extension("svg");
            path_buf
        };

        if cfg!(feature = "imagick") && svg_path.exists() {
            #[cfg(feature = "imagick")]
            {
                use magick_rust::{MagickWand, PixelWand};
                
                // Initialize magick_rust if it hasn't been already
                let _ = magick_rust::magick_wand_genesis();
                
                let mut svg = MagickWand::new();
                svg.read_image(svg_path.to_str().ok_or("Invalid SVG path")?)?;
                
                let (res_x, res_y) = svg.get_image_resolution();
                let width = svg.get_image_width() as f64;
                let height = svg.get_image_height() as f64;
                
                let x_ratio = res_x / width;
                let y_ratio = res_y / height;
                
                svg.remove_image();
                svg.set_resolution(max_x as f64 * x_ratio, max_y as f64 * y_ratio)?;
                
                let mut pixel_wand = PixelWand::new();
                pixel_wand.set_color("transparent")?;
                svg.set_background_color(&pixel_wand)?;
                
                svg.read_image(svg_path.to_str().ok_or("Invalid SVG path")?)?;
                svg.set_image_format("png32")?;
                
                let blob = svg.write_image_blob("png32")?;
                let image = Image::load_from_data(&blob)?;
                
                return Ok(image);
            }
        }
        
        // Fallback to direct image loading
        let image = Image::new(&full_path)?;
        Ok(image)
    }
}

// Register the provider
pub fn register() {
    crate::preview::register_provider(Box::new(Unknown));
}