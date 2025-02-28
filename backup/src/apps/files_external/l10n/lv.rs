use once_cell::sync::Lazy;
use std::collections::HashMap;
use rust_i18n::t;

// Define translations for Latvian
pub static LV_TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut translations = HashMap::new();
    translations.insert("Access granted", "Piešķirta pieeja");
    translations.insert("Error configuring Dropbox storage", "Kļūda, konfigurējot Dropbox krātuvi");
    translations.insert("Grant access", "Piešķirt pieeju");
    translations.insert("Please provide a valid Dropbox app key and secret.", "Lūdzu, norādiet derīgu Dropbox lietotnes atslēgu un noslēpumu.");
    translations.insert("Error configuring Google Drive storage", "Kļūda, konfigurējot Google Drive krātuvi");
    translations.insert("<b>Warning:</b> \"smbclient\" is not installed. Mounting of CIFS/SMB shares is not possible. Please ask your system administrator to install it.", "<b>Brīdinājums:</b> nav uzinstalēts "smbclient". Nevar montēt CIFS/SMB koplietojumus. Lūdzu, vaicājiet savam sistēmas administratoram, lai to uzinstalē.");
    translations.insert("<b>Warning:</b> The FTP support in PHP is not enabled or installed. Mounting of FTP shares is not possible. Please ask your system administrator to install it.", "<b>Brīdinājums: </b> uz PHP nav aktivēts vai instalēts FTP atbalsts. Nevar montēt FTP koplietojumus. Lūdzu, vaicājiet savam sistēmas administratoram, lai to uzinstalē.");
    translations.insert("<b>Warning:</b> The Curl support in PHP is not enabled or installed. Mounting of ownCloud / WebDAV or GoogleDrive is not possible. Please ask your system administrator to install it.", "<b>Brīdinājums:</b> PHP Curl atbalsts nav instalēts.  OwnCloud / WebDAV vai GoogleDrive montēšana nav iespējama. Lūdziet sistēmas administratoram lai tas tiek uzstādīts.");
    translations.insert("External Storage", "Ārējā krātuve");
    translations.insert("Folder name", "Mapes nosaukums");
    translations.insert("External storage", "Ārējā krātuve");
    translations.insert("Configuration", "Konfigurācija");
    translations.insert("Options", "Opcijas");
    translations.insert("Applicable", "Piemērojams");
    translations.insert("Add storage", "Pievienot krātuvi");
    translations.insert("None set", "Neviens nav iestatīts");
    translations.insert("All Users", "Visi lietotāji");
    translations.insert("Groups", "Grupas");
    translations.insert("Users", "Lietotāji");
    translations.insert("Delete", "Dzēst");
    translations.insert("Enable User External Storage", "Aktivēt lietotāja ārējo krātuvi");
    translations.insert("Allow users to mount their own external storage", "Ļaut lietotājiem montēt pašiem savu ārējo krātuvi");
    translations.insert("SSL root certificates", "SSL saknes sertifikāti");
    translations.insert("Import Root Certificate", "Importēt saknes sertifikātus");
    translations
});

// Define plural forms for Latvian
pub fn get_plural_form(n: i64) -> usize {
    if n % 10 == 1 && n % 100 != 11 {
        0
    } else if n != 0 {
        1
    } else {
        2
    }
}

// Implement translation function
pub fn translate(key: &str) -> String {
    match LV_TRANSLATIONS.get(key) {
        Some(&value) => value.to_string(),
        None => key.to_string(),
    }
}

// Implement plural translation function
pub fn translate_plural(singular: &str, plural: &str, n: i64) -> String {
    let form = get_plural_form(n);
    match form {
        0 => translate(singular),
        _ => translate(plural),
    }
}