use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("Access granted", "Priėjimas suteiktas");
    m.insert("Error configuring Dropbox storage", "Klaida nustatinėjant Dropbox talpyklą");
    m.insert("Grant access", "Suteikti priėjimą");
    m.insert("Please provide a valid Dropbox app key and secret.", "Prašome įvesti teisingus Dropbox \"app key\" ir \"secret\".");
    m.insert("Error configuring Google Drive storage", "Klaida nustatinėjant Google Drive talpyklą");
    m.insert("<b>Warning:</b> \"smbclient\" is not installed. Mounting of CIFS/SMB shares is not possible. Please ask your system administrator to install it.", "<b>Įspėjimas:</b> \"smbclient\" nėra įdiegtas. CIFS/SMB dalinimasis nėra galimas. Prašome susisiekti su sistemos administratoriumi kad būtų įdiegtas \"smbclient\"");
    m.insert("<b>Warning:</b> The FTP support in PHP is not enabled or installed. Mounting of FTP shares is not possible. Please ask your system administrator to install it.", "<b>Įspėjimas:</b> FTP palaikymas PHP sistemoje nėra įjungtas arba nėra įdiegtas.  FTP dalinimosi įjungimas nėra galimas. Prašome susisiekti su sistemos administratoriumi kad būtų įdiegtas FTP palaikymas. ");
    m.insert("<b>Warning:</b> The Curl support in PHP is not enabled or installed. Mounting of ownCloud / WebDAV or GoogleDrive is not possible. Please ask your system administrator to install it.", "<b>Įspėjimas:</b> \"Curl\" palaikymas PHP terpėje nėra įjungtas arba įdiegtas. ownCloud/WebDAV ar GoogleDrive įjungimas nebus įmanomas. Prašome susisiekti su sistemos administratoriumi kad būtų įdiegtas arba įjungtas \"Curl\" palaikymas.");
    m.insert("External Storage", "Išorinės saugyklos");
    m.insert("Folder name", "Katalogo pavadinimas");
    m.insert("External storage", "Išorinė saugykla");
    m.insert("Configuration", "Konfigūracija");
    m.insert("Options", "Nustatymai");
    m.insert("Applicable", "Pritaikyti");
    m.insert("Add storage", "Pridėti saugyklą");
    m.insert("None set", "Nieko nepasirinkta");
    m.insert("All Users", "Visi vartotojai");
    m.insert("Groups", "Grupės");
    m.insert("Users", "Vartotojai");
    m.insert("Delete", "Ištrinti");
    m.insert("Enable User External Storage", "Įjungti vartotojų išorines saugyklas");
    m.insert("Allow users to mount their own external storage", "Leisti vartotojams pridėti savo išorines saugyklas");
    m.insert("SSL root certificates", "SSL sertifikatas");
    m.insert("Import Root Certificate", "Įkelti pagrindinį sertifikatą");
    m
});

pub const PLURAL_FORMS: &str = "nplurals=3; plural=(n%10==1 && n%100!=11 ? 0 : n%10>=2 && (n%100<10 || n%100>=20) ? 1 : 2);";