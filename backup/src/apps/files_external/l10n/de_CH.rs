use std::collections::HashMap;
use once_cell::sync::Lazy;

// Define translations for de_CH (Swiss German)
pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("Access granted", "Zugriff gestattet");
    m.insert("Error configuring Dropbox storage", "Fehler beim Einrichten von Dropbox");
    m.insert("Grant access", "Zugriff gestatten");
    m.insert("Please provide a valid Dropbox app key and secret.", "Bitte tragen Sie einen gültigen Dropbox-App-Key mit Secret ein.");
    m.insert("Error configuring Google Drive storage", "Fehler beim Einrichten von Google Drive");
    m.insert("<b>Warning:</b> \"smbclient\" is not installed. Mounting of CIFS/SMB shares is not possible. Please ask your system administrator to install it.", "<b>Warnung:</b> «smbclient» ist nicht installiert. Das Einhängen von CIFS/SMB-Freigaben ist nicht möglich. Bitten Sie Ihren Systemadministrator, dies zu installieren.");
    m.insert("<b>Warning:</b> The FTP support in PHP is not enabled or installed. Mounting of FTP shares is not possible. Please ask your system administrator to install it.", "<b>Warnung::</b> Die FTP Unterstützung  von PHP ist nicht aktiviert oder installiert. Bitte wenden Sie sich an Ihren Systemadministrator.");
    m.insert("<b>Warning:</b> The Curl support in PHP is not enabled or installed. Mounting of ownCloud / WebDAV or GoogleDrive is not possible. Please ask your system administrator to install it.", "<b>Achtung:</b> Die Curl-Unterstützung  von PHP ist nicht aktiviert oder installiert. Das Laden von ownCloud / WebDAV oder GoogleDrive Freigaben ist nicht möglich. Bitte Sie Ihren Systemadministrator, das Modul zu installieren.");
    m.insert("External Storage", "Externer Speicher");
    m.insert("Folder name", "Ordnername");
    m.insert("External storage", "Externer Speicher");
    m.insert("Configuration", "Konfiguration");
    m.insert("Options", "Optionen");
    m.insert("Applicable", "Zutreffend");
    m.insert("Add storage", "Speicher hinzufügen");
    m.insert("None set", "Nicht definiert");
    m.insert("All Users", "Alle Benutzer");
    m.insert("Groups", "Gruppen");
    m.insert("Users", "Benutzer");
    m.insert("Delete", "Löschen");
    m.insert("Enable User External Storage", "Externen Speicher für Benutzer aktivieren");
    m.insert("Allow users to mount their own external storage", "Erlaubt Benutzern, ihre eigenen externen Speicher einzubinden");
    m.insert("SSL root certificates", "SSL-Root-Zertifikate");
    m.insert("Import Root Certificate", "Root-Zertifikate importieren");
    m
});

pub static PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";

pub fn get_translation(key: &str) -> &'static str {
    TRANSLATIONS.get(key).copied().unwrap_or(key)
}