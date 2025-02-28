use std::path::Path;

/// Provider trait for preview generation
pub trait Provider {
    /// Returns the MIME type that this provider can handle
    fn get_mime_type(&self) -> &str;

    /// Get thumbnail for file at path
    ///
    /// # Parameters
    /// * `path` - Path of file
    /// * `max_x` - The maximum X size of the thumbnail. It can be smaller depending on the shape of the image
    /// * `max_y` - The maximum Y size of the thumbnail. It can be smaller depending on the shape of the image
    /// * `scaling_up` - Disable/Enable upscaling of previews
    /// * `file_view` - File view object of user folder
    ///
    /// # Returns
    /// * `Ok(Some(Image))` - If the preview was generated successfully
    /// * `Ok(None)` - If no preview was generated
    /// * `Err` - If an error occurred during preview generation
    fn get_thumbnail<P: AsRef<Path>, V>(
        &self,
        path: P,
        max_x: u32,
        max_y: u32,
        scaling_up: bool,
        file_view: &V,
    ) -> Result<Option<Image>, PreviewError>;
}

/// Base implementation for preview providers
pub struct ProviderBase<T> {
    options: T,
}

impl<T> ProviderBase<T> {
    /// Create a new provider with the given options
    pub fn new(options: T) -> Self {
        Self { options }
    }

    /// Get the provider options
    pub fn options(&self) -> &T {
        &self.options
    }
}

/// Represents an image in the preview system
pub struct Image {
    // Image implementation details would go here
}

/// Error type for preview operations
#[derive(Debug, thiserror::Error)]
pub enum PreviewError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Image processing error: {0}")]
    ImageProcessing(String),
    
    #[error("File not found: {0}")]
    FileNotFound(String),
    
    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),
}