// Copyright (c) 2013 Georg Ehrke georg@ownCloud.com
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use std::path::Path;
use regex::Regex;
use anyhow::{Result, anyhow};
use async_trait::async_trait;

use crate::preview::provider::Provider;
use crate::files::file_view::FileView;
use crate::image::Image;
use crate::utils::getid3::{GetId3, copy_tags_to_comments};
use crate::config::SERVERROOT;

pub struct MP3;

#[async_trait]
impl Provider for MP3 {
    fn mime_type(&self) -> &str {
        r"audio/mpeg"
    }

    async fn thumbnail(
        &self,
        path: &str,
        max_x: u32,
        max_y: u32,
        scaling_up: bool,
        fileview: &dyn FileView,
    ) -> Result<Option<Image>> {
        let tmp_path = fileview.to_tmp_file(path).await?;

        let getid3 = GetId3::new();
        let mut tags = getid3.analyze(&tmp_path)?;
        copy_tags_to_comments(&mut tags);

        // Clean up the temporary file
        let _ = std::fs::remove_file(&tmp_path);

        if let Some(picture_data) = tags.id3v2.and_then(|id3| id3.apic.get(0).map(|p| &p.data)) {
            let image = Image::new(picture_data)?;
            if image.is_valid() {
                return Ok(Some(image));
            }
        }

        self.get_no_cover_thumbnail()
    }
}

impl MP3 {
    fn get_no_cover_thumbnail(&self) -> Result<Option<Image>> {
        let icon = format!("{}/core/img/filetypes/audio.png", SERVERROOT);

        if !Path::new(&icon).exists() {
            return Ok(None);
        }

        let image = Image::new_from_file(&icon)?;
        if image.is_valid() {
            Ok(Some(image))
        } else {
            Ok(None)
        }
    }
}

pub fn register() {
    crate::preview::register_provider(Box::new(MP3));
}