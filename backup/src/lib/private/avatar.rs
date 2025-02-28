use std::path::Path;

use async_trait::async_trait;
use thiserror::Error;

/**
 * This module gets and sets users avatars.
 */

#[derive(Error, Debug)]
pub enum AvatarError {
    #[error("Unknown filetype")]
    UnknownFiletype,
    #[error("Invalid image")]
    InvalidImage,
    #[error("Image is not square")]
    NotSquare,
    #[error("File system error: {0}")]
    FileSystemError(#[from] std::io::Error),
    #[error("Image processing error: {0}")]
    ImageError(String),
}

type Result<T> = std::result::Result<T, AvatarError>;

struct Avatar {
    view: FilesView,
}

struct OcImage {
    // Fields would depend on the actual image implementation
    data: Vec<u8>,
    mime_type: String,
    width: u32,
    height: u32,
}

#[async_trait]
trait FilesViewTrait {
    async fn file_exists(&self, path: &str) -> bool;
    async fn file_get_contents(&self, path: &str) -> Result<Vec<u8>>;
    async fn file_put_contents(&self, path: &str, data: &[u8]) -> Result<()>;
    async fn unlink(&self, path: &str) -> Result<()>;
}

struct FilesView {
    base_path: String,
}

impl OcImage {
    fn new(data: &[u8]) -> Result<Self> {
        // In a real implementation, this would use image crate to parse and validate
        // For this example, we'll simulate with dummy values
        let mime_type = if data.starts_with(&[0xFF, 0xD8, 0xFF]) {
            "image/jpeg".to_string()
        } else if data.starts_with(&[0x89, 0x50, 0x4E, 0x47]) {
            "image/png".to_string()
        } else {
            return Err(AvatarError::UnknownFiletype);
        };
        
        // In real code, would extract dimensions from the image data
        let width = 100; // Dummy value
        let height = 100; // Dummy value
        
        Ok(Self {
            data: data.to_vec(),
            mime_type,
            width,
            height,
        })
    }
    
    fn mime_type(&self) -> &str {
        &self.mime_type
    }
    
    fn valid(&self) -> bool {
        // In a real implementation, this would check if the image is valid
        !self.data.is_empty()
    }
    
    fn width(&self) -> u32 {
        self.width
    }
    
    fn height(&self) -> u32 {
        self.height
    }
    
    fn resize(&mut self, size: u32) {
        // In a real implementation, this would resize the image
        // For simplicity, we'll just update the dimensions
        self.width = size;
        self.height = size;
    }
    
    fn load_from_data(&mut self, data: Vec<u8>) -> Result<()> {
        // This would process the image data in a real implementation
        self.data = data;
        Ok(())
    }
}

#[async_trait]
impl FilesViewTrait for FilesView {
    async fn file_exists(&self, path: &str) -> bool {
        Path::new(&format!("{}/{}", self.base_path, path)).exists()
    }
    
    async fn file_get_contents(&self, path: &str) -> Result<Vec<u8>> {
        tokio::fs::read(format!("{}/{}", self.base_path, path))
            .await
            .map_err(AvatarError::FileSystemError)
    }
    
    async fn file_put_contents(&self, path: &str, data: &[u8]) -> Result<()> {
        tokio::fs::write(format!("{}/{}", self.base_path, path), data)
            .await
            .map_err(AvatarError::FileSystemError)
    }
    
    async fn unlink(&self, path: &str) -> Result<()> {
        let full_path = format!("{}/{}", self.base_path, path);
        if Path::new(&full_path).exists() {
            tokio::fs::remove_file(full_path)
                .await
                .map_err(AvatarError::FileSystemError)
        } else {
            Ok(())
        }
    }
}

impl Avatar {
    /// Create a new avatar manager for a user
    /// 
    /// # Arguments
    /// * `user` - User to do avatar-management with
    pub fn new(user: &str) -> Self {
        Self {
            view: FilesView {
                base_path: format!("/{}", user),
            },
        }
    }
    
    /// Get the user's avatar
    /// 
    /// # Arguments
    /// * `size` - Size in px of the avatar, defaults to 64
    /// 
    /// # Returns
    /// * `Some(OcImage)` containing the avatar or `None` if there's no image
    pub async fn get(&self, size: u32) -> Option<OcImage> {
        let ext = if self.view.file_exists("avatar.jpg").await {
            "jpg"
        } else if self.view.file_exists("avatar.png").await {
            "png"
        } else {
            return None;
        };
        
        let data = match self.view.file_get_contents(&format!("avatar.{}", ext)).await {
            Ok(data) => data,
            Err(_) => return None,
        };
        
        let mut avatar = OcImage::new(&data).ok()?;
        avatar.load_from_data(data).ok()?;
        avatar.resize(size);
        Some(avatar)
    }
    
    /// Set the user's avatar
    /// 
    /// # Arguments
    /// * `data` - Image data to set as the new avatar
    /// 
    /// # Errors
    /// * `AvatarError::UnknownFiletype` if the provided file is not a jpg or png image
    /// * `AvatarError::InvalidImage` if the provided image is not valid
    /// * `AvatarError::NotSquare` if the image is not square
    pub async fn set(&self, data: &[u8]) -> Result<()> {
        let img = OcImage::new(data)?;
        
        let mut file_type = img.mime_type()
            .split('/')
            .last()
            .unwrap_or("")
            .to_string();
            
        if file_type == "jpeg" {
            file_type = "jpg".to_string();
        }
        
        if file_type != "jpg" && file_type != "png" {
            return Err(AvatarError::UnknownFiletype);
        }
        
        if !img.valid() {
            return Err(AvatarError::InvalidImage);
        }
        
        if img.height() != img.width() {
            return Err(AvatarError::NotSquare);
        }
        
        // Ignore potential errors when removing old avatars
        let _ = self.view.unlink("avatar.jpg").await;
        let _ = self.view.unlink("avatar.png").await;
        
        self.view.file_put_contents(&format!("avatar.{}", file_type), data).await
    }
    
    /// Remove the user's avatar
    pub async fn remove(&self) -> Result<()> {
        // Ignore potential errors when avatar files don't exist
        let _ = self.view.unlink("avatar.jpg").await;
        let _ = self.view.unlink("avatar.png").await;
        Ok(())
    }
}