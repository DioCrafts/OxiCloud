//! OwnCloud - VObject Compound Property
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

use sabre_vobject::property::{compound::Compound, Parameter};
use std::string::String;

/// This struct overrides Sabre\VObject\Property::serialize() to not
/// double escape commas and semi-colons in compound properties.
pub struct CompoundProperty {
    inner: Compound,
}

impl CompoundProperty {
    /// Creates a new CompoundProperty wrapper around a Compound property
    pub fn new(compound: Compound) -> Self {
        Self { inner: compound }
    }

    /// Turns the object back into a serialized blob.
    ///
    /// # Returns
    ///
    /// A string representation of the property
    pub fn serialize(&self) -> String {
        let mut str = if let Some(group) = &self.inner.group {
            format!("{}.{}", group, self.inner.name)
        } else {
            self.inner.name.clone()
        };

        for param in &self.inner.parameters {
            str.push(';');
            str.push_str(&param.serialize());
        }

        // Replace newlines with escaped newlines
        let value = self.inner.value.replace('\n', "\\n");
        str.push(':');
        str.push_str(&value);

        let mut out = String::new();
        let mut remaining = str;

        while !remaining.is_empty() {
            if remaining.chars().count() > 75 {
                // Handle UTF-8 aware string slicing - equivalent to mb_strcut
                let mut char_count = 0;
                let mut byte_index = 0;
                
                for (i, c) in remaining.char_indices() {
                    char_count += 1;
                    if char_count > 75 {
                        byte_index = i;
                        break;
                    }
                }
                
                if byte_index == 0 {
                    byte_index = remaining.len();
                }
                
                out.push_str(&remaining[..byte_index]);
                out.push_str("\r\n");
                
                let remainder = &remaining[byte_index..];
                remaining = format!(" {}", remainder);
            } else {
                out.push_str(&remaining);
                out.push_str("\r\n");
                remaining.clear();
            }
        }

        out
    }
    
    /// Returns a reference to the inner Compound
    pub fn inner(&self) -> &Compound {
        &self.inner
    }
    
    /// Returns a mutable reference to the inner Compound
    pub fn inner_mut(&mut self) -> &mut Compound {
        &mut self.inner
    }
}

impl AsRef<Compound> for CompoundProperty {
    fn as_ref(&self) -> &Compound {
        &self.inner
    }
}

impl From<Compound> for CompoundProperty {
    fn from(compound: Compound) -> Self {
        Self::new(compound)
    }
}