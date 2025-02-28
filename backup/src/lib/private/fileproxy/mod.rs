// Módulos generados automáticamente

pub mod fileoperations;

// Contenido fusionado desde src/lib/private/fileproxy.rs
// Copyright (c) 2011 Robin Appelman <icewind1991@gmail.com>
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
// License as published by the Free Software Foundation; either
// version 3 of the License, or any later version.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU AFFERO GENERAL PUBLIC LICENSE for more details.
//
// You should have received a copy of the GNU Affero General Public
// License along with this library.  If not, see <http://www.gnu.org/licenses/>.

use std::any::Any;
use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};

lazy_static::lazy_static! {
    static ref FILE_PROXIES: Mutex<Vec<Arc<dyn FileProxy>>> = Mutex::new(Vec::new());
    static ref ENABLED: Mutex<bool> = Mutex::new(true);
}

/// Trait for manipulating filesystem requests
///
/// Manipulation happens by using 2 kind of proxy operations, pre and post proxies
/// that manipulate the filesystem call and the result of the call respectively
///
/// A pre-proxy receives the filepath as arguments (or 2 filepaths in case of
/// operations like copy or move) and return a boolean
/// If a pre-proxy returns false the file operation will be canceled
/// All filesystem operations have a pre-proxy
///
/// A post-proxy receives 2 arguments, the filepath and the result of the operation.
/// The return value of the post-proxy will be used as the new result of the operation
/// The operations that have a post-proxy are:
/// file_get_contents, is_file, is_dir, file_exists, stat, is_readable,
/// is_writable, filemtime, filectime, file_get_contents,
/// getMimeType, hash, fopen, free_space and search
pub trait FileProxy: Send + Sync {
    /// Fallback function when a proxy operation is not implemented
    fn call_method(&self, method: &str, args: &[&dyn Any]) -> Option<Box<dyn Any>>;

    // Pre-operation methods
    fn pre_mkdir(&self, path: &Path) -> bool {
        true
    }

    fn pre_rmdir(&self, path: &Path) -> bool {
        true
    }

    fn pre_unlink(&self, path: &Path) -> bool {
        true
    }
    
    fn pre_rename(&self, old_path: &Path, new_path: &Path) -> bool {
        true
    }

    fn pre_copy(&self, source: &Path, target: &Path) -> bool {
        true
    }

    fn pre_fopen(&self, path: &Path) -> bool {
        true
    }

    fn pre_touch(&self, path: &Path) -> bool {
        true
    }

    fn pre_file_put_contents(&self, path: &Path) -> bool {
        true
    }

    // Post-operation methods
    fn post_file_get_contents(&self, path: &Path, result: Vec<u8>) -> Vec<u8> {
        result
    }

    fn post_is_file(&self, path: &Path, result: bool) -> bool {
        result
    }

    fn post_is_dir(&self, path: &Path, result: bool) -> bool {
        result
    }

    fn post_file_exists(&self, path: &Path, result: bool) -> bool {
        result
    }

    fn post_stat(&self, path: &Path, result: Option<HashMap<String, i64>>) -> Option<HashMap<String, i64>> {
        result
    }

    fn post_is_readable(&self, path: &Path, result: bool) -> bool {
        result
    }

    fn post_is_writable(&self, path: &Path, result: bool) -> bool {
        result
    }

    fn post_filemtime(&self, path: &Path, result: i64) -> i64 {
        result
    }

    fn post_filectime(&self, path: &Path, result: i64) -> i64 {
        result
    }

    fn post_get_mime_type(&self, path: &Path, result: String) -> String {
        result
    }

    fn post_hash(&self, path: &Path, result: String) -> String {
        result
    }

    fn post_fopen(&self, path: &Path, result: Option<Box<dyn Any>>) -> Option<Box<dyn Any>> {
        result
    }

    fn post_free_space(&self, path: &Path, result: u64) -> u64 {
        result
    }

    fn post_search(&self, path: &Path, result: Vec<String>) -> Vec<String> {
        result
    }
}

/// Class for manipulating filesystem requests
pub struct FileProxyManager;

impl FileProxyManager {
    /// Register a proxy to be used
    pub fn register(proxy: Arc<dyn FileProxy>) {
        let mut proxies = FILE_PROXIES.lock().unwrap();
        proxies.push(proxy);
    }

    /// Get proxies that implement the specified operation
    pub fn get_proxies(operation: &str) -> Vec<Arc<dyn FileProxy>> {
        let proxies = FILE_PROXIES.lock().unwrap();
        proxies.iter()
            .filter_map(|proxy| {
                let method_exists = proxy.call_method(&format!("has_method_{}", operation), &[]).is_some();
                if method_exists {
                    Some(Arc::clone(proxy))
                } else {
                    None
                }
            })
            .collect()
    }

    /// Run pre-operation proxies
    pub fn run_pre_proxies(operation: &str, filepath: &mut Path, filepath2: Option<&mut Path>) -> bool {
        if !*ENABLED.lock().unwrap() {
            return true;
        }

        let operation = format!("pre_{}", operation);
        let proxies = Self::get_proxies(&operation);

        for proxy in proxies {
            match filepath2 {
                Some(path2) => {
                    if proxy.call_method(&operation, &[&filepath, &path2])
                        .and_then(|result| result.downcast::<bool>().ok())
                        .map(|result| *result)
                        .unwrap_or(true) == false {
                        return false;
                    }
                },
                None => {
                    if proxy.call_method(&operation, &[&filepath])
                        .and_then(|result| result.downcast::<bool>().ok())
                        .map(|result| *result)
                        .unwrap_or(true) == false {
                        return false;
                    }
                }
            }
        }
        
        true
    }

    /// Run post-operation proxies
    pub fn run_post_proxies<T: 'static>(operation: &str, path: &Path, result: T) -> T 
    where 
        T: Any + Clone,
    {
        if !*ENABLED.lock().unwrap() {
            return result;
        }

        let operation = format!("post_{}", operation);
        let proxies = Self::get_proxies(&operation);
        
        let mut current_result: Box<dyn Any> = Box::new(result);
        
        for proxy in proxies {
            if let Some(new_result) = proxy.call_method(&operation, &[&path, &current_result]) {
                current_result = new_result;
            }
        }
        
        match current_result.downcast::<T>() {
            Ok(typed_result) => *typed_result,
            Err(_) => panic!("Type mismatch in post-proxy result"),
        }
    }

    /// Clear all registered proxies
    pub fn clear_proxies() {
        let mut proxies = FILE_PROXIES.lock().unwrap();
        proxies.clear();
    }

    /// Enable or disable proxies
    pub fn set_enabled(enabled: bool) {
        let mut proxy_enabled = ENABLED.lock().unwrap();
        *proxy_enabled = enabled;
    }

    /// Check if proxies are enabled
    pub fn is_enabled() -> bool {
        *ENABLED.lock().unwrap()
    }
}