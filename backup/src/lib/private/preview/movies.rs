// Copyright (c) 2013 Frank Karlitschek frank@owncloud.org
// Copyright (c) 2013 Georg Ehrke georg@ownCloud.com
// This file is licensed under the Affero General Public License version 3 or
// later.
// See the COPYING-README file.

use std::fs;
use std::io::{Read, Write};
use std::path::Path;
use std::process::Command;

use async_trait::async_trait;
use regex::Regex;

use crate::preview::{Provider, ProviderError, RegisterProvider};
use crate::util::{self, OcImage, TmpFile};

pub struct Movie;

#[async_trait]
impl Provider for Movie {
    fn mime_type(&self) -> &str {
        r"video/.*"
    }

    async fn thumbnail(
        &self,
        path: &str,
        max_x: u32,
        max_y: u32,
        _scaling_up: bool,
        fileview: &dyn FileView,
    ) -> Result<Option<OcImage>, ProviderError> {
        // Check if we're running on Windows, where this is not supported
        if util::running_on_windows() {
            return Ok(None);
        }

        // Check if shell_exec is enabled and avconv is available
        if !util::is_shell_exec_enabled() || !util::is_command_available("avconv") {
            return Ok(None);
        }

        // Create temporary files
        let abs_path = TmpFile::new()?;
        let tmp_path = TmpFile::new()?;

        // Open the file and read the first megabyte
        let mut handle = fileview.fopen(path, "rb").await?;
        let mut first_mb = vec![0u8; 1048576]; // 1024 * 1024 = 1048576
        let bytes_read = handle.read(&mut first_mb)?;
        first_mb.truncate(bytes_read);

        // Write the first megabyte to the temporary file
        fs::write(&abs_path, &first_mb)?;

        // Run avconv to generate the thumbnail
        let abs_path_str = abs_path.to_str().ok_or(ProviderError::PathConversionError)?;
        let tmp_path_str = tmp_path.to_str().ok_or(ProviderError::PathConversionError)?;
        
        let status = Command::new("avconv")
            .args(&[
                "-an",
                "-y",
                "-ss", "1",
                "-i", abs_path_str,
                "-f", "mjpeg",
                "-vframes", "1",
                tmp_path_str,
            ])
            .status()?;

        if !status.success() {
            return Err(ProviderError::CommandFailed);
        }

        // Load the generated image
        let image = OcImage::new(&tmp_path);

        // Return the image if it's valid
        if image.valid() {
            Ok(Some(image))
        } else {
            Ok(None)
        }
    }
}

pub fn register(register_provider: &mut dyn RegisterProvider) {
    // movie preview is currently not supported on Windows
    if !util::running_on_windows() 
        && util::is_shell_exec_enabled() 
        && util::is_command_available("avconv") {
        
        register_provider.register(Box::new(Movie));
    }
}

#[async_trait]
pub trait FileView {
    async fn fopen(&self, path: &str, mode: &str) -> Result<Box<dyn Read>, ProviderError>;
}