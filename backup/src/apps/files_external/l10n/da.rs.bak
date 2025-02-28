use std::collections::HashMap;
use once_cell::sync::Lazy;

/// Danish translations for the files_external module
pub static TRANSLATIONS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    let mut translations = HashMap::new();
    translations.insert("Access granted", "Adgang godkendt");
    translations.insert("Error configuring Dropbox storage", "Fejl ved konfiguration af Dropbox plads");
    translations.insert("Grant access", "Godkend adgang");
    translations.insert("Please provide a valid Dropbox app key and secret.", "Angiv venligst en valid Dropbox app nøgle og hemmelighed");
    translations.insert("Error configuring Google Drive storage", "Fejl ved konfiguration af Google Drive plads");
    translations.insert("<b>Warning:</b> \"smbclient\" is not installed. Mounting of CIFS/SMB shares is not possible. Please ask your system administrator to install it.", "<b> Advarsel: </ b> \"smbclient\" ikke er installeret. Montering af CIFS / SMB delinger er ikke muligt. Spørg din systemadministrator om at installere det.");
    translations.insert("<b>Warning:</b> The FTP support in PHP is not enabled or installed. Mounting of FTP shares is not possible. Please ask your system administrator to install it.", "<b> Advarsel: </ b> FTP-understøttelse i PHP ikke er aktiveret eller installeret. Montering af FTP delinger er ikke muligt. Spørg din systemadministrator om at installere det.");
    translations.insert("<b>Warning:</b> The Curl support in PHP is not enabled or installed. Mounting of ownCloud / WebDAV or GoogleDrive is not possible. Please ask your system administrator to install it.", "<b>Advarsel:</b> Understøttelsen for Curl i PHP er enten ikke aktiveret eller ikke installeret. Det er ikke muligt, at montere ownCloud / WebDAV eller GoogleDrive. Spørg din system administrator om at installere det. ");
    translations.insert("External Storage", "Ekstern opbevaring");
    translations.insert("Folder name", "Mappenavn");
    translations.insert("External storage", "Eksternt lager");
    translations.insert("Configuration", "Opsætning");
    translations.insert("Options", "Valgmuligheder");
    translations.insert("Applicable", "Kan anvendes");
    translations.insert("Add storage", "Tilføj lager");
    translations.insert("None set", "Ingen sat");
    translations.insert("All Users", "Alle brugere");
    translations.insert("Groups", "Grupper");
    translations.insert("Users", "Brugere");
    translations.insert("Delete", "Slet");
    translations.insert("Enable User External Storage", "Aktiver ekstern opbevaring for brugere");
    translations.insert("Allow users to mount their own external storage", "Tillad brugere at montere deres egne eksterne opbevaring");
    translations.insert("SSL root certificates", "SSL-rodcertifikater");
    translations.insert("Import Root Certificate", "Importer rodcertifikat");
    translations
});

/// Plural forms for Danish language
pub static PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";

/// Get translation for a given key
pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}