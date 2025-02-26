use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Norwegian Bokmål (Norway) translations
pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("Access granted", "Tilgang innvilget");
    m.insert("Error configuring Dropbox storage", "Feil ved konfigurering av Dropbox-lagring");
    m.insert("Grant access", "Gi tilgang");
    m.insert("Please provide a valid Dropbox app key and secret.", "Vær vennlig å oppgi gyldig Dropbox appnøkkel og hemmelighet.");
    m.insert("Error configuring Google Drive storage", "Feil med konfigurering av Google Drive");
    m.insert("<b>Warning:</b> \"smbclient\" is not installed. Mounting of CIFS/SMB shares is not possible. Please ask your system administrator to install it.", "<b>Advarsel:</b> \"smbclient\" er ikke installert. Kan ikke montere CIFS/SMB mapper. Ta kontakt med din systemadministrator for å installere det.");
    m.insert("<b>Warning:</b> The FTP support in PHP is not enabled or installed. Mounting of FTP shares is not possible. Please ask your system administrator to install it.", "<b>Advarsel:</b> FTP støtte i PHP er ikke slått på eller innstallert. Kan ikke montere FTP mapper. Ta kontakt med din systemadministrator for å innstallere det.");
    m.insert("<b>Warning:</b> The Curl support in PHP is not enabled or installed. Mounting of ownCloud / WebDAV or GoogleDrive is not possible. Please ask your system administrator to install it.", "<b>Advarsel:</b> Curl støtte i PHP er ikke aktivert eller innstallert. Kan ikke montere owncloud/WebDAV eller Googledrive. Ta kontakt med din systemadministrator for å innstallerer det.");
    m.insert("External Storage", "Ekstern lagring");
    m.insert("Folder name", "Mappenavn");
    m.insert("External storage", "Ekstern lagringsplass");
    m.insert("Configuration", "Konfigurasjon");
    m.insert("Options", "Innstillinger");
    m.insert("Applicable", "Anvendelig");
    m.insert("Add storage", "Legg til lagringsplass");
    m.insert("None set", "Ingen valgt");
    m.insert("All Users", "Alle brukere");
    m.insert("Groups", "Grupper");
    m.insert("Users", "Brukere");
    m.insert("Delete", "Slett");
    m.insert("Enable User External Storage", "Aktiver ekstern lagring for bruker");
    m.insert("Allow users to mount their own external storage", "Tillat brukere å koble til egne eksterne lagringsmedium");
    m.insert("SSL root certificates", "SSL root-sertifikater");
    m.insert("Import Root Certificate", "Importer root-sertifikat");
    m
});

/// Returns the plural form ID to use for the given count
pub fn get_plural_form(n: i64) -> usize {
    if n != 1 {
        1
    } else {
        0
    }
}

/// Number of plural forms
pub const PLURAL_FORMS_COUNT: usize = 2;