/**
 * Copyright (c) 2012 Bart Visscher <bartv@thisnet.nl>
 * This file is licensed under the Affero General Public License version 3 or
 * later.
 * See the COPYING-README file.
 */

use std::fs;
use std::io::{self, Write};
use std::path::Path;
use quick_xml::{Writer, Element, EventWriter};
use database_schema::{Table, Column, Index, SchemaManager};
use config::Config;

pub struct Mdb2SchemaWriter;

impl Mdb2SchemaWriter {
    /// Saves the database schema to an XML file
    ///
    /// # Arguments
    ///
    /// * `file` - Path to the file where the schema will be saved
    /// * `sm` - Schema manager instance containing the database schema
    ///
    /// # Returns
    ///
    /// Returns a Result indicating success or failure
    pub fn save_schema_to_file<P: AsRef<Path>>(file: P, sm: &dyn SchemaManager) -> io::Result<()> {
        let mut xml = Element::new("database");
        
        let db_name = Config::get_value("dbname").unwrap_or_else(|| "owncloud".to_string());
        xml.add_child(Element::new("name").text(db_name));
        xml.add_child(Element::new("create").text("true"));
        xml.add_child(Element::new("overwrite").text("false"));
        xml.add_child(Element::new("charset").text("utf8"));
        
        for table in sm.list_tables() {
            let mut table_element = Element::new("table");
            Self::save_table(&table, &mut table_element);
            xml.add_child(table_element);
        }
        
        let xml_string = quick_xml::se::to_string(&xml)?;
        fs::write(file, xml_string)?;
        
        Ok(())
    }

    fn save_table(table: &Table, xml: &mut Element) {
        xml.add_child(Element::new("name").text(table.get_name()));
        
        let mut declaration = Element::new("declaration");
        
        for column in table.get_columns() {
            let mut field = Element::new("field");
            Self::save_column(&column, &mut field);
            declaration.add_child(field);
        }
        
        for index in table.get_indexes() {
            if index.get_name() == "PRIMARY" {
                let mut autoincrement = false;
                for column_name in index.get_columns() {
                    if let Some(column) = table.get_column(column_name) {
                        if column.is_autoincrement() {
                            autoincrement = true;
                            break;
                        }
                    }
                }
                
                if autoincrement {
                    continue;
                }
            }
            
            let mut index_element = Element::new("index");
            Self::save_index(&index, &mut index_element);
            declaration.add_child(index_element);
        }
        
        xml.add_child(declaration);
    }

    fn save_column(column: &Column, xml: &mut Element) {
        xml.add_child(Element::new("name").text(column.get_name()));
        
        match column.get_type() {
            "SmallInt" | "Integer" | "BigInt" => {
                xml.add_child(Element::new("type").text("integer"));
                
                let default = if column.get_default().is_none() && column.is_autoincrement() {
                    "0".to_string()
                } else {
                    column.get_default().unwrap_or_default()
                };
                
                xml.add_child(Element::new("default").text(default));
                xml.add_child(Element::new("notnull").text(Self::to_bool(column.is_notnull())));
                
                if column.is_autoincrement() {
                    xml.add_child(Element::new("autoincrement").text("1"));
                }
                
                if column.is_unsigned() {
                    xml.add_child(Element::new("unsigned").text("true"));
                }
                
                let length = match column.get_type() {
                    "SmallInt" => "2",
                    "BigInt" => "8",
                    _ => "4",
                };
                
                xml.add_child(Element::new("length").text(length));
            },
            "String" => {
                xml.add_child(Element::new("type").text("text"));
                
                let default = column.get_default().unwrap_or_default();
                let default = if default.trim().is_empty() {
                    "false".to_string()
                } else {
                    default
                };
                
                xml.add_child(Element::new("default").text(default));
                xml.add_child(Element::new("notnull").text(Self::to_bool(column.is_notnull())));
                xml.add_child(Element::new("length").text(column.get_length().to_string()));
            },
            "Text" => {
                xml.add_child(Element::new("type").text("clob"));
                xml.add_child(Element::new("notnull").text(Self::to_bool(column.is_notnull())));
            },
            "Decimal" => {
                xml.add_child(Element::new("type").text("decimal"));
                xml.add_child(Element::new("default").text(column.get_default().unwrap_or_default()));
                xml.add_child(Element::new("notnull").text(Self::to_bool(column.is_notnull())));
                xml.add_child(Element::new("length").text("15"));
            },
            "Boolean" => {
                xml.add_child(Element::new("type").text("integer"));
                xml.add_child(Element::new("default").text(column.get_default().unwrap_or_default()));
                xml.add_child(Element::new("notnull").text(Self::to_bool(column.is_notnull())));
                xml.add_child(Element::new("length").text("1"));
            },
            "DateTime" => {
                xml.add_child(Element::new("type").text("timestamp"));
                xml.add_child(Element::new("default").text(column.get_default().unwrap_or_default()));
                xml.add_child(Element::new("notnull").text(Self::to_bool(column.is_notnull())));
            },
            _ => {
                // Handle unknown types or leave empty
            }
        }
    }

    fn save_index(index: &Index, xml: &mut Element) {
        xml.add_child(Element::new("name").text(index.get_name()));
        
        if index.is_primary() {
            xml.add_child(Element::new("primary").text("true"));
        } else if index.is_unique() {
            xml.add_child(Element::new("unique").text("true"));
        }
        
        for column in index.get_columns() {
            let mut field = Element::new("field");
            field.add_child(Element::new("name").text(column));
            field.add_child(Element::new("sorting").text("ascending"));
            xml.add_child(field);
        }
    }

    fn to_bool(value: bool) -> &'static str {
        if value { "true" } else { "false" }
    }
}