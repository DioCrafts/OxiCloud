// Módulos generados automáticamente

pub mod js;
pub mod css;

// Contenido fusionado desde src/lib/private/minimizer.rs
use std::io::Read;
use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use std::io;

/// Trait defining the common functionality for asset minifiers
pub trait Minimizer {
    /// The content type of the minimized files
    fn content_type(&self) -> &str;

    /// Minimize the given files
    fn minimize_files(&self, files: &[FileInfo]) -> Result<String, MinimizeError>;

    /// Generate an ETag for the given files
    fn generate_etag(&self, files: &[FileInfo]) -> String {
        let fullpath_files: Vec<String> = files
            .iter()
            .map(|file_info| format!("{}/{}", file_info.base_path, file_info.name))
            .collect();
        
        cache::generate_cache_key_from_files(&fullpath_files)
    }

    /// Output the minimized files with proper headers
    fn output(&self, files: &[FileInfo], cache_key: &str) -> Result<(), MinimizeError> {
        let etag = self.generate_etag(files);
        let cache_key = format!("{}-{}", cache_key, etag);
        
        // Send content type header
        response::set_header("Content-Type", self.content_type());
        response::enable_caching();
        
        let cache = cache::get_global_cache();
        let mut gzout = None;
        
        // Check if we can use cached version
        if !request::is_no_cache() && !cfg!(debug_assertions) {
            response::set_etag_header(&etag);
            gzout = cache.get(&format!("{}.gz", cache_key));
        }
        
        // If not in cache, minimize and compress
        let gzout = match gzout {
            Some(data) => data,
            None => {
                let out = self.minimize_files(files)?;
                let gzout = compress(&out)?;
                cache.set(&format!("{}.gz", cache_key), &gzout);
                response::set_etag_header(&etag);
                gzout
            }
        };
        
        // On some systems (e.g. SLES 11, but not Ubuntu) mod_deflate and zlib compression 
        // will compress the output twice. This results in broken core.css and core.js.
        // To avoid it, we switch off zlib compression.
        // Since mod_deflate is still active, Apache will compress what needs to be compressed, 
        // i.e. no disadvantage.
        #[cfg(feature = "apache")]
        if apache::get_modules().contains(&"mod_deflate".to_string()) && 
           server_config::get("zlib.output_compression") == "On" {
            server_config::set("zlib.output_compression", "Off");
        }
        
        // Determine if we should send gzipped content
        let out = if let Some(encoding) = request::accept_gzip() {
            response::set_header("Content-Encoding", &encoding);
            gzout
        } else {
            decompress(&gzout)?
        };
        
        response::set_header("Content-Length", &out.len().to_string());
        response::send_bytes(&out);
        
        Ok(())
    }

    /// Clear the minification cache
    fn clear_cache(&self) {
        let cache = cache::get_global_cache();
        cache.clear("core.css");
        cache.clear("core.js");
    }
}

/// Represents a file to be minimized
#[derive(Debug, Clone)]
pub struct FileInfo {
    pub base_path: String,
    pub media_type: String, 
    pub name: String,
}

/// Error type for minification operations
#[derive(Debug, thiserror::Error)]
pub enum MinimizeError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    
    #[error("File not found: {0}")]
    FileNotFound(String),
    
    #[error("Cache error: {0}")]
    Cache(String),
    
    #[error("Minification error: {0}")]
    Minification(String),
}

/// Compress a string with gzip
fn compress(data: &str) -> Result<Vec<u8>, io::Error> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    std::io::Write::write_all(&mut encoder, data.as_bytes())?;
    encoder.finish()
}

/// Decompress gzipped data
fn decompress(data: &[u8]) -> Result<Vec<u8>, io::Error> {
    // Simple gzip header check (1F 8B 08 00 00 00 00 00 00)
    if data.len() < 10 || data[0..2] != [0x1f, 0x8b] || data[2] != 8 || data[3..9] != [0, 0, 0, 0, 0, 0] {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Not a valid gzip format"));
    }
    
    let mut decoder = GzDecoder::new(&data[..]);
    let mut buf = Vec::new();
    decoder.read_to_end(&mut buf)?;
    Ok(buf)
}

// Mock implementations for external dependencies
mod cache {
    pub struct Cache;
    
    impl Cache {
        pub fn get(&self, _key: &str) -> Option<Vec<u8>> {
            None
        }
        
        pub fn set(&self, _key: &str, _value: &[u8]) {
            // Implementation would go here
        }
        
        pub fn clear(&self, _prefix: &str) {
            // Implementation would go here
        }
    }
    
    pub fn get_global_cache() -> Cache {
        Cache {}
    }
    
    pub fn generate_cache_key_from_files(_files: &[String]) -> String {
        // Implementation would go here
        "generated_etag".to_string()
    }
}

mod response {
    pub fn set_header(_name: &str, _value: &str) {
        // Implementation would go here
    }
    
    pub fn enable_caching() {
        // Implementation would go here
    }
    
    pub fn set_etag_header(_etag: &str) {
        // Implementation would go here
    }
    
    pub fn send_bytes(_bytes: &[u8]) {
        // Implementation would go here
    }
}

mod request {
    pub fn is_no_cache() -> bool {
        false
    }
    
    pub fn accept_gzip() -> Option<String> {
        Some("gzip".to_string())
    }
}

#[cfg(feature = "apache")]
mod apache {
    pub fn get_modules() -> Vec<String> {
        Vec::new()
    }
}

#[cfg(feature = "apache")]
mod server_config {
    pub fn get(_key: &str) -> String {
        "Off".to_string()
    }
    
    pub fn set(_key: &str, _value: &str) {
        // Implementation would go here
    }
}