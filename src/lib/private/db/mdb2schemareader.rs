use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use doctrine_dbal_schema::{Column, Index, Platform, Schema, Table};
use serde_xml_rs::from_reader;
use thiserror::Error;

/// Error types for MDB2SchemaReader
#[derive(Error, Debug)]
pub enum MDB2SchemaReaderError {
    #[error("Domain error: {0}")]
    DomainError(String),
    
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("XML parsing error: {0}")]
    XmlError(#[from] serde_xml_rs::Error),
}

/// Reader for MDB2 schema XML files
///
/// Reads XML schema files and creates the corresponding Schema objects
pub struct MDB2SchemaReader {
    /// Database name
    db_name: String,
    
    /// Database table prefix
    db_table_prefix: String,
    
    /// DBAL Platform to use
    platform: Box<dyn Platform>,
}

impl MDB2SchemaReader {
    /// Create a new MDB2SchemaReader with the given configuration and platform
    pub fn new(config: &Config, platform: Box<dyn Platform>) -> Self {
        Self {
            platform,
            db_name: config.get_value("dbname", "owncloud".to_string()),
            db_table_prefix: config.get_value("dbtableprefix", "oc_".to_string()),
        }
    }

    /// Load a schema from a file
    ///
    /// # Arguments
    ///
    /// * `file_path` - Path to the XML schema file
    ///
    /// # Returns
    ///
    /// Result containing the Schema or an error
    pub fn load_schema_from_file<P: AsRef<Path>>(&self, file_path: P) -> Result<Schema, MDB2SchemaReaderError> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        
        let xml: XmlSchema = from_reader(contents.as_bytes())?;
        let mut schema = Schema::new();
        
        for element in xml.elements {
            match element {
                XmlElement::Name(_) | XmlElement::Create(_) | XmlElement::Overwrite(_) | XmlElement::Charset(_) => {
                    // These elements are ignored
                },
                XmlElement::Table(table_xml) => {
                    self.load_table(&mut schema, table_xml)?;
                },
                XmlElement::Unknown(name) => {
                    return Err(MDB2SchemaReaderError::DomainError(format!("Unknown element: {}", name)));
                }
            }
        }
        
