// sv.rs

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Access granted", "Åtkomst beviljad");
        m.insert("Error configuring Dropbox storage", "Fel vid konfigurering av Dropbox");
        m.insert("Grant access", "Bevilja åtkomst");
        m.insert("Please provide a valid Dropbox app key and secret.", "Ange en giltig Dropbox nyckel och hemlighet.");
        m.insert("Error configuring Google Drive storage", "Fel vid konfigurering av Google Drive");
        m.insert("<b>Warning:</b> \"smbclient\" is not installed. Mounting of CIFS/SMB shares is not possible. Please ask your system administrator to install it.", "<b>Varning:</b> \"smb-klienten\" är inte installerad. Montering av CIFS/SMB delningar är inte möjligt. Kontakta din systemadministratör för att få den installerad.");
        m.insert("<b>Warning:</b> The FTP support in PHP is not enabled or installed. Mounting of FTP shares is not possible. Please ask your system administrator to install it.", "<b>Varning:</b> Stöd för FTP i PHP är inte aktiverat eller installerat. Montering av FTP-delningar är inte möjligt. Kontakta din systemadministratör för att få det installerat.");
        m.insert("<b>Warning:</b> The Curl support in PHP is not enabled or installed. Mounting of ownCloud / WebDAV or GoogleDrive is not possible. Please ask your system administrator to install it.", "<b>Varning:<b> Curl-stöd i PHP är inte aktiverat eller installerat. Montering av ownCloud / WebDAV eller GoogleDrive är inte möjligt. Vänligen be din administratör att installera det.");
        m.insert("External Storage", "Extern lagring");
        m.insert("Folder name", "Mappnamn");
        m.insert("External storage", "Extern lagring");
        m.insert("Configuration", "Konfiguration");
        m.insert("Options", "Alternativ");
        m.insert("Applicable", "Tillämplig");
        m.insert("Add storage", "Lägg till lagring");
        m.insert("None set", "Ingen angiven");
        m.insert("All Users", "Alla användare");
        m.insert("Groups", "Grupper");
        m.insert("Users", "Användare");
        m.insert("Delete", "Radera");
        m.insert("Enable User External Storage", "Aktivera extern lagring för användare");
        m.insert("Allow users to mount their own external storage", "Tillåt användare att montera egen extern lagring");
        m.insert("SSL root certificates", "SSL rotcertifikat");
        m.insert("Import Root Certificate", "Importera rotcertifikat");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

pub fn get_plural_form() -> &'static str {
    &PLURAL_FORMS
}