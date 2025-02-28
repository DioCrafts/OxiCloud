// ownCloud - App Framework
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

use regex::Regex;
use std::any::Any;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct MethodAnnotationError {
    message: String,
}

impl MethodAnnotationError {
    fn new(message: &str) -> Self {
        MethodAnnotationError {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for MethodAnnotationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Method annotation error: {}", self.message)
    }
}

impl Error for MethodAnnotationError {}

/// Reads and parses annotations from doc comments
pub struct MethodAnnotationReader {
    annotations: Vec<String>,
}

impl MethodAnnotationReader {
    /// Creates a new MethodAnnotationReader for the specified object and method
    ///
    /// # Arguments
    ///
    /// * `object` - An object or type that implements the specified method
    /// * `method_name` - The name of the method to inspect for annotations
    ///
    /// # Returns
    ///
    /// A Result containing the MethodAnnotationReader or an error
    pub fn new<T: Any>(object: &T, method_name: &str) -> Result<Self, Box<dyn Error>> {
        // In Rust we can't directly use reflection like in PHP
        // This is a simplified implementation that assumes doc comments are passed externally
        // In a real implementation, this would likely use the `rustdoc` crate or proc macros
        
        // We're simulating getting the doc comment as this functionality 
        // requires proc macros or external tooling in Rust
        let doc_comment = Self::get_doc_comment_for_method::<T>(object, method_name)?;
        
        // Parse annotations using regex
        let re = Regex::new(r"@([A-Z]\w+)")?;
        let mut annotations = Vec::new();
        
        for cap in re.captures_iter(&doc_comment) {
            if let Some(annotation) = cap.get(1) {
                annotations.push(annotation.as_str().to_string());
            }
        }
        
        Ok(MethodAnnotationReader { annotations })
    }
    
    /// Simulates retrieving a doc comment for a method
    /// 
    /// In a real implementation, this would use procedural macros or external tools
    fn get_doc_comment_for_method<T: Any>(_object: &T, method_name: &str) -> Result<String, Box<dyn Error>> {
        // This is a placeholder - in a real implementation you would need to use
        // procedural macros or external tooling to extract actual doc comments
        Err(Box::new(MethodAnnotationError::new(&format!(
            "Cannot retrieve doc comment for method {} using runtime reflection in Rust",
            method_name
        ))))
    }

    /// Check if a method contains an annotation
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the annotation to check for
    ///
    /// # Returns
    ///
    /// `true` if the annotation is found, `false` otherwise
    pub fn has_annotation(&self, name: &str) -> bool {
        self.annotations.iter().any(|a| a == name)
    }
}

// For testing purposes, a version that can be constructed with predefined annotations
#[cfg(test)]
impl MethodAnnotationReader {
    pub fn with_annotations(annotations: Vec<String>) -> Self {
        MethodAnnotationReader { annotations }
    }
}