        Ok(schema)
    }

    /// Load a table definition from XML into the schema
    fn load_table(&self, schema: &mut Schema, xml: TableXml) -> Result<(), MDB2SchemaReaderError> {
        let mut table = None;
        
        for element in xml.elements {
            match element {
                TableElement::Name(name) => {
                    let name = name.replace("*dbprefix*", &self.db_table_prefix);
                    let name = self.platform.quote_identifier(&name);
                    table = Some(schema.create_table(&name));
                },
                TableElement::Create(_) | TableElement::Overwrite(_) | TableElement::Charset(_) => {
                    // These elements are ignored
                },
                TableElement::Declaration(declaration) => {
                    if let Some(table) = table.as_mut() {
                        self.load_declaration(table, declaration)?;
                    } else {
                        return Err(MDB2SchemaReaderError::DomainError("Table declaration before table name".to_string()));
                    }
                },
                TableElement::Unknown(name) => {
                    return Err(MDB2SchemaReaderError::DomainError(format!("Unknown element: {}", name)));
                }
            }
        }
        
        Ok(())
    }

    /// Load a declaration into a table
    fn load_declaration(&self, table: &mut Table, xml: DeclarationXml) -> Result<(), MDB2SchemaReaderError> {
        for element in xml.elements {
            match element {
                DeclarationElement::Field(field) => {
                    self.load_field(table, field)?;
                },
                DeclarationElement::Index(index) => {
                    self.load_index(table, index)?;
                },
                DeclarationElement::Unknown(name) => {
                    return Err(MDB2SchemaReaderError::DomainError(format!("Unknown element: {}", name)));
                }
            }
        }
        
        Ok(())
    }

    /// Load a field definition into a table
    fn load_field(&self, table: &mut Table, xml: FieldXml) -> Result<(), MDB2SchemaReaderError> {
        let mut options = HashMap::new();
        let mut name = None;
        let mut field_type = None;
        
        for element in xml.elements {
            match element {
                FieldElement::Name(value) => {
                    let quoted_name = self.platform.quote_identifier(&value);
                    name = Some(quoted_name);
                },
                FieldElement::Type(value) => {
                    let converted_type = match value.as_str() {
                        "text" => "string",
                        "clob" => "text",
                        "timestamp" => "datetime",
                        other => other,
                    };
                    field_type = Some(converted_type.to_string());
                },
                FieldElement::Length(value) => {
                    options.insert("length".to_string(), value);
                },
                FieldElement::Unsigned(value) => {
                    options.insert("unsigned".to_string(), self.as_bool(&value).to_string());
                },
                FieldElement::NotNull(value) => {
                    options.insert("notnull".to_string(), self.as_bool(&value).to_string());
                },
                FieldElement::AutoIncrement(value) => {
                    options.insert("autoincrement".to_string(), self.as_bool(&value).to_string());
                },
                FieldElement::Default(value) => {
                    options.insert("default".to_string(), value);
                },
                FieldElement::Comments(value) => {
                    options.insert("comment".to_string(), value);
                },
                FieldElement::Primary(value) => {
                    options.insert("primary".to_string(), self.as_bool(&value).to_string());
                },
                FieldElement::Unknown(name) => {
                    return Err(MDB2SchemaReaderError::DomainError(format!("Unknown element: {}", name)));
                }
            }
        }
        
        if let (Some(name), Some(type_name)) = (name, field_type) {
            // Process default values
            if options.get("default").map_or(true, |v| v.is_empty()) {
                if options.get("notnull").map_or(true, |v| v == "false") {
                    options.remove("default");
                    options.insert("notnull".to_string(), "false".to_string());
                } else {
                    options.insert("default".to_string(), "".to_string());
                }
                
                if type_name == "integer" {
                    options.insert("default".to_string(), "0".to_string());
                } else if type_name == "boolean" {
                    options.insert("default".to_string(), "false".to_string());
                }
                
                if options.get("autoincrement").map_or(false, |v| v == "true") {
                    options.remove("default");
                }
            }
            
            // Convert default values for specific types
            if type_name == "integer" && options.contains_key("default") {
                if let Some(default) = options.get("default") {
                    if let Ok(value) = default.parse::<i32>() {
                        options.insert("default".to_string(), value.to_string());
                    }
                }
            }
            
            // Adjust integer type based on length
            let mut final_type = type_name.clone();
            if type_name == "integer" {
                if let Some(length) = options.get("length") {
                    if let Ok(length_value) = length.parse::<i32>() {
                        if length_value < 4 {
                            final_type = "smallint".to_string();
                        } else if length_value > 4 {
                            final_type = "bigint".to_string();
                        }
                    }
                }
            }
            
            // Convert boolean defaults
            if type_name == "boolean" && options.contains_key("default") {
                if let Some(default) = options.get("default") {
                    let bool_value = self.as_bool(default);
                    options.insert("default".to_string(), bool_value.to_string());
                }
            }
            
            // Set primary key for autoincrement + notnull columns
            if options.get("autoincrement").map_or(false, |v| v == "true") &&
               options.get("notnull").map_or(false, |v| v == "true") {
                options.insert("primary".to_string(), "true".to_string());
            }
            
            // Add the column to the table
            table.add_column(&name, &final_type, options);
            
            // If this is a primary key, set it
            if options.get("primary").map_or(false, |v| v == "true") {
                table.set_primary_key(&[name]);
            }
        }
        
        Ok(())
    }

    /// Load index definition into a table
    fn load_index(&self, table: &mut Table, xml: IndexXml) -> Result<(), MDB2SchemaReaderError> {
        let mut name = None;
        let mut primary = false;
        let mut unique = false;
        let mut fields = Vec::new();
        
        for element in xml.elements {
            match element {
                IndexElement::Name(value) => {
                    name = Some(value);
                },
                IndexElement::Primary(value) => {
                    primary = self.as_bool(&value);
                },
                IndexElement::Unique(value) => {
                    unique = self.as_bool(&value);
                },
                IndexElement::Field(field_xml) => {
                    for field_element in field_xml.elements {
                        match field_element {
                            FieldRefElement::Name(field_name) => {
                                let quoted_name = self.platform.quote_identifier(&field_name);
                                fields.push(quoted_name);
                            },
                            FieldRefElement::Sorting(_) => {
                                // Sorting is ignored
                            },
                            FieldRefElement::Unknown(name) => {
                                return Err(MDB2SchemaReaderError::DomainError(format!("Unknown element: {}", name)));
                            }
                        }
                    }
                },
                IndexElement::Unknown(name) => {
                    return Err(MDB2SchemaReaderError::DomainError(format!("Unknown element: {}", name)));
                }
            }
        }
        
        if !fields.is_empty() {
            if primary {
                table.set_primary_key(&fields, name.as_deref());
            } else if unique {
                table.add_unique_index(&fields, name.as_deref());
            } else {
                table.add_index(&fields, name.as_deref());
            }
        } else {
            return Err(MDB2SchemaReaderError::DomainError(format!("Empty index definition: {:?}", name)));
        }
        
        Ok(())
    }

    /// Convert a string to boolean
    fn as_bool<S: AsRef<str>>(&self, value: S) -> bool {
        match value.as_ref() {
            "true" => true,
            "false" => false,
            other => other.parse::<bool>().unwrap_or(false),
        }
    }
}

/// Configuration trait for accessing config values
pub trait Config {
    fn get_value<T: ToString>(&self, key: &str, default: T) -> String;
}

// XML data structures for parsing the schema

#[derive(Debug, Deserialize)]
struct XmlSchema {
    elements: Vec<XmlElement>,
}

#[derive(Debug, Deserialize)]
enum XmlElement {
    Name(String),
    Create(String),
    Overwrite(String),
    Charset(String),
    Table(TableXml),
    Unknown(String),
}

#[derive(Debug, Deserialize)]
struct TableXml {
    elements: Vec<TableElement>,
}

#[derive(Debug, Deserialize)]
enum TableElement {
    Name(String),
    Create(String),
    Overwrite(String),
    Charset(String),
    Declaration(DeclarationXml),
    Unknown(String),
}

#[derive(Debug, Deserialize)]
struct DeclarationXml {
    elements: Vec<DeclarationElement>,
}

#[derive(Debug, Deserialize)]
enum DeclarationElement {
    Field(FieldXml),
    Index(IndexXml),
    Unknown(String),
}

#[derive(Debug, Deserialize)]
struct FieldXml {
    elements: Vec<FieldElement>,
}

#[derive(Debug, Deserialize)]
enum FieldElement {
    Name(String),
    Type(String),
    Length(String),
    Unsigned(String),
    NotNull(String),
    AutoIncrement(String),
    Default(String),
    Comments(String),
    Primary(String),
    Unknown(String),
}

#[derive(Debug, Deserialize)]
struct IndexXml {
    elements: Vec<IndexElement>,
}

#[derive(Debug, Deserialize)]
enum IndexElement {
    Name(String),
    Primary(String),
    Unique(String),
    Field(FieldRefXml),
    Unknown(String),
}

#[derive(Debug, Deserialize)]
struct FieldRefXml {
    elements: Vec<FieldRefElement>,
}

#[derive(Debug, Deserialize)]
enum FieldRefElement {
    Name(String),
    Sorting(String),
    Unknown(String),
}