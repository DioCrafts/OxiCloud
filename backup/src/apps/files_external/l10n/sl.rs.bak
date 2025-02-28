use phf::phf_map;
use rust_i18n::t;

// Static translations map using PHF for efficient compile-time hashing
pub static SL_TRANSLATIONS: phf::Map<&'static str, &'static str> = phf_map! {
    "Access granted" => "Dostop je odobren",
    "Error configuring Dropbox storage" => "Napaka nastavljanja shrambe Dropbox",
    "Grant access" => "Odobri dostop",
    "Please provide a valid Dropbox app key and secret." => "Vpisati je treba veljaven ključ programa in kodo za Dropbox",
    "Error configuring Google Drive storage" => "Napaka nastavljanja shrambe Google Drive",
    "<b>Warning:</b> \"smbclient\" is not installed. Mounting of CIFS/SMB shares is not possible. Please ask your system administrator to install it." => "<b>Opozorilo:</b> paket \"smbclient\" ni nameščen. Priklapljanje pogonov CIFS/SMB ne bo mogoče.",
    "<b>Warning:</b> The FTP support in PHP is not enabled or installed. Mounting of FTP shares is not possible. Please ask your system administrator to install it." => "<b>Opozorilo:</b> podpora FTP v PHP ni omogočena ali pa ni nameščena. Priklapljanje pogonov FTP zato ne bo mogoče.",
    "<b>Warning:</b> The Curl support in PHP is not enabled or installed. Mounting of ownCloud / WebDAV or GoogleDrive is not possible. Please ask your system administrator to install it." => "<b>Opozorilo:</b> podpora za Curl v PHP ni omogočena ali pa ni nameščena. Priklapljanje točke ownCloud / WebDAV ali GoogleDrive zato ne bo mogoče. Zahtevane pakete je treba pred uporabo namestiti.",
    "External Storage" => "Zunanja podatkovna shramba",
    "Folder name" => "Ime mape",
    "External storage" => "Zunanja shramba",
    "Configuration" => "Nastavitve",
    "Options" => "Možnosti",
    "Applicable" => "Se uporablja",
    "Add storage" => "Dodaj shrambo",
    "None set" => "Ni nastavljeno",
    "All Users" => "Vsi uporabniki",
    "Groups" => "Skupine",
    "Users" => "Uporabniki",
    "Delete" => "Izbriši",
    "Enable User External Storage" => "Omogoči zunanjo uporabniško podatkovno shrambo",
    "Allow users to mount their own external storage" => "Dovoli uporabnikom priklop lastne zunanje podatkovne shrambe",
    "SSL root certificates" => "Korenska potrdila SSL",
    "Import Root Certificate" => "Uvozi korensko potrdilo",
};

// Plural forms definition for Slovenian language
pub const PLURAL_FORMS: &str = "nplurals=4; plural=(n%100==1 ? 0 : n%100==2 ? 1 : n%100==3 || n%100==4 ? 2 : 3);";

// Register translations with the i18n system
pub fn register_translations() {
    rust_i18n::set_locale("sl");
    
    for (key, value) in &SL_TRANSLATIONS {
        rust_i18n::add_translation("sl", key, value);
    }
    
    rust_i18n::set_plural_rule("sl", |n| {
        if n % 100 == 1 {
            0
        } else if n % 100 == 2 {
            1
        } else if n % 100 == 3 || n % 100 == 4 {
            2
        } else {
            3
        }
    });
}