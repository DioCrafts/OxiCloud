use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("Access granted", "Zugriff gestattet");
    map.insert("Error configuring Dropbox storage", "Fehler beim Einrichten von Dropbox");
    map.insert("Grant access", "Zugriff gestatten");
    map.insert("Please provide a valid Dropbox app key and secret.", "Bitte trage einen gültigen Dropbox-App-Key mit Secret ein.");
    map.insert("Error configuring Google Drive storage", "Fehler beim Einrichten von Google Drive");
    map.insert("<b>Warning:</b> \"smbclient\" is not installed. Mounting of CIFS/SMB shares is not possible. Please ask your system administrator to install it.", "<b>Warnung:</b> \"smbclient\" ist nicht installiert. Das Einhängen von CIFS/SMB-Freigaben ist nicht möglich. Bitte Deinen System-Administrator, dies zu installieren.");
    map.insert("<b>Warning:</b> The FTP support in PHP is not enabled or installed. Mounting of FTP shares is not possible. Please ask your system administrator to install it.", "<b>Warnung::</b> Die FTP Unterstützung  von PHP ist nicht aktiviert oder installiert. Bitte wende Dich an Deinen Systemadministrator.");
    map.insert("<b>Warning:</b> The Curl support in PHP is not enabled or installed. Mounting of ownCloud / WebDAV or GoogleDrive is not possible. Please ask your system administrator to install it.", "<b>Warnung:</b> Die Curl-Unterstützung in PHP ist nicht aktiviert oder installiert. Das Einbinden von ownCloud / WebDav der GoogleDrive-Freigaben ist nicht möglich. Bitte Deinen Systemadminstrator um die Installation. ");
    map.insert("External Storage", "Externer Speicher");
    map.insert("Folder name", "Ordnername");
    map.insert("External storage", "Externer Speicher");
    map.insert("Configuration", "Konfiguration");
    map.insert("Options", "Optionen");
    map.insert("Applicable", "Zutreffend");
    map.insert("Add storage", "Speicher hinzufügen");
    map.insert("None set", "Nicht definiert");
    map.insert("All Users", "Alle Benutzer");
    map.insert("Groups", "Gruppen");
    map.insert("Users", "Benutzer");
    map.insert("Delete", "Löschen");
    map.insert("Enable User External Storage", "Externen Speicher für Benutzer aktivieren");
    map.insert("Allow users to mount their own external storage", "Erlaubt Benutzern ihre eigenen externen Speicher einzubinden");
    map.insert("SSL root certificates", "SSL-Root-Zertifikate");
    map.insert("Import Root Certificate", "Root-Zertifikate importieren");
    map
});

pub const PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}