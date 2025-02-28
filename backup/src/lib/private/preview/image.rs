// Copyright (c) 2013 Frank Karlitschek frank@owncloud.org
// Copyright (c) 2013 Georg Ehrke georg@ownCloud.com
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use regex::Regex;
use std::io::Read;
use std::path::Path;

use crate::preview::{Provider, ProviderResult};
use crate::fileview::FileView;
use crate::image::OcImage;

pub struct Image;

impl Provider for Image {
    fn get_mime_type(&self) -> &str {
        r"image\/.*"
    }

    async fn get_thumbnail(
        &self,
        path: &Path,
        max_x: u32,
        max_y: u32,
        scaling_up: bool,
        fileview: &dyn FileView,
    ) -> ProviderResult<OcImage> {
        // Get file info
        let file_info = fileview.get_file_info(path).await?;

        // Check if file is encrypted
        let image = if file_info.is_encrypted() {
            let mut content = Vec::new();
            let mut file = fileview.fopen(path, "r").await?;
            file.read_to_end(&mut content)?;
            OcImage::from_bytes(&content)?
        } else {
            let local_path = fileview.get_local_file(path).await?;
            OcImage::load_from_file(&local_path)?
        };

        if image.is_valid() {
            Ok(image)
        } else {
            Err("Invalid image".into())
        }
    }
}

pub fn register() {
    crate::preview::register_provider(Box::new(Image));
}