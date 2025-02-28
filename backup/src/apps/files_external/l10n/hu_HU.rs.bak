use std::collections::HashMap;
use rust_gettext::Catalog;

pub fn get_translation_data() -> (HashMap<&'static str, &'static str>, &'static str) {
    let translations = HashMap::from([
        ("Access granted", "Érvényes hozzáférés"),
        ("Error configuring Dropbox storage", "A Dropbox tárolót nem sikerült beállítani"),
        ("Grant access", "Megadom a hozzáférést"),
        ("Please provide a valid Dropbox app key and secret.", "Adjon meg egy érvényes Dropbox app key-t és secretet!"),
        ("Error configuring Google Drive storage", "A Google Drive tárolót nem sikerült beállítani"),
        ("<b>Warning:</b> \"smbclient\" is not installed. Mounting of CIFS/SMB shares is not possible. Please ask your system administrator to install it.", "<b>Figyelem:</b> az \"smbclient\" nincs telepítve a kiszolgálón. Emiatt nem lehet CIFS/SMB megosztásokat fölcsatolni. Kérje meg a rendszergazdát, hogy telepítse a szükséges programot."),
        ("<b>Warning:</b> The FTP support in PHP is not enabled or installed. Mounting of FTP shares is not possible. Please ask your system administrator to install it.", "<b>Figyelem:</b> a PHP FTP támogatása vagy nincs telepítve, vagy nincs engedélyezve a kiszolgálón. Emiatt nem lehetséges FTP-tárolókat fölcsatolni. Kérje meg a rendszergazdát, hogy telepítse a szükséges programot."),
        ("<b>Warning:</b> The Curl support in PHP is not enabled or installed. Mounting of ownCloud / WebDAV or GoogleDrive is not possible. Please ask your system administrator to install it.", "<b>Figyelmeztetés:</b> A PHP-ben nincs telepítve vagy engedélyezve a Curl támogatás. Nem lehetséges ownCloud / WebDAV ill. GoogleDrive tárolók becsatolása. Kérje meg a rendszergazdát, hogy telepítse a szükséges programot!"),
        ("External Storage", "Külső tárolási szolgáltatások becsatolása"),
        ("Folder name", "Mappanév"),
        ("External storage", "Külső tárolók"),
        ("Configuration", "Beállítások"),
        ("Options", "Opciók"),
        ("Applicable", "Érvényességi kör"),
        ("Add storage", "Tároló becsatolása"),
        ("None set", "Nincs beállítva"),
        ("All Users", "Az összes felhasználó"),
        ("Groups", "Csoportok"),
        ("Users", "Felhasználók"),
        ("Delete", "Törlés"),
        ("Enable User External Storage", "Külső tárolók engedélyezése a felhasználók részére"),
        ("Allow users to mount their own external storage", "Lehetővé teszi, hogy a felhasználók külső tárolási szolgáltatásokat csatoljanak be a saját területükre"),
        ("SSL root certificates", "SSL tanúsítványok"),
        ("Import Root Certificate", "SSL tanúsítványok importálása"),
    ]);

    let plural_forms = "nplurals=2; plural=(n != 1);";
    
    (translations, plural_forms)
}

pub fn create_catalog() -> Catalog {
    let (translations, plural_forms) = get_translation_data();
    let mut catalog = Catalog::new("hu_HU", plural_forms);
    
    for (msgid, msgstr) in translations {
        catalog.add_simple_translation(msgid, msgstr);
    }
    
    catalog
}