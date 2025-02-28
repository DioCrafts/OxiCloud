//! Gestión de plantillas para la UI

use anyhow::{Result, Context as AnyhowContext};
use log::{info, warn};
use tera::{Tera, Function, Value, from_value, to_value};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use humansize::{format_size, BINARY};

/// Inicializa el motor de plantillas con funciones personalizadas
pub fn init_templates() -> Result<Tera> {
    // Crear instancia de Tera
    let mut tera = Tera::new("templates/**/*.html")
        .context("Error al cargar plantillas")?;
    
    // Registrar funciones personalizadas
    tera.register_function("format_date", format_date_function());
    tera.register_function("format_size", format_size_function());
    tera.register_function("get_file_icon", get_file_icon_function());
    
    info!("Motor de plantillas inicializado");
    Ok(tera)
}

/// Función para formatear fechas
fn format_date_function() -> impl Function {
    Box::new(move |args: &HashMap<String, Value>| -> tera::Result<Value> {
        let date = match args.get("date") {
            Some(date) => match from_value::<DateTime<Utc>>(date.clone()) {
                Ok(date) => date,
                Err(_) => {
                    return Err(tera::Error::msg("El argumento 'date' debe ser una fecha válida"));
                }
            },
            None => {
                return Err(tera::Error::msg("La función format_date requiere un argumento 'date'"));
            }
        };
        
        let format = args.get("format")
            .and_then(|f| f.as_str())
            .unwrap_or("%d/%m/%Y %H:%M");
        
        let formatted = date.format(format).to_string();
        Ok(to_value(formatted)?)
    })
}

/// Función para formatear tamaños de archivo en unidades legibles
fn format_size_function() -> impl Function {
    Box::new(move |args: &HashMap<String, Value>| -> tera::Result<Value> {
        let size = match args.get("size") {
            Some(size) => match from_value::<i64>(size.clone()) {
                Ok(size) => size,
                Err(_) => {
                    return Err(tera::Error::msg("El argumento 'size' debe ser un número"));
                }
            },
            None => {
                return Err(tera::Error::msg("La función format_size requiere un argumento 'size'"));
            }
        };
        
        if size == 0 {
            return Ok(to_value("0 B")?);
        }
        
        let formatted = format_size(size as u64, BINARY);
        Ok(to_value(formatted)?)
    })
}

/// Función para obtener el icono CSS según el tipo de archivo
fn get_file_icon_function() -> impl Function {
    Box::new(move |args: &HashMap<String, Value>| -> tera::Result<Value> {
        let mime_type = match args.get("mime_type") {
            Some(mime) => match from_value::<String>(mime.clone()) {
                Ok(mime) => mime,
                Err(_) => {
                    return Err(tera::Error::msg("El argumento 'mime_type' debe ser una cadena"));
                }
            },
            None => {
                return Err(tera::Error::msg("La función get_file_icon requiere un argumento 'mime_type'"));
            }
        };
        
        let is_directory = args.get("is_directory")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        
        let icon = if is_directory {
            "fa-folder"
        } else if mime_type.starts_with("image/") {
            "fa-file-image"
        } else if mime_type.starts_with("video/") {
            "fa-file-video"
        } else if mime_type.starts_with("audio/") {
            "fa-file-audio"
        } else if mime_type.starts_with("text/") {
            "fa-file-alt"
        } else if mime_type == "application/pdf" {
            "fa-file-pdf"
        } else if mime_type.contains("spreadsheet") || mime_type.contains("excel") {
            "fa-file-excel"
        } else if mime_type.contains("presentation") || mime_type.contains("powerpoint") {
            "fa-file-powerpoint"
        } else if mime_type.contains("document") || mime_type.contains("word") {
            "fa-file-word"
        } else if mime_type.contains("zip") || mime_type.contains("compressed") {
            "fa-file-archive"
        } else {
            "fa-file"
        };
        
        Ok(to_value(icon)?)
    })
}