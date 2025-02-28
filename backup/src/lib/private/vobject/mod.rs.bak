// Módulos generados automáticamente

pub mod stringproperty;
pub mod compoundproperty;

// Contenido fusionado desde src/lib/private/vobject.rs
// Copyright notice
// ownCloud
//
// @author Bart Visscher
// @copyright 2011 Bart Visscher bartv@thisnet.nl
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

use std::collections::HashMap;
use chrono::{DateTime, Local};
use log::error;
use rand::{Rng, thread_rng};
use std::time::{SystemTime, UNIX_EPOCH};
use std::ops::{Deref, DerefMut};

// These would be actual imports to Sabre equivalent libraries in Rust
// For now, they're mock types to demonstrate structure
pub mod sabre {
    pub mod vobject {
        use std::collections::HashMap;
        use chrono::{DateTime, Local};

        #[derive(Clone, Debug)]
        pub struct Component {
            pub name: String,
            pub children: Vec<Component>,
            pub properties: HashMap<String, Property>,
        }

        impl Component {
            pub fn new(name: &str) -> Self {
                Self {
                    name: name.to_owned(),
                    children: Vec::new(),
                    properties: HashMap::new(),
                }
            }

            pub fn add(&mut self, item: impl Into<ComponentItem>, item_value: Option<String>) {
                // Implementation would go here
            }
        }

        pub enum ComponentItem {
            Component(Component),
            Property(Property),
        }

        impl From<Component> for ComponentItem {
            fn from(comp: Component) -> Self {
                ComponentItem::Component(comp)
            }
        }

        impl From<Property> for ComponentItem {
            fn from(prop: Property) -> Self {
                ComponentItem::Property(prop)
            }
        }

        #[derive(Clone, Debug)]
        pub struct Property {
            pub name: String,
            pub value: String,
            pub parameters: Vec<Parameter>,
        }

        impl Property {
            pub fn new(name: &str, value: impl Into<String>) -> Self {
                Self {
                    name: name.to_owned(),
                    value: value.into(),
                    parameters: Vec::new(),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Parameter {
            pub name: String,
            pub value: String,
        }

        impl Parameter {
            pub fn new(name: &str, value: impl Into<String>) -> Self {
                Self {
                    name: name.to_owned(),
                    value: value.into(),
                }
            }
        }

        pub struct Reader;

        impl Reader {
            pub fn read(data: &str) -> Result<Component, Box<dyn std::error::Error>> {
                // Implementation would go here
                Ok(Component::new("MOCK"))
            }
        }

        pub mod property {
            pub mod datetime {
                pub const LOCALTZ: i32 = 0;
            }

            #[derive(Clone, Debug)]
            pub struct DateTime {
                pub name: String,
                pub value: String,
            }

            impl DateTime {
                pub fn new(name: &str) -> Self {
                    Self {
                        name: name.to_owned(),
                        value: String::new(),
                    }
                }

                pub fn set_date_time(&mut self, date_time: chrono::DateTime<chrono::Local>, date_type: i32) {
                    // Implementation would go here
                }
            }
        }
    }
}

/// This struct provides a streamlined interface to the Sabre VObject classes
pub struct OcVObject {
    vobject: sabre::vobject::Component,
}

impl OcVObject {
    /// Returns the inner VObject
    pub fn get_vobject(&self) -> &sabre::vobject::Component {
        &self.vobject
    }

    /// Parses the VObject
    /// 
    /// # Arguments
    /// * `data` - VObject as string
    /// 
    /// # Returns
    /// * `Option<OcVObject>` - Parsed VObject or None on error
    pub fn parse(data: &str) -> Option<OcVObject> {
        // This would set up equivalent to PHP's class map
        // Sabre\VObject\Property::$classMap['LAST-MODIFIED'] = 'Sabre\VObject\Property\DateTime';
        
        match sabre::vobject::Reader::read(data) {
            Ok(vobject) => Some(OcVObject::new(vobject)),
            Err(e) => {
                error!("vobject: {}", e);
                None
            }
        }
    }

    /// Escapes semicolons in the values
    /// 
    /// # Arguments
    /// * `value` - Values to escape
    /// 
    /// # Returns
    /// * `String` - String with escaped semicolons
    pub fn escape_semicolons(value: &[String]) -> String {
        let mut escaped = value.to_vec();
        for i in &mut escaped {
            *i = i.replace(";", "\\\\;");
        }
        escaped.join(";")
    }

    /// Creates an array out of a multivalue property
    /// 
    /// # Arguments
    /// * `value` - String with potentially escaped semicolons
    /// 
    /// # Returns
    /// * `Vec<String>` - Array of values
    pub fn unescape_semicolons(value: &str) -> Vec<String> {
        let mut result = Vec::new();
        let mut parts = value.split(';').collect::<Vec<_>>();
        
        let mut i = 0;
        while i < parts.len() {
            let mut current = parts[i].to_string();
            
            if current.ends_with("\\\\") {
                if i + 1 < parts.len() {
                    current = format!("{}{};{}", 
                        &current[..current.len()-2], 
                        ";", 
                        parts[i+1]);
                    parts.remove(i+1);
                    i -= 1;
                } else {
                    current = format!("{}{}", &current[..current.len()-2], ";");
                }
            }
            
            result.push(current);
            i += 1;
        }
        
        result
    }

