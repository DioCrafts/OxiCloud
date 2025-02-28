// Módulos generados automáticamente

pub mod svg;
pub mod office;
pub mod pdf;
pub mod image;
pub mod mp3;
pub mod provider;
pub mod unknown;
pub mod office_dash_fallback;
pub mod movies;
pub mod office_dash_cl;
pub mod txt;

// Contenido fusionado desde src/lib/private/preview.rs
//! Copyright (c) 2013 Frank Karlitschek frank@owncloud.org
//! Copyright (c) 2013 Georg Ehrke georg@ownCloud.com
//! This file is licensed under the Affero General Public License version 3 or
//! later.
//! See the COPYING-README file.
//!
//! Thumbnails:
//! structure of filename:
//! /data/user/thumbnails/pathhash/x-y.png

use std::collections::HashMap;
use std::sync::Once;
use std::path::Path;

mod preview {
    pub mod image;
    pub mod movies;
    pub mod mp3;
    pub mod pdf;
    pub mod svg;
    pub mod txt;
    pub mod unknown;
    pub mod office;
}

use crate::files::View;
use crate::log;
use crate::config;
use crate::user;
use crate::response;
use crate::image::Image;

/// Trait for preview providers
pub trait PreviewProvider: Send + Sync {
    fn get_mime_type(&self) -> &str;
    fn get_thumbnail(&self, file: &str, max_x: u32, max_y: u32, scaling_up: bool, file_view: &View) -> Option<Image>;
}

pub struct Preview {
    /// The thumbnail folder
    pub THUMBNAILS_FOLDER: &'static str ,  // Convertido de const

    // Config
    max_scale_factor: u32,
    config_max_x: Option<u32>,
    config_max_y: Option<u32>,

    // File views
    file_view: View,
    user_view: View,

    // Variables
    file: String,
    max_x: u32,
    max_y: u32,
    scaling_up: bool,

    // Preview image object
    preview: Option<Image>,
}

// Static provider registry
static mut PROVIDERS: Option<HashMap<String, Box<dyn PreviewProvider>>> = None;
static mut REGISTERED_PROVIDERS: Option<Vec<(String, HashMap<String, String>)>> = None;
static INIT_PROVIDERS: Once = Once::new();

impl Preview {
    /// Creates a new Preview instance
    ///
    /// # Arguments
    /// * `user` - Userid - if no user is given, current user will be used
    /// * `root` - Path of root
    /// * `file` - The path to the file where you want a thumbnail from
    /// * `max_x` - The maximum X size of the thumbnail. It can be smaller depending on the shape of the image
    /// * `max_y` - The maximum Y size of the thumbnail. It can be smaller depending on the shape of the image
    /// * `scaling_up` - Disable/Enable upscaling of previews
    ///
    /// # Returns
    /// * `Result<Preview, String>` - Preview instance or error message
    pub fn new(
        user: &str, 
        root: &str, 
        file: &str, 
        max_x: u32, 
        max_y: u32, 
        scaling_up: bool
    ) -> Result<Self, String> {
        // Set config
        let config_max_x = config::get_value::<u32>("preview_max_x", None);
        let config_max_y = config::get_value::<u32>("preview_max_y", None);
        let max_scale_factor = config::get_value::<u32>("preview_max_scale_factor", Some(2));
        
        // Init user if empty
        let user = if user.is_empty() {
            user::get_user()
        } else {
            user.to_string()
        };
        
        // Init file views
        let file_view = View::new(&format!("/{}/{}", user, root));
        let user_view = View::new(&format!("/{}", user));
        
        // Create preview instance
        let mut preview = Self {
            max_scale_factor,
            config_max_x,
            config_max_y,
            file_view,
            user_view,
            file: String::new(),
            max_x: 1,
            max_y: 1,
            scaling_up: true,
            preview: None,
        };
        
        // Set parameters
        preview.set_file(file);
        preview.set_max_x(max_x)?;
        preview.set_max_y(max_y)?;
        preview.set_scaling_up(scaling_up);
        
        // Check if there are preview backends
        Self::init_providers();
        
        unsafe {
            if PROVIDERS.as_ref().unwrap().is_empty() {
                log::write("core", "No preview providers exist", log::Level::Error);
                return Err("No preview providers".to_string());
            }
        }
        
        Ok(preview)
    }
    
    /// Returns the path of the file you want a thumbnail from
    pub fn get_file(&self) -> &str {
        &self.file
    }
    
    /// Returns the max width of the preview
    pub fn get_max_x(&self) -> u32 {
        self.max_x
    }
    
    /// Returns the max height of the preview
    pub fn get_max_y(&self) -> u32 {
        self.max_y
    }
    
    /// Returns whether or not scaling up is enabled
    pub fn get_scaling_up(&self) -> bool {
        self.scaling_up
    }
    
    /// Returns the name of the thumbnails folder
    pub fn get_thumbnails_folder(&self) -> &'static str {
        Self::THUMBNAILS_FOLDER
    }
    
    /// Returns the max scale factor
    pub fn get_max_scale_factor(&self) -> u32 {
        self.max_scale_factor
    }
    
    /// Returns the max width set in configuration
    pub fn get_config_max_x(&self) -> Option<u32> {
        self.config_max_x
    }
    
    /// Returns the max height set in configuration
    pub fn get_config_max_y(&self) -> Option<u32> {
        self.config_max_y
    }
    
    /// Set the path of the file you want a thumbnail from
    pub fn set_file(&mut self, file: &str) -> &mut Self {
        self.file = file.to_string();
        self
    }
    
    /// Set the max width of the preview
    pub fn set_max_x(&mut self, max_x: u32) -> Result<&mut Self, String> {
        if max_x == 0 {
            return Err("Cannot set width of 0 or smaller!".to_string());
        }
        
        let mut max_x = max_x;
        if let Some(config_max_x) = self.get_config_max_x() {
            if max_x > config_max_x {
                log::write("core", &format!("maxX reduced from {} to {}", max_x, config_max_x), log::Level::Debug);
                max_x = config_max_x;
            }
        }
        
        self.max_x = max_x;
        Ok(self)
    }
    
    /// Set the max height of the preview
    pub fn set_max_y(&mut self, max_y: u32) -> Result<&mut Self, String> {
        if max_y == 0 {
            return Err("Cannot set height of 0 or smaller!".to_string());
        }
        
        let mut max_y = max_y;
        if let Some(config_max_y) = self.get_config_max_y() {
            if max_y > config_max_y {
                log::write("core", &format!("maxY reduced from {} to {}", max_y, config_max_y), log::Level::Debug);
                max_y = config_max_y;
            }
        }
        
        self.max_y = max_y;
        Ok(self)
    }
    
    /// Set whether or not scaling up is enabled
    pub fn set_scaling_up(&mut self, scaling_up: bool) -> &mut Self {
        let scaling_up = if self.get_max_scale_factor() == 1 {
            false
        } else {
            scaling_up
        };
        
        self.scaling_up = scaling_up;
        self
    }
    
    /// Check if all parameters are valid
    pub fn is_file_valid(&self) -> bool {
        let file = self.get_file();
        if file.is_empty() {
            log::write("core", "No filename passed", log::Level::Debug);
            return false;
        }
        
        if !self.file_view.file_exists(file) {
            log::write("core", &format!("File:\"{}\" not found", file), log::Level::Debug);
            return false;
        }
        
        true
    }
    
    /// Deletes previews of a file with specific x and y
    pub fn delete_preview(&self) -> bool {
        let file = self.get_file();
        
        let file_info = self.file_view.get_file_info(file);
        let file_id = file_info.get("fileid").unwrap();
        
        let preview_path = format!("{}/{}/{}-{}.png", 
            self.get_thumbnails_folder(), 
            file_id, 
            self.get_max_x(), 
            self.get_max_y()
        );
        
        self.user_view.unlink(&preview_path);
        !self.user_view.file_exists(&preview_path)
    }
    
    /// Deletes all previews of a file
    pub fn delete_all_previews(&self) -> bool {
        let file = self.get_file();
        
        let file_info = self.file_view.get_file_info(file);
        let file_id = file_info.get("fileid").unwrap();
        
        let preview_path = format!("{}/{}/", 
            self.get_thumbnails_folder(), 
            file_id
        );
        
        self.user_view.delete_all(&preview_path);
        self.user_view.rmdir(&preview_path);
        !self.user_view.is_dir(&preview_path)
    }
    
    /// Check if thumbnail or bigger version of thumbnail of file is cached
    fn is_cached(&self) -> Option<String> {
        let file = self.get_file();
        let max_x = self.get_max_x();
        let max_y = self.get_max_y();
        let scaling_up = self.get_scaling_up();
        let max_scale_factor = self.get_max_scale_factor();
        
        let file_info = self.file_view.get_file_info(file);
        let file_id = file_info.get("fileid")?;
        
        let preview_path = format!("{}/{}/", 
            self.get_thumbnails_folder(), 
            file_id
        );
        
        if !self.user_view.is_dir(&preview_path) {
            return None;
        }
        
        // Does a preview with the wanted height and width already exist?
        let exact_preview = format!("{}{}-{}.png", preview_path, max_x, max_y);
        if self.user_view.file_exists(&exact_preview) {
            return Some(exact_preview);
        }
        
        let wanted_aspect_ratio = max_x as f64 / max_y as f64;
        
        // Map for usable cached thumbnails
        let mut possible_thumbnails: HashMap<u32, String> = HashMap::new();
        
        let all_thumbnails = self.user_view.get_directory_content(&preview_path);
        for thumbnail in all_thumbnails {
            let name = thumbnail.get("name").unwrap().trim_end_matches(".png");
            let size_parts: Vec<&str> = name.split('-').collect();
            if size_parts.len() != 2 {
                continue;
            }
            
            let x = match size_parts[0].parse::<u32>() {
                Ok(x) => x,
                Err(_) => continue,
            };
            
            let y = match size_parts[1].parse::<u32>() {
                Ok(y) => y,
                Err(_) => continue,
            };
            
            let aspect_ratio = x as f64 / y as f64;
            if (aspect_ratio - wanted_aspect_ratio).abs() > 0.0001 {
                continue;
            }
            
            if x < max_x || y < max_y {
                if scaling_up {
                    let scale_factor = max_x as f64 / x as f64;
                    if scale_factor > max_scale_factor as f64 {
                        continue;
                    }
                } else {
                    continue;
                }
            }
            
            possible_thumbnails.insert(x, thumbnail.get("path").unwrap().to_string());
        }
        
        if possible_thumbnails.is_empty() {
            return None;
        }
        
        if possible_thumbnails.len() == 1 {
            return Some(possible_thumbnails.values().next().unwrap().to_string());
        }
        
        // Sort keys
        let mut keys: Vec<u32> = possible_thumbnails.keys().cloned().collect();
        keys.sort();
        
        // Find suitable thumbnail
        if keys[0] > max_x {
            return Some(possible_thumbnails.get(&keys[0]).unwrap().to_string());
        }
        
        if keys[keys.len() - 1] < max_x {
            return Some(possible_thumbnails.get(&keys[keys.len() - 1]).unwrap().to_string());
        }
        
        for width in keys {
            if width >= max_x {
                return Some(possible_thumbnails.get(&width).unwrap().to_string());
            }
        }
        
        None
    }
    
    /// Return a preview of a file
    pub fn get_preview(&mut self) -> Option<&Image> {
        if let Some(ref preview) = self.preview {
            if preview.valid() {
                return Some(preview);
            }
        }
        
        self.preview = None;
        let file = self.get_file();
        let max_x = self.get_max_x();
        let max_y = self.get_max_y();
        let scaling_up = self.get_scaling_up();
        
        let file_info = self.file_view.get_file_info(file);
        let file_id = file_info.get("fileid")?;
        
        if let Some(cached_path) = self.is_cached() {
            if let Ok(content) = self.user_view.file_get_contents(&cached_path) {
                let image = Image::new(&content);
                if image.valid() {
                    self.preview = Some(image);
                    self.resize_and_crop();
                }
            }
        }
        
        if self.preview.is_none() {
            let mimetype = self.file_view.get_mime_type(file);
            
            unsafe {
                for (supported_mime, provider) in PROVIDERS.as_ref().unwrap() {
                    if !regex::Regex::new(supported_mime).unwrap().is_match(&mimetype) {
                        continue;
                    }
                    
                    log::write("core", &format!("Generating preview for \"{}\" with \"{}\"", 
                        file, std::any::type_name::<dyn PreviewProvider>()), log::Level::Debug);
                    
                    if let Some(preview) = provider.get_thumbnail(file, max_x, max_y, scaling_up, &self.file_view) {
                        self.preview = Some(preview);
                        self.resize_and_crop();
                        
                        let preview_path = format!("{}/{}/", self.get_thumbnails_folder(), file_id);
                        let cache_path = format!("{}{}-{}.png", preview_path, max_x, max_y);
                        
                        if !self.user_view.is_dir(&format!("{}/", self.get_thumbnails_folder())) {
                            self.user_view.mkdir(&format!("{}/", self.get_thumbnails_folder()));
                        }
                        
                        if !self.user_view.is_dir(&preview_path) {
                            self.user_view.mkdir(&preview_path);
                        }
                        
                        if let Some(ref preview) = self.preview {
                            self.user_view.file_put_contents(&cache_path, &preview.data());
                        }
                        
                        break;
                    }
                }
            }
        }
        
        if self.preview.is_none() {
            self.preview = Some(Image::new(&[]));
        }
        
        self.preview.as_ref()
    }
    
    /// Show preview
    pub fn show_preview(&mut self) {
        response::enable_caching(3600 * 24); // 24 hours
        if self.preview.is_none() {
            self.get_preview();
        }
        
        if let Some(ref preview) = self.preview {
            preview.show();
        }
    }
    
    /// Show preview (alias for show_preview)
    pub fn show(&mut self) {
        self.show_preview();
    }
    
    /// Resize, crop and fix orientation
    fn resize_and_crop(&mut self) {
        let Some(ref mut image) = self.preview else {
            log::write("core", "self.preview is not an instance of Image", log::Level::Debug);
            return;
        };
        
        let x = self.get_max_x();
        let y = self.get_max_y();
        let scaling_up = self.get_scaling_up();
        let max_scale_factor = self.get_max_scale_factor();
        
        image.fix_orientation();
        
        let real_x = image.width() as u32;
        let real_y = image.height() as u32;
        
        if x == real_x && y == real_y {
            return;
        }
        
        let factor_x = x as f64 / real_x as f64;
        let factor_y = y as f64 / real_y as f64;
        
        let mut factor = if factor_x >= factor_y { factor_x } else { factor_y };
        
        if !scaling_up && factor > 1.0 {
            factor = 1.0;
        }
        
        if factor > max_scale_factor as f64 {
            log::write("core", &format!("scalefactor reduced from {} to {}", factor, max_scale_factor), log::Level::Debug);
            factor = max_scale_factor as f64;
        }
        
        let new_x_size = (real_x as f64 * factor) as u32;
        let new_y_size = (real_y as f64 * factor) as u32;
        
        image.precise_resize(new_x_size, new_y_size);
        
        if new_x_size == x && new_y_size == y {
            return;
        }
        
        if new_x_size >= x && new_y_size >= y {
            let crop_x = ((x as i32 - new_x_size as i32).abs() as f64 * 0.5) as u32;
            // Don't crop previews on the Y axis, this sucks if it's a document.
            let crop_y = 0;
            
            image.crop(crop_x, crop_y, x, y);
            return;
        }
        
        if new_x_size < x || new_y_size < y {
            if new_x_size > x {
                let crop_x = ((new_x_size - x) as f64 * 0.5) as u32;
                image.crop(crop_x, 0, x, new_y_size);
            }
            
            if new_y_size > y {
                let crop_y = ((new_y_size - y) as f64 * 0.5) as u32;
                image.crop(0, crop_y, new_x_size, y);
            }
            
            let new_x_size = image.width() as u32;
            let new_y_size = image.height() as u32;
            
            // Create new image with transparent background
            image.create_background_layer(x, y, (255, 255, 255));
            
            let merge_x = ((x - new_x_size) as f64 * 0.5) as u32;
            let merge_y = ((y - new_y_size) as f64 * 0.5) as u32;
            
            image.merge_with_background(merge_x, merge_y);
        }
    }
    
    /// Register a new preview provider to be used
    pub fn register_provider(class: &str, options: HashMap<String, String>) {
        unsafe {
            let providers = REGISTERED_PROVIDERS.get_or_insert_with(Vec::new);
            providers.push((class.to_string(), options));
        }
    }
    
    /// Create instances of all the registered preview providers
    fn init_providers() {
        INIT_PROVIDERS.call_once(|| {
            unsafe {
                PROVIDERS = Some(HashMap::new());
                
                if !config::get_value::<bool>("enable_previews", Some(true)) {
                    let provider = Box::new(preview::unknown::Unknown::new(HashMap::new())) as Box<dyn PreviewProvider>;
                    PROVIDERS.as_mut().unwrap().insert(provider.get_mime_type().to_string(), provider);
                    return;
                }
                
                let providers = REGISTERED_PROVIDERS.as_ref().unwrap_or(&Vec::new());
                
                for (class, options) in providers {
                    let provider: Box<dyn PreviewProvider> = match class.as_str() {
                        "OC\\Preview\\Image" => Box::new(preview::image::Image::new(options.clone())),
                        "OC\\Preview\\Movies" => Box::new(preview::movies::Movies::new(options.clone())),
                        "OC\\Preview\\MP3" => Box::new(preview::mp3::MP3::new(options.clone())),
                        "OC\\Preview\\PDF" => Box::new(preview::pdf::PDF::new(options.clone())),
                        "OC\\Preview\\SVG" => Box::new(preview::svg::SVG::new(options.clone())),
                        "OC\\Preview\\TXT" => Box::new(preview::txt::TXT::new(options.clone())),
                        "OC\\Preview\\Unknown" => Box::new(preview::unknown::Unknown::new(options.clone())),
                        "OC\\Preview\\Office" => Box::new(preview::office::Office::new(options.clone())),
                        _ => continue,
                    };
                    
                    PROVIDERS.as_mut().unwrap().insert(provider.get_mime_type().to_string(), provider);
                }
                
                // Sort providers by mime type length (descending)
                let mut providers: Vec<(String, Box<dyn PreviewProvider>)> = 
                    PROVIDERS.as_ref().unwrap().iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect();
                
                providers.sort_by(|(a, _), (b, _)| b.len().cmp(&a.len()));
                
                PROVIDERS = Some(providers.into_iter().collect());
            }
        });
    }
    
    /// Hook for after a file is written
    pub fn post_write(args: &HashMap<String, String>) {
        Self::post_delete(args);
    }
    
    /// Hook for after a file is deleted
    pub fn post_delete(args: &HashMap<String, String>) {
        let mut path = args.get("path").unwrap_or(&String::new()).to_string();
        
        if path.starts_with('/') {
            path = path[1..].to_string();
        }
        
        if let Ok(preview) = Preview::new(&user::get_user(), "files/", &path, 1, 1, true) {
            preview.delete_all_previews();
        }
    }
    
    /// Check if a mime type is supported by any preview provider
    pub fn is_mime_supported(mimetype: &str) -> bool {
        if !config::get_value::<bool>("enable_previews", Some(true)) {
            return false;
        }
        
        Self::init_providers();
        
        unsafe {
            // Remove last element because it has the mimetype *
            let providers = PROVIDERS.as_ref().unwrap();
            
            for (supported_mime, _) in providers.iter().take(providers.len() - 1) {
                if regex::Regex::new(supported_mime).unwrap().is_match(mimetype) {
                    return true;
                }
            }
        }
        
        false
    }
}
pub mod office_dash_fallback; // Añadido por reparador automático
