use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Access granted", "Přístup povolen");
        m.insert("Error configuring Dropbox storage", "Chyba při nastavení úložiště Dropbox");
        m.insert("Grant access", "Povolit přístup");
        m.insert("Please provide a valid Dropbox app key and secret.", "Zadejte, prosím, platný klíč a bezpečnostní frázi aplikace Dropbox.");
        m.insert("Error configuring Google Drive storage", "Chyba při nastavení úložiště Google Drive");
        m.insert("<b>Warning:</b> \"smbclient\" is not installed. Mounting of CIFS/SMB shares is not possible. Please ask your system administrator to install it.", "<b>Varování:</b> není nainstalován program \"smbclient\". Není možné připojení oddílů CIFS/SMB. Prosím požádejte svého správce systému ať jej nainstaluje.");
        m.insert("<b>Warning:</b> The FTP support in PHP is not enabled or installed. Mounting of FTP shares is not possible. Please ask your system administrator to install it.", "<b>Varování:</b> podpora FTP v PHP není povolena nebo není nainstalována. Není možné připojení oddílů FTP. Prosím požádejte svého správce systému ať ji nainstaluje.");
        m.insert("<b>Warning:</b> The Curl support in PHP is not enabled or installed. Mounting of ownCloud / WebDAV or GoogleDrive is not possible. Please ask your system administrator to install it.", "<b>Varování:</b> podpora CURL v PHP není povolena nebo není nainstalována. Není možné připojení oddílů ownCloud, WebDAV, či GoogleDrive. Prosím požádejte svého správce systému ať ji nainstaluje.");
        m.insert("External Storage", "Externí úložiště");
        m.insert("Folder name", "Název složky");
        m.insert("External storage", "Externí úložiště");
        m.insert("Configuration", "Nastavení");
        m.insert("Options", "Možnosti");
        m.insert("Applicable", "Přístupný pro");
        m.insert("Add storage", "Přidat úložiště");
        m.insert("None set", "Nenastaveno");
        m.insert("All Users", "Všichni uživatelé");
        m.insert("Groups", "Skupiny");
        m.insert("Users", "Uživatelé");
        m.insert("Delete", "Smazat");
        m.insert("Enable User External Storage", "Zapnout externí uživatelské úložiště");
        m.insert("Allow users to mount their own external storage", "Povolit uživatelům připojení jejich vlastních externích úložišť");
        m.insert("SSL root certificates", "Kořenové certifikáty SSL");
        m.insert("Import Root Certificate", "Importovat kořenového certifikátu");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=3; plural=(n==1) ? 0 : (n>=2 && n<=4) ? 1 : 2;";
}