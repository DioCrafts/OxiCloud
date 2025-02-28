/**
 * Copyright (c) 2013 Georg Ehrke georg@ownCloud.com
 * This file is licensed under the Affero General Public License version 3 or
 * later.
 * See the COPYING-README file.
 */

use async_trait::async_trait;
use regex::Regex;
use std::path::Path;
use std::fs;
use std::process::Command;

/* //There is no (good) rust-only solution for converting 2003 word documents to pdfs / pngs ...
pub struct DOC;

#[async_trait]
impl Provider for DOC {
    fn get_mime_type(&self) -> Option<Regex> {
        Regex::new(r"application/msword").ok()
    }

    async fn get_thumbnail(
        &self,
        path: &Path,
        max_x: u32,
        max_y: u32,
        scaling_up: bool,
        file_view: &dyn FileView,
    ) -> Result<Option<Image>, PreviewError> {
        // Not implemented
        Ok(None)
    }
}

preview_provider_registration!(DOC);
*/

pub struct DOCX;

#[async_trait]
impl Provider for DOCX {
    fn get_mime_type(&self) -> Option<Regex> {
        Regex::new(r"application/vnd.openxmlformats-officedocument.wordprocessingml.document").ok()
    }

    async fn get_thumbnail(
        &self,
        path: &Path,
        max_x: u32,
        max_y: u32,
        scaling_up: bool,
        file_view: &dyn FileView,
    ) -> Result<Option<Image>, PreviewError> {
        // Create temp file
        let tmp_doc = file_view.to_tmp_file(path)?;
        
        // Use a transform library to convert docx to PDF
        // Note: this would need to be implemented with a Rust library or external command
        transform_doc_to_pdf(&tmp_doc)?;
        
        // Use an image processing library to convert PDF to image
        let pdf_image = convert_pdf_to_image(&tmp_doc)?;
        
        // Clean up temp file
        fs::remove_file(&tmp_doc)?;
        
        // Create image object and validate
        let image = Image::new(pdf_image);
        if image.is_valid() {
            Ok(Some(image))
        } else {
            Ok(None)
        }
    }
}

preview_provider_registration!(DOCX);

pub struct MSOfficeExcel;

#[async_trait]
impl Provider for MSOfficeExcel {
    fn get_mime_type(&self) -> Option<Regex> {
        None
    }

    async fn get_thumbnail(
        &self,
        path: &Path,
        max_x: u32,
        max_y: u32,
        scaling_up: bool,
        file_view: &dyn FileView,
    ) -> Result<Option<Image>, PreviewError> {
        // Create temp files
        let abs_path = file_view.to_tmp_file(path)?;
        let tmp_path = create_temp_file()?;
        
        // Convert Excel to PDF
        // This would need to be implemented with a Rust library or external command
        convert_excel_to_pdf(&abs_path, &tmp_path)?;
        
        // Convert PDF to image
        let pdf_image = convert_pdf_to_image(&tmp_path)?;
        
        // Clean up temp files
        fs::remove_file(&abs_path)?;
        fs::remove_file(&tmp_path)?;
        
        // Create image object and validate
        let image = Image::new(pdf_image);
        if image.is_valid() {
            Ok(Some(image))
        } else {
            Ok(None)
        }
    }
}

pub struct XLS;

#[async_trait]
impl Provider for XLS {
    fn get_mime_type(&self) -> Option<Regex> {
        Regex::new(r"application/vnd.ms-excel").ok()
    }
}

// Implement delegation to MSOfficeExcel
impl_delegate_provider!(XLS, MSOfficeExcel);

preview_provider_registration!(XLS);

pub struct XLSX;

#[async_trait]
impl Provider for XLSX {
    fn get_mime_type(&self) -> Option<Regex> {
        Regex::new(r"application/vnd.openxmlformats-officedocument.spreadsheetml.sheet").ok()
    }
}

// Implement delegation to MSOfficeExcel
impl_delegate_provider!(XLSX, MSOfficeExcel);

preview_provider_registration!(XLSX);

/* //There is no (good) rust-only solution for converting powerpoint documents to pdfs / pngs ...
pub struct MSOfficePowerPoint;

#[async_trait]
impl Provider for MSOfficePowerPoint {
    fn get_mime_type(&self) -> Option<Regex> {
        None
    }

    async fn get_thumbnail(
        &self,
        path: &Path,
        max_x: u32,
        max_y: u32,
        scaling_up: bool,
        file_view: &dyn FileView,
    ) -> Result<Option<Image>, PreviewError> {
        Ok(None)
    }
}

pub struct PPT;

#[async_trait]
impl Provider for PPT {
    fn get_mime_type(&self) -> Option<Regex> {
        Regex::new(r"application/vnd.ms-powerpoint").ok()
    }
}

impl_delegate_provider!(PPT, MSOfficePowerPoint);

preview_provider_registration!(PPT);

pub struct PPTX;

#[async_trait]
impl Provider for PPTX {
    fn get_mime_type(&self) -> Option<Regex> {
        Regex::new(r"application/vnd.openxmlformats-officedocument.presentationml.presentation").ok()
    }
}

impl_delegate_provider!(PPTX, MSOfficePowerPoint);

preview_provider_registration!(PPTX);
*/

// Helper functions

fn transform_doc_to_pdf(input_path: &Path) -> Result<(), PreviewError> {
    // Implementation would use a Rust library or call an external command
    // Example with external command:
    let output = Command::new("libreoffice")
        .args(&["--headless", "--convert-to", "pdf", input_path.to_str().unwrap()])
        .output()
        .map_err(|e| PreviewError::ConversionError(e.to_string()))?;
    
    if !output.status.success() {
        return Err(PreviewError::ConversionError(
            String::from_utf8_lossy(&output.stderr).to_string()
        ));
    }
    
    Ok(())
}

fn convert_pdf_to_image(pdf_path: &Path) -> Result<Vec<u8>, PreviewError> {
    // Implementation would use a Rust library or call an external command
    // Example with external command:
    let output = Command::new("convert")
        .args(&[format!("{}[0]", pdf_path.to_str().unwrap()), "jpg:-"])
        .output()
        .map_err(|e| PreviewError::ConversionError(e.to_string()))?;
    
    if !output.status.success() {
        return Err(PreviewError::ConversionError(
            String::from_utf8_lossy(&output.stderr).to_string()
        ));
    }
    
    Ok(output.stdout)
}

fn convert_excel_to_pdf(input_path: &Path, output_path: &Path) -> Result<(), PreviewError> {
    // Implementation would use a Rust library or call an external command
    // Example with external command:
    let output = Command::new("libreoffice")
        .args(&[
            "--headless", 
            "--convert-to", 
            "pdf", 
            "--outdir", 
            output_path.parent().unwrap().to_str().unwrap(),
            input_path.to_str().unwrap()
        ])
        .output()
        .map_err(|e| PreviewError::ConversionError(e.to_string()))?;
    
    if !output.status.success() {
        return Err(PreviewError::ConversionError(
            String::from_utf8_lossy(&output.stderr).to_string()
        ));
    }
    
    Ok(())
}

fn create_temp_file() -> Result<std::path::PathBuf, PreviewError> {
    tempfile::NamedTempFile::new()
        .map(|file| file.into_temp_path())
        .map_err(|e| PreviewError::IoError(e.to_string()))
}

// Required types/traits that would be defined elsewhere in the application

#[async_trait]
pub trait Provider: Send + Sync {
    fn get_mime_type(&self) -> Option<Regex>;
    
    async fn get_thumbnail(
        &self,
        path: &Path,
        max_x: u32,
        max_y: u32,
        scaling_up: bool,
        file_view: &dyn FileView,
    ) -> Result<Option<Image>, PreviewError>;
}

pub trait FileView: Send + Sync {
    fn to_tmp_file(&self, path: &Path) -> Result<std::path::PathBuf, PreviewError>;
}

#[derive(Debug)]
pub enum PreviewError {
    IoError(String),
    ConversionError(String),
    ImageError(String),
}

pub struct Image {
    data: Vec<u8>,
}

impl Image {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }
    
    pub fn is_valid(&self) -> bool {
        !self.data.is_empty()
    }
}

// Macros that would be defined elsewhere

macro_rules! preview_provider_registration {
    ($provider:ty) => {
        // Registration would happen here
    };
}

macro_rules! impl_delegate_provider {
    ($child:ty, $parent:ty) => {
        #[async_trait]
        impl Provider for $child {
            async fn get_thumbnail(
                &self,
                path: &Path,
                max_x: u32,
                max_y: u32,
                scaling_up: bool,
                file_view: &dyn FileView,
            ) -> Result<Option<Image>, PreviewError> {
                let parent = <$parent>::default();
                parent.get_thumbnail(path, max_x, max_y, scaling_up, file_view).await
            }
        }
    };
}