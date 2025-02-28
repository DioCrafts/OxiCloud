// Copyright (c) 2013 Georg Ehrke georg@ownCloud.com
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use regex::Regex;
use std::io::Read;

use crate::preview::{Provider, RegisterProvider};
use crate::files::View;
use crate::image::OcImage;

pub struct Txt;

impl Provider for Txt {
    fn get_mime_type(&self) -> &str {
        r"text/plain"
    }

    async fn get_thumbnail(
        &self,
        path: &str,
        max_x: i32,
        max_y: i32,
        scaling_up: bool,
        fileview: &View,
    ) -> Option<OcImage> {
        // Open the file
        let mut file = match fileview.fopen(path, "r").await {
            Ok(f) => f,
            Err(_) => return None,
        };

        // Read content
        let mut content = String::new();
        if let Err(_) = file.read_to_string(&mut content) {
            return None;
        }

        // Don't create previews of empty text files
        if content.trim().is_empty() {
            return None;
        }

        // Split into lines
        let line_pattern = Regex::new(r"\r\n|\n|\r").unwrap();
        let lines: Vec<&str> = line_pattern.split(&content).collect();

        let font_size = 5; // 5px
        let line_size = (font_size as f32 * 1.25).ceil() as i32;

        // Create image
        let mut image = match OcImage::create(max_x as u32, max_y as u32) {
            Some(img) => img,
            None => return None,
        };

        // Set background color (white)
        image.allocate_color(255, 255, 255);
        
        // Set text color (black)
        let text_color = image.allocate_color(0, 0, 0);

        // Draw text
        for (index, line) in lines.iter().enumerate() {
            let index = index as i32 + 1;

            let x = 1;
            let y = (index * line_size) - font_size;

            image.draw_string(1, x, y, line, text_color);

            if (index * line_size) >= max_y {
                break;
            }
        }

        if image.is_valid() {
            Some(image)
        } else {
            None
        }
    }
}

// Register the provider
impl RegisterProvider for Txt {
    fn register() {
        crate::preview::register_provider(Box::new(Txt));
    }
}

pub struct MarkDown;

impl Provider for MarkDown {
    fn get_mime_type(&self) -> &str {
        r"text/(x-)?markdown"
    }
}

// MarkDown inherits the get_thumbnail method from Txt
impl std::ops::Deref for MarkDown {
    type Target = Txt;

    fn deref(&self) -> &Self::Target {
        &Txt
    }
}

// Register the provider
impl RegisterProvider for MarkDown {
    fn register() {
        crate::preview::register_provider(Box::new(MarkDown));
    }
}