    /// Constructor
    /// 
    /// # Arguments
    /// * `vobject_or_name` - Either a Component or a string name
    pub fn new<T: Into<VObjectInput>>(vobject_or_name: T) -> Self {
        let vobj = match vobject_or_name.into() {
            VObjectInput::Component(component) => component,
            VObjectInput::Name(name) => sabre::vobject::Component::new(&name),
        };
        
        Self { vobject: vobj }
    }

    /// Add an item to the vobject
    pub fn add<T: Into<sabre::vobject::ComponentItem>>(&mut self, item: T, item_value: Option<String>) {
        self.vobject.add(item, item_value);
    }

    /// Add property to vobject
    /// 
    /// # Arguments
    /// * `name` - Name of property
    /// * `value` - Value of property
    /// * `parameters` - Parameters of property
    /// 
    /// # Returns
    /// * `sabre::vobject::Property` - Newly created property
    pub fn add_property(
        &mut self, 
        name: &str, 
        value: impl Into<PropertyValue>, 
        parameters: HashMap<String, String>
    ) -> sabre::vobject::Property {
        let prop_value = match value.into() {
            PropertyValue::String(s) => s,
            PropertyValue::Array(arr) => Self::escape_semicolons(&arr),
        };
        
        let property = sabre::vobject::Property::new(name, prop_value);
        
        // Add the property with its parameters
        // Implementation depends on actual Sabre equivalent

        property
    }

    /// Set a UID for the vobject
    pub fn set_uid(&mut self) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        let random_number: u32 = thread_rng().gen();
        let uid = format!("{:x}", md5::compute(format!("{}{}", random_number, timestamp)))[..10].to_string();
        
        // Add UID property
        // self.vobject.add('UID', uid);
    }

    /// Set a string property
    pub fn set_string(&mut self, name: &str, string: &str) {
        // Implementation depends on actual Sabre equivalent
        // For now this is a placeholder to show the structure
    }

    /// Sets or unsets the Date and Time for a property.
    /// When `datetime` is set to 'now', use the current time
    /// When `datetime` is None, unset the property
    pub fn set_date_time(
        &mut self, 
        name: &str, 
        datetime: Option<DateTimeValue>, 
        date_type: i32
    ) {
        // Implementation depends on actual Sabre equivalent
        // For now this is a placeholder to show the structure
    }

    /// Get property value as string
    pub fn get_as_string(&self, name: &str) -> String {
        // Implementation depends on actual Sabre equivalent
        String::new()
    }

    /// Get property value as array
    pub fn get_as_array(&self, name: &str) -> Vec<String> {
        if let Some(value) = self.vobject.properties.get(name) {
            value.value
                .split(',')
                .map(|s| s.trim().to_string())
                .collect()
        } else {
            Vec::new()
        }
    }
}

// Input types for constructor
pub enum VObjectInput {
    Component(sabre::vobject::Component),
    Name(String),
}

impl From<sabre::vobject::Component> for VObjectInput {
    fn from(component: sabre::vobject::Component) -> Self {
        VObjectInput::Component(component)
    }
}

impl From<&str> for VObjectInput {
    fn from(name: &str) -> Self {
        VObjectInput::Name(name.to_string())
    }
}

impl From<String> for VObjectInput {
    fn from(name: String) -> Self {
        VObjectInput::Name(name)
    }
}

// Property value types
pub enum PropertyValue {
    String(String),
    Array(Vec<String>),
}

impl From<String> for PropertyValue {
    fn from(s: String) -> Self {
        PropertyValue::String(s)
    }
}

impl From<&str> for PropertyValue {
    fn from(s: &str) -> Self {
        PropertyValue::String(s.to_string())
    }
}

impl From<Vec<String>> for PropertyValue {
    fn from(arr: Vec<String>) -> Self {
        PropertyValue::Array(arr)
    }
}

// DateTime value types
pub enum DateTimeValue {
    Now,
    DateTime(DateTime<Local>),
}

impl From<&str> for DateTimeValue {
    fn from(s: &str) -> Self {
        if s == "now" {
            DateTimeValue::Now
        } else {
            // For simplicity, we'll just return Now
            // In a real implementation, you'd parse the string
            DateTimeValue::Now
        }
    }
}

impl From<DateTime<Local>> for DateTimeValue {
    fn from(dt: DateTime<Local>) -> Self {
        DateTimeValue::DateTime(dt)
    }
}

// Implementation of Deref and DerefMut to allow accessing vobject's fields directly
impl Deref for OcVObject {
    type Target = sabre::vobject::Component;

    fn deref(&self) -> &Self::Target {
        &self.vobject
    }
}

impl DerefMut for OcVObject {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.vobject
    }
}