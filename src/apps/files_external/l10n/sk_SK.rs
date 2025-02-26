use std::collections::HashMap;
use rust_gettext::Catalog;

pub fn get_sk_sk_translation() -> (HashMap<&'static str, &'static str>, &'static str) {
    let translations: HashMap<&'static str, &'static str> = [
        ("Access granted", "Prístup povolený"),
        ("Error configuring Dropbox storage", "Chyba pri konfigurácii úložiska Dropbox"),
        ("Grant access", "Povoliť prístup"),
        ("Please provide a valid Dropbox app key and secret.", "Zadajte platný kľúč aplikácie a heslo Dropbox"),
        ("Error configuring Google Drive storage", "Chyba pri konfigurácii úložiska Google drive"),
        ("<b>Warning:</b> \"smbclient\" is not installed. Mounting of CIFS/SMB shares is not possible. Please ask your system administrator to install it.", "<b>Upozornenie:</b> \"smbclient\" nie je nainštalovaný. Nie je možné pripojenie oddielov CIFS/SMB. Požiadajte administrátora systému, nech ho nainštaluje."),
        ("<b>Warning:</b> The FTP support in PHP is not enabled or installed. Mounting of FTP shares is not possible. Please ask your system administrator to install it.", "<b>Upozornenie:</b> Podpora FTP v PHP nie je povolená alebo nainštalovaná. Nie je možné pripojenie oddielov FTP. Požiadajte administrátora systému, nech ho nainštaluje."),
        ("<b>Warning:</b> The Curl support in PHP is not enabled or installed. Mounting of ownCloud / WebDAV or GoogleDrive is not possible. Please ask your system administrator to install it.", "<b>Varovanie:</b> nie je nainštalovaná, alebo povolená, podpora Curl v PHP. Nie je možné pripojenie oddielov ownCloud, WebDAV, či GoogleDrive. Prosím požiadajte svojho administrátora systému, nech ju nainštaluje."),
        ("External Storage", "Externé úložisko"),
        ("Folder name", "Meno priečinka"),
        ("External storage", "Externé úložisko"),
        ("Configuration", "Nastavenia"),
        ("Options", "Možnosti"),
        ("Applicable", "Aplikovateľné"),
        ("Add storage", "Pridať úložisko"),
        ("None set", "Žiadne nastavené"),
        ("All Users", "Všetci používatelia"),
        ("Groups", "Skupiny"),
        ("Users", "Používatelia"),
        ("Delete", "Zmazať"),
        ("Enable User External Storage", "Povoliť externé úložisko"),
        ("Allow users to mount their own external storage", "Povoliť používateľom pripojiť ich vlastné externé úložisko"),
        ("SSL root certificates", "Koreňové SSL certifikáty"),
        ("Import Root Certificate", "Importovať koreňový certifikát"),
    ].iter().cloned().collect();
    
    let plural_forms = "nplurals=3; plural=(n==1) ? 0 : (n>=2 && n<=4) ? 1 : 2;";
    
    (translations, plural_forms)
}

pub fn get_sk_sk_catalog() -> Catalog {
    let (translations, plural_forms) = get_sk_sk_translation();
    let mut catalog = Catalog::new();
    
    for (msgid, msgstr) in translations {
        catalog.add_simple_translation(msgid, msgstr);
    }
    
    catalog.set_plural_forms_formula(plural_forms);
    catalog
}