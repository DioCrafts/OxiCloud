//! # VObject String Property
//!
//! This class adds escaping of simple string properties.
//!
//! This library is free software; you can redistribute it and/or
//! modify it under the terms of the GNU AFFERO GENERAL PUBLIC LICENSE
//! License as published by the Free Software Foundation; either
//! version 3 of the License, or any later version.
//!
//! This library is distributed in the hope that it will be useful,
//! but WITHOUT ANY WARRANTY; without even the implied warranty of
//! MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//! GNU AFFERO GENERAL PUBLIC LICENSE for more details.
//!
//! You should have received a copy of the GNU Affero General Public
//! License along with this library.  If not, see <http://www.gnu.org/licenses/>.

use crate::sabre::vobject::{Parameter, Property};

/// This struct overrides Property::serialize() to properly
/// escape commas and semi-colons in string properties.
pub struct StringProperty {
    /// The base Property this extends
    inner: Property,
}

impl StringProperty {
    /// Create a new StringProperty from a base Property
    pub fn new(property: Property) -> Self {
        Self { inner: property }
    }

    /// Turns the object back into a serialized blob.
    pub fn serialize(&self) -> String {
        let mut str = String::new();
        
        // Add name with optional group prefix
        if let Some(group) = &self.inner.group {
            str.push_str(&format!("{}.", group));
        }
        str.push_str(&self.inner.name);
        
        // Add parameters
        for param in &self.inner.parameters {
            str.push(';');
            str.push_str(&param.serialize());
        }
        
        // Process and escape value
        let src = [r"\", "\n", ";", ","];
        let out = [r"\\", r"\n", r"\;", r"\,"];
        
        let value = self.inner.value.replace(r"\,", ",")
                                    .replace(r"\;", ";")
                                    .replace(r"\\", r"\");
        
        str.push(':');
        
        let mut escaped_value = value;
        for (i, s) in src.iter().enumerate() {
            escaped_value = escaped_value.replace(s, out[i]);
        }
        str.push_str(&escaped_value);
        
        // Handle line folding
        let mut result = String::new();
        let mut remaining = str;
        
        while !remaining.is_empty() {
            if remaining.len() > 75 {
                let (line, rest) = split_at_char_boundary(&remaining, 75);
                result.push_str(line);
                result.push_str("\r\n");
                remaining = format!(" {}", rest);
            } else {
                result.push_str(&remaining);
                result.push_str("\r\n");
                break;
            }
        }
        
        result
    }
}

// Helper function to safely split a string at a UTF-8 char boundary
fn split_at_char_boundary(s: &str, mid: usize) -> (&str, &str) {
    if mid >= s.len() {
        return (s, "");
    }
    
    let mut end = mid;
    while !s.is_char_boundary(end) {
        end -= 1;
    }
    
    s.split_at(end)
}

// Implementation to delegate methods to the inner Property
impl std::ops::Deref for StringProperty {
    type Target = Property;
    
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl std::ops::DerefMut for StringProperty {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}