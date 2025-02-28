use std::collections::HashMap;
use phf::phf_map;

// English (Great Britain) translations
pub static EN_GB_TRANSLATIONS: phf::Map<&'static str, &'static str> = phf_map! {
    "Access granted" => "Access granted",
    "Error configuring Dropbox storage" => "Error configuring Dropbox storage",
    "Grant access" => "Grant access",
    "Please provide a valid Dropbox app key and secret." => "Please provide a valid Dropbox app key and secret.",
    "Error configuring Google Drive storage" => "Error configuring Google Drive storage",
    "<b>Warning:</b> \"smbclient\" is not installed. Mounting of CIFS/SMB shares is not possible. Please ask your system administrator to install it." => "<b>Warning:</b> \"smbclient\" is not installed. Mounting of CIFS/SMB shares is not possible. Please ask your system administrator to install it.",
    "<b>Warning:</b> The FTP support in PHP is not enabled or installed. Mounting of FTP shares is not possible. Please ask your system administrator to install it." => "<b>Warning:</b> The FTP support in PHP is not enabled or installed. Mounting of FTP shares is not possible. Please ask your system administrator to install it.",
    "<b>Warning:</b> The Curl support in PHP is not enabled or installed. Mounting of ownCloud / WebDAV or GoogleDrive is not possible. Please ask your system administrator to install it." => "<b>Warning:</b> The Curl support in PHP is not enabled or installed. Mounting of ownCloud / WebDAV or GoogleDrive is not possible. Please ask your system administrator to install it.",
    "External Storage" => "External Storage",
    "Folder name" => "Folder name",
    "External storage" => "External storage",
    "Configuration" => "Configuration",
    "Options" => "Options",
    "Applicable" => "Applicable",
    "Add storage" => "Add storage",
    "None set" => "None set",
    "All Users" => "All Users",
    "Groups" => "Groups",
    "Users" => "Users",
    "Delete" => "Delete",
    "Enable User External Storage" => "Enable User External Storage",
    "Allow users to mount their own external storage" => "Allow users to mount their own external storage",
    "SSL root certificates" => "SSL root certificates",
    "Import Root Certificate" => "Import Root Certificate",
};

pub fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}

pub fn get_translations() -> &'static phf::Map<&'static str, &'static str> {
    &EN_GB_TRANSLATIONS
}

// Alternative implementation using a function to create the HashMap at runtime
pub fn create_translations() -> HashMap<String, String> {
    let mut translations = HashMap::new();
    
    for (key, value) in &EN_GB_TRANSLATIONS {
        translations.insert(key.to_string(), value.to_string());
    }
    
    translations
}