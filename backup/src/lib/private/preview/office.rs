/*
 * Copyright (c) 2013 Georg Ehrke georg@ownCloud.com
 * This file is licensed under the Affero General Public License version 3 or
 * later.
 * See the COPYING-README file.
 */

use std::process::Command;
use std::{env, path::Path};

mod office_cl;
mod office_fallback;

/// Utility function to check if a command exists in PATH
fn command_exists(command: &str) -> bool {
    Command::new("which")
        .arg(command)
        .output()
        .map(|output| !output.stdout.is_empty())
        .unwrap_or(false)
}

/// Initializes the appropriate office preview module based on system capabilities
pub fn initialize_office_preview() -> Result<(), String> {
    // Check if ImageMagick is available (required for both backends)
    if !cfg!(feature = "imagick") {
        return Err("ImageMagick extension is not available".to_string());
    }

    // LibreOffice preview is currently not supported on Windows
    if env::consts::OS != "windows" {
        // Check if LibreOffice or OpenOffice is available
        let libreoffice_available = command_exists("libreoffice");
        let openoffice_available = command_exists("openoffice");
        
        // Get custom LibreOffice path from config if set
        let custom_libreoffice_path = config::get_config_value("preview_libreoffice_path");
        let has_custom_path = custom_libreoffice_path.is_some();

        if libreoffice_available || openoffice_available || has_custom_path {
            // Use the cloud-based LibreOffice implementation
            office_cl::register()?;
        } else {
            // Use fallback implementation
            office_fallback::register()?;
        }
    } else {
        // On Windows, always use fallback implementation
        office_fallback::register()?;
    }

    Ok(())
}

/// Configuration utility module
mod config {
    use std::path::PathBuf;
    
    pub fn get_config_value(key: &str) -> Option<String> {
        // This is a placeholder for the actual config implementation
        // In real code, this would access the application's configuration system
        None
    }
}