use std::collections::HashMap;
use rust_i18n::t;

pub fn initialize_translations() -> HashMap<&'static str, &'static str> {
    let mut translations = HashMap::new();
    
    translations.insert("Access granted", "Zugriff gestattet");
    translations.insert("Error configuring Dropbox storage", "Fehler beim Einrichten von Dropbox");
    translations.insert("Grant access", "Zugriff gestatten");
    translations.insert("Please provide a valid Dropbox app key and secret.", "Bitte tragen Sie einen gültigen Dropbox-App-Key mit Secret ein.");
    translations.insert("Error configuring Google Drive storage", "Fehler beim Einrichten von Google Drive");
    translations.insert("<b>Warning:</b> \"smbclient\" is not installed. Mounting of CIFS/SMB shares is not possible. Please ask your system administrator to install it.", "<b>Warnung:</b> \"smbclient\" ist nicht installiert. Das Einhängen von CIFS/SMB-Freigaben ist nicht möglich. Bitten Sie Ihren Systemadministrator, dies zu installieren.");
    translations.insert("<b>Warning:</b> The FTP support in PHP is not enabled or installed. Mounting of FTP shares is not possible. Please ask your system administrator to install it.", "<b>Warnung::</b> Die FTP Unterstützung  von PHP ist nicht aktiviert oder installiert. Bitte wenden Sie sich an Ihren Systemadministrator.");
    translations.insert("<b>Warning:</b> The Curl support in PHP is not enabled or installed. Mounting of ownCloud / WebDAV or GoogleDrive is not possible. Please ask your system administrator to install it.", "<b>Achtung:</b> Die Curl-Unterstützung  von PHP ist nicht aktiviert oder installiert. Das Laden von ownCloud / WebDAV oder GoogleDrive Freigaben ist nicht möglich. Bitte Sie Ihren Systemadministrator, das Modul zu installieren.");
    translations.insert("External Storage", "Externer Speicher");
    translations.insert("Folder name", "Ordnername");
    translations.insert("External storage", "Externer Speicher");
    translations.insert("Configuration", "Konfiguration");
    translations.insert("Options", "Optionen");
    translations.insert("Applicable", "Zutreffend");
    translations.insert("Add storage", "Speicher hinzufügen");
    translations.insert("None set", "Nicht definiert");
    translations.insert("All Users", "Alle Benutzer");
    translations.insert("Groups", "Gruppen");
    translations.insert("Users", "Benutzer");
    translations.insert("Delete", "Löschen");
    translations.insert("Enable User External Storage", "Externen Speicher für Benutzer aktivieren");
    translations.insert("Allow users to mount their own external storage", "Erlaubt Benutzern, ihre eigenen externen Speicher einzubinden");
    translations.insert("SSL root certificates", "SSL-Root-Zertifikate");
    translations.insert("Import Root Certificate", "Root-Zertifikate importieren");
    
    translations
}

pub fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}

// Register this module with the i18n system
#[cfg(feature = "i18n")]
pub fn register_i18n() {
    rust_i18n::set_locale("de_DE");
    
    let translations = initialize_translations();
    for (key, value) in translations {
        rust_i18n::add_translation("de_DE", key, value);
    }
    
    rust_i18n::set_plural_rule("de_DE", get_plural_forms());
}