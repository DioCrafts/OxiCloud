/**
 * Copyright (c) 2013 Georg Ehrke georg@ownCloud.com
 * This file is licensed under the Affero General Public License version 3 or
 * later.
 * See the COPYING-README file.
 */
use crate::preview::Provider;
use crate::util::{self, TempFile};
use async_trait::async_trait;
use regex::Regex;
use std::fs;
use std::path::Path;
use std::process::Command;
use tracing::{error, info};

// Only define these providers if not running on Windows
#[cfg(not(target_os = "windows"))]
pub mod office {
    use super::*;

    pub struct Office {
        cmd: Option<String>,
    }

    impl Office {
        pub fn new() -> Self {
            let mut office = Self { cmd: None };
            office.init_cmd();
            office
        }

        fn init_cmd(&mut self) {
            let mut cmd = String::new();

            if let Some(path) = crate::config::get_system_value::<String>("preview_libreoffice_path") {
                cmd = path;
            }

            if cmd.is_empty() {
                if let Ok(output) = Command::new("which").arg("libreoffice").output() {
                    if !output.stdout.is_empty() {
                        cmd = "libreoffice".to_string();
                    }
                }
            }

            if cmd.is_empty() {
                if let Ok(output) = Command::new("which").arg("openoffice").output() {
                    if !output.stdout.is_empty() {
                        cmd = "openoffice".to_string();
                    }
                }
            }

            self.cmd = if cmd.is_empty() { None } else { Some(cmd) };
        }
    }

    #[async_trait]
    impl Provider for Office {
        fn mime_type(&self) -> Option<Regex> {
            None
        }

        async fn thumbnail(
            &self,
            path: &str,
            max_x: u32,
            max_y: u32,
            scaling_up: bool,
            fileview: &dyn TempFile,
        ) -> Option<crate::image::Image> {
            if self.cmd.is_none() {
                return None;
            }

            let abs_path = fileview.to_tmp_file(path)?;
            let tmp_dir = std::env::temp_dir();
            let tmp_dir_str = tmp_dir.to_str()?;

            let default_parameters = " --headless --nologo --nofirststartwizard --invisible --norestore -convert-to pdf -outdir ";
            let cl_parameters = crate::config::get_system_value::<String>(
                "preview_office_cl_parameters",
            ).unwrap_or_else(|| default_parameters.to_string());

            let export_cmd = format!("export HOME=/{}", tmp_dir_str);
            let exec_cmd = format!(
                "{} {} {} {}",
                self.cmd.as_ref()?,
                cl_parameters,
                shell_escape::escape(tmp_dir_str.into()),
                shell_escape::escape(abs_path.to_string_lossy().into())
            );

            // Execute the commands
            let status = Command::new("sh")
                .arg("-c")
                .arg(format!("{}\n{}", export_cmd, exec_cmd))
                .status();

            if let Err(e) = status {
                error!("Failed to execute office conversion: {}", e);
                return None;
            }

            // Create image from the PDF
            let pdf_path = format!("{}.pdf", abs_path.to_string_lossy());
            
            let image = match crate::image::Image::from_pdf(&pdf_path, 0) {
                Ok(img) => img,
                Err(e) => {
                    error!("Failed to create image from PDF: {}", e);
                    // Clean up temporary files
                    let _ = fs::remove_file(&abs_path);
                    let _ = fs::remove_file(&pdf_path);
                    return None;
                }
            };

            // Clean up temporary files
            let _ = fs::remove_file(&abs_path);
            let _ = fs::remove_file(&pdf_path);

            if image.is_valid() {
                Some(image)
            } else {
                None
            }
        }
    }

    pub struct MSOfficeDoc(Office);

    impl MSOfficeDoc {
        pub fn new() -> Self {
            Self(Office::new())
        }
    }

    #[async_trait]
    impl Provider for MSOfficeDoc {
        fn mime_type(&self) -> Option<Regex> {
            Some(Regex::new(r"application/msword").ok()?)
        }

        async fn thumbnail(
            &self,
            path: &str,
            max_x: u32,
            max_y: u32,
            scaling_up: bool,
            fileview: &dyn TempFile,
        ) -> Option<crate::image::Image> {
            self.0.thumbnail(path, max_x, max_y, scaling_up, fileview).await
        }
    }

    pub struct MSOffice2003(Office);

    impl MSOffice2003 {
        pub fn new() -> Self {
            Self(Office::new())
        }
    }

    #[async_trait]
    impl Provider for MSOffice2003 {
        fn mime_type(&self) -> Option<Regex> {
            Some(Regex::new(r"application/vnd\.ms-.*").ok()?)
        }

        async fn thumbnail(
            &self,
            path: &str,
            max_x: u32,
            max_y: u32,
            scaling_up: bool,
            fileview: &dyn TempFile,
        ) -> Option<crate::image::Image> {
            self.0.thumbnail(path, max_x, max_y, scaling_up, fileview).await
        }
    }

    pub struct MSOffice2007(Office);

    impl MSOffice2007 {
        pub fn new() -> Self {
            Self(Office::new())
        }
    }

    #[async_trait]
    impl Provider for MSOffice2007 {
        fn mime_type(&self) -> Option<Regex> {
            Some(Regex::new(r"application/vnd\.openxmlformats-officedocument\..*").ok()?)
        }

        async fn thumbnail(
            &self,
            path: &str,
            max_x: u32,
            max_y: u32,
            scaling_up: bool,
            fileview: &dyn TempFile,
        ) -> Option<crate::image::Image> {
            self.0.thumbnail(path, max_x, max_y, scaling_up, fileview).await
        }
    }

    pub struct OpenDocument(Office);

    impl OpenDocument {
        pub fn new() -> Self {
            Self(Office::new())
        }
    }

    #[async_trait]
    impl Provider for OpenDocument {
        fn mime_type(&self) -> Option<Regex> {
            Some(Regex::new(r"application/vnd\.oasis\.opendocument\..*").ok()?)
        }

        async fn thumbnail(
            &self,
            path: &str,
            max_x: u32,
            max_y: u32,
            scaling_up: bool,
            fileview: &dyn TempFile,
        ) -> Option<crate::image::Image> {
            self.0.thumbnail(path, max_x, max_y, scaling_up, fileview).await
        }
    }

    pub struct StarOffice(Office);

    impl StarOffice {
        pub fn new() -> Self {
            Self(Office::new())
        }
    }

    #[async_trait]
    impl Provider for StarOffice {
        fn mime_type(&self) -> Option<Regex> {
            Some(Regex::new(r"application/vnd\.sun\.xml\..*").ok()?)
        }

        async fn thumbnail(
            &self,
            path: &str,
            max_x: u32,
            max_y: u32,
            scaling_up: bool,
            fileview: &dyn TempFile,
        ) -> Option<crate::image::Image> {
            self.0.thumbnail(path, max_x, max_y, scaling_up, fileview).await
        }
    }

    pub fn register_providers() {
        crate::preview::register_provider(Box::new(MSOfficeDoc::new()));
        crate::preview::register_provider(Box::new(MSOffice2003::new()));
        crate::preview::register_provider(Box::new(MSOffice2007::new()));
        crate::preview::register_provider(Box::new(OpenDocument::new()));
        crate::preview::register_provider(Box::new(StarOffice::new()));
    }
}

#[cfg(not(target_os = "windows"))]
pub use self::office::register_providers;

// Empty implementation for Windows
#[cfg(target_os = "windows")]
pub fn register_providers() {
    // Office preview is currently not supported on Windows
}