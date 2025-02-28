use std::collections::HashMap;
use rust_i18n::t;

// Locale: Slovak (Slovakia)
pub fn register_sk_sk_translation() -> HashMap<String, String> {
    let mut translations = HashMap::new();
    
    translations.insert(
        "App \"%s\" can't be installed because it is not compatible with this version of ownCloud.".to_string(),
        "Aplikácia \"%s\" nemôže byť nainštalovaná kvôli nekompatibilite z danou verziou ownCloudu.".to_string(),
    );
    translations.insert(
        "No app name specified".to_string(),
        "Nešpecifikované meno aplikácie".to_string(),
    );
    translations.insert(
        "Help".to_string(),
        "Pomoc".to_string(),
    );
    translations.insert(
        "Personal".to_string(),
        "Osobné".to_string(),
    );
    translations.insert(
        "Settings".to_string(),
        "Nastavenia".to_string(),
    );
    translations.insert(
        "Users".to_string(),
        "Používatelia".to_string(),
    );
    translations.insert(
        "Admin".to_string(),
        "Administrátor".to_string(),
    );
    translations.insert(
        "Failed to upgrade \"%s\".".to_string(),
        "Zlyhala aktualizácia \"%s\".".to_string(),
    );
    translations.insert(
        "Unknown filetype".to_string(),
        "Neznámy typ súboru".to_string(),
    );
    translations.insert(
        "Invalid image".to_string(),
        "Chybný obrázok".to_string(),
    );
    translations.insert(
        "web services under your control".to_string(),
        "webové služby pod Vašou kontrolou".to_string(),
    );
    translations.insert(
        "cannot open \"%s\"".to_string(),
        "nemožno otvoriť \"%s\"".to_string(),
    );
    translations.insert(
        "ZIP download is turned off.".to_string(),
        "Sťahovanie súborov ZIP je vypnuté.".to_string(),
    );
    translations.insert(
        "Files need to be downloaded one by one.".to_string(),
        "Súbory musia byť nahrávané jeden za druhým.".to_string(),
    );
    translations.insert(
        "Back to Files".to_string(),
        "Späť na súbory".to_string(),
    );
    translations.insert(
        "Selected files too large to generate zip file.".to_string(),
        "Zvolené súbory sú príliš veľké na vygenerovanie zip súboru.".to_string(),
    );
    translations.insert(
        "Download the files in smaller chunks, seperately or kindly ask your administrator.".to_string(),
        "Stiahnite súbory po menších častiach, samostatne, alebo sa obráťte na správcu.".to_string(),
    );
    translations.insert(
        "No source specified when installing app".to_string(),
        "Nešpecifikovaný zdroj pri inštalácii aplikácie".to_string(),
    );
    translations.insert(
        "No href specified when installing app from http".to_string(),
        "Nešpecifikovaný atribút \"href\" pri inštalácii aplikácie pomocou protokolu \"http\"".to_string(),
    );
    translations.insert(
        "No path specified when installing app from local file".to_string(),
        "Nešpecifikovaná cesta pri inštalácii aplikácie z lokálneho súboru".to_string(),
    );
    translations.insert(
        "Archives of type %s are not supported".to_string(),
        "Typ archívu %s nie je podporovaný".to_string(),
    );
    translations.insert(
        "Failed to open archive when installing app".to_string(),
        "Zlyhanie pri otváraní archívu počas inštalácie aplikácie".to_string(),
    );
    translations.insert(
        "App does not provide an info.xml file".to_string(),
        "Aplikácia neposkytuje súbor info.xml".to_string(),
    );
    translations.insert(
        "App can't be installed because of not allowed code in the App".to_string(),
        "Aplikácia nemôže byť inštalovaná pre nepovolený kód v aplikácii".to_string(),
    );
    translations.insert(
        "App can't be installed because it is not compatible with this version of ownCloud".to_string(),
        "Aplikácia nemôže byť inštalovaná pre nekompatibilitu z danou verziou ownCloudu".to_string(),
    );
    translations.insert(
        "App can't be installed because it contains the <shipped>true</shipped> tag which is not allowed for non shipped apps".to_string(),
        "Aplikácia nemôže byť inštalovaná pretože obsahuje <shipped>pravý</shipped> štítok, ktorý nie je povolený pre zaslané \"shipped\" aplikácie".to_string(),
    );
    translations.insert(
        "App can't be installed because the version in info.xml/version is not the same as the version reported from the app store".to_string(),
        "Aplikácia nemôže byť inštalovaná pretože verzia v info.xml/version nezodpovedá verzii špecifikovanej v aplikačnom obchode".to_string(),
    );
    translations.insert(
        "App directory already exists".to_string(),
        "Aplikačný adresár už existuje".to_string(),
    );
    translations.insert(
        "Can't create app folder. Please fix permissions. %s".to_string(),
        "Nemožno vytvoriť aplikačný priečinok. Prosím upravte povolenia. %s".to_string(),
    );
    translations.insert(
        "Application is not enabled".to_string(),
        "Aplikácia nie je zapnutá".to_string(),
    );
    translations.insert(
        "Authentication error".to_string(),
        "Chyba autentifikácie".to_string(),
    );
    translations.insert(
        "Token expired. Please reload page.".to_string(),
        "Token vypršal. Obnovte, prosím, stránku.".to_string(),
    );
    translations.insert(
        "Files".to_string(),
        "Súbory".to_string(),
    );
    translations.insert(
        "Text".to_string(),
        "Text".to_string(),
    );
    translations.insert(
        "Images".to_string(),
        "Obrázky".to_string(),
    );
    translations.insert(
        "%s enter the database username.".to_string(),
        "Zadajte používateľské meno %s databázy..".to_string(),
    );
    translations.insert(
        "%s enter the database name.".to_string(),
        "Zadajte názov databázy pre %s databázy.".to_string(),
    );
    translations.insert(
        "%s you may not use dots in the database name".to_string(),
        "V názve databázy %s nemôžete používať bodky".to_string(),
    );
    translations.insert(
        "MS SQL username and/or password not valid: %s".to_string(),
        "Používateľské meno, alebo heslo MS SQL nie je platné: %s".to_string(),
    );
    translations.insert(
        "You need to enter either an existing account or the administrator.".to_string(),
        "Musíte zadať jestvujúci účet alebo administrátora.".to_string(),
    );
    translations.insert(
        "MySQL username and/or password not valid".to_string(),
        "Používateľské meno a/alebo heslo pre MySQL databázu je neplatné".to_string(),
    );
    translations.insert(
        "DB Error: \"%s\"".to_string(),
        "Chyba DB: \"%s\"".to_string(),
    );
    translations.insert(
        "Offending command was: \"%s\"".to_string(),
        "Podozrivý príkaz bol: \"%s\"".to_string(),
    );
    translations.insert(
        "MySQL user '%s'@'localhost' exists already.".to_string(),
        "Používateľ '%s'@'localhost' už v MySQL existuje.".to_string(),
    );
    translations.insert(
        "Drop this user from MySQL".to_string(),
        "Zahodiť používateľa z MySQL.".to_string(),
    );
    translations.insert(
        "MySQL user '%s'@'%%' already exists".to_string(),
        "Používateľ '%s'@'%%' už v MySQL existuje".to_string(),
    );
    translations.insert(
        "Drop this user from MySQL.".to_string(),
        "Zahodiť používateľa z MySQL.".to_string(),
    );
    translations.insert(
        "Oracle connection could not be established".to_string(),
        "Nie je možné pripojiť sa k Oracle".to_string(),
    );
    translations.insert(
        "Oracle username and/or password not valid".to_string(),
        "Používateľské meno a/alebo heslo pre Oracle databázu je neplatné".to_string(),
    );
    translations.insert(
        "Offending command was: \"%s\", name: %s, password: %s".to_string(),
        "Podozrivý príkaz bol: \"%s\", meno: %s, heslo: %s".to_string(),
    );
    translations.insert(
        "PostgreSQL username and/or password not valid".to_string(),
        "Používateľské meno a/alebo heslo pre PostgreSQL databázu je neplatné".to_string(),
    );
    translations.insert(
        "Set an admin username.".to_string(),
        "Zadajte používateľské meno administrátora.".to_string(),
    );
    translations.insert(
        "Set an admin password.".to_string(),
        "Zadajte heslo administrátora.".to_string(),
    );
    translations.insert(
        "Your web server is not yet properly setup to allow files synchronization because the WebDAV interface seems to be broken.".to_string(),
        "Váš webový server nie je správne nastavený na synchronizáciu, pretože rozhranie WebDAV je poškodené.".to_string(),
    );
    translations.insert(
        "Please double check the <a href='%s'>installation guides</a>.".to_string(),
        "Prosím skontrolujte <a href='%s'>inštalačnú príručku</a>.".to_string(),
    );
    translations.insert(
        "Could not find category \"%s\"".to_string(),
        "Nemožno nájsť danú kategóriu \"%s\"".to_string(),
    );
    translations.insert(
        "seconds ago".to_string(),
        "pred sekundami".to_string(),
    );
    translations.insert(
        "_%n minute ago_::_%n minutes ago_".to_string(),
        "pred %n minútami".to_string(),
    );
    translations.insert(
        "_%n hour ago_::_%n hours ago_".to_string(),
        "pred %n hodinami".to_string(),
    );
    translations.insert(
        "today".to_string(),
        "dnes".to_string(),
    );
    translations.insert(
        "yesterday".to_string(),
        "včera".to_string(),
    );
    translations.insert(
        "_%n day go_::_%n days ago_".to_string(),
        "pred %n dňami".to_string(),
    );
    translations.insert(
        "last month".to_string(),
        "minulý mesiac".to_string(),
    );
    translations.insert(
        "_%n month ago_::_%n months ago_".to_string(),
        "pred %n mesiacmi".to_string(),
    );
    translations.insert(
        "last year".to_string(),
        "minulý rok".to_string(),
    );
    translations.insert(
        "years ago".to_string(),
        "pred rokmi".to_string(),
    );
    translations.insert(
        "Caused by:".to_string(),
        "Príčina:".to_string(),
    );
    
    translations
}

pub fn get_plural_forms() -> &'static str {
    "nplurals=3; plural=(n==1) ? 0 : (n>=2 && n<=4) ? 1 : 2;"
}