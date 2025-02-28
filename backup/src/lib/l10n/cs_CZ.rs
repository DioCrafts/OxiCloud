use std::collections::HashMap;
use rust_i18n::i18n;

#[allow(dead_code)]
pub fn get_translations() -> HashMap<&'static str, &'static str> {
    let mut translations = HashMap::new();
    
    translations.insert("App \"%s\" can't be installed because it is not compatible with this version of ownCloud.", "Aplikace \"%s\" nemůže být nainstalována, protože není kompatibilní s touto verzí ownCloud.");
    translations.insert("No app name specified", "Nebyl zadan název aplikace");
    translations.insert("Help", "Nápověda");
    translations.insert("Personal", "Osobní");
    translations.insert("Settings", "Nastavení");
    translations.insert("Users", "Uživatelé");
    translations.insert("Admin", "Administrace");
    translations.insert("Failed to upgrade \"%s\".", "Selhala aktualizace verze \"%s\".");
    translations.insert("Unknown filetype", "Neznámý typ souboru");
    translations.insert("Invalid image", "Chybný obrázek");
    translations.insert("web services under your control", "webové služby pod Vaší kontrolou");
    translations.insert("cannot open \"%s\"", "nelze otevřít \"%s\"");
    translations.insert("ZIP download is turned off.", "Stahování v ZIPu je vypnuto.");
    translations.insert("Files need to be downloaded one by one.", "Soubory musí být stahovány jednotlivě.");
    translations.insert("Back to Files", "Zpět k souborům");
    translations.insert("Selected files too large to generate zip file.", "Vybrané soubory jsou příliš velké pro vytvoření ZIP souboru.");
    translations.insert("Download the files in smaller chunks, seperately or kindly ask your administrator.", "Stáhněte soubory po menších částech, samostatně, nebo se obraťte na správce.");
    translations.insert("No source specified when installing app", "Nebyl zadán zdroj při instalaci aplikace");
    translations.insert("No href specified when installing app from http", "Nebyl zadán odkaz pro instalaci aplikace z HTTP");
    translations.insert("No path specified when installing app from local file", "Nebyla zadána cesta pro instalaci aplikace z místního souboru");
    translations.insert("Archives of type %s are not supported", "Archivy typu %s nejsou podporovány");
    translations.insert("Failed to open archive when installing app", "Chyba při otevírání archivu během instalace aplikace");
    translations.insert("App does not provide an info.xml file", "Aplikace neposkytuje soubor info.xml");
    translations.insert("App can't be installed because of not allowed code in the App", "Aplikace nemůže být nainstalována, protože obsahuje nepovolený kód");
    translations.insert("App can't be installed because it is not compatible with this version of ownCloud", "Aplikace nemůže být nainstalována, protože není kompatibilní s touto verzí ownCloud");
    translations.insert("App can't be installed because it contains the <shipped>true</shipped> tag which is not allowed for non shipped apps", "Aplikace nemůže být nainstalována, protože obsahuje značku\n<shipped>\n\ntrue\n</shipped>\n\ncož není povoleno pro nedodávané aplikace");
    translations.insert("App can't be installed because the version in info.xml/version is not the same as the version reported from the app store", "Aplikace nemůže být nainstalována, protože verze uvedená v info.xml/version nesouhlasí s verzí oznámenou z úložiště aplikací.");
    translations.insert("App directory already exists", "Adresář aplikace již existuje");
    translations.insert("Can't create app folder. Please fix permissions. %s", "Nelze vytvořit složku aplikace. Opravte práva souborů. %s");
    translations.insert("Application is not enabled", "Aplikace není povolena");
    translations.insert("Authentication error", "Chyba ověření");
    translations.insert("Token expired. Please reload page.", "Token vypršel. Obnovte prosím stránku.");
    translations.insert("Files", "Soubory");
    translations.insert("Text", "Text");
    translations.insert("Images", "Obrázky");
    translations.insert("%s enter the database username.", "Zadejte uživatelské jméno %s databáze.");
    translations.insert("%s enter the database name.", "Zadejte název databáze pro %s databáze.");
    translations.insert("%s you may not use dots in the database name", "V názvu databáze %s nesmíte používat tečky.");
    translations.insert("MS SQL username and/or password not valid: %s", "Uživatelské jméno či heslo MSSQL není platné: %s");
    translations.insert("You need to enter either an existing account or the administrator.", "Musíte zadat existující účet či správce.");
    translations.insert("MySQL username and/or password not valid", "Uživatelské jméno či heslo MySQL není platné");
    translations.insert("DB Error: \"%s\"", "Chyba databáze: \"%s\"");
    translations.insert("Offending command was: \"%s\"", "Příslušný příkaz byl: \"%s\"");
    translations.insert("MySQL user '%s'@'localhost' exists already.", "Uživatel '%s'@'localhost' již v MySQL existuje.");
    translations.insert("Drop this user from MySQL", "Zrušte tohoto uživatele z MySQL");
    translations.insert("MySQL user '%s'@'%%' already exists", "Uživatel '%s'@'%%' již v MySQL existuje");
    translations.insert("Drop this user from MySQL.", "Zrušte tohoto uživatele z MySQL");
    translations.insert("Oracle connection could not be established", "Spojení s Oracle nemohlo být navázáno");
    translations.insert("Oracle username and/or password not valid", "Uživatelské jméno či heslo Oracle není platné");
    translations.insert("Offending command was: \"%s\", name: %s, password: %s", "Příslušný příkaz byl: \"%s\", jméno: %s, heslo: %s");
    translations.insert("PostgreSQL username and/or password not valid", "Uživatelské jméno či heslo PostgreSQL není platné");
    translations.insert("Set an admin username.", "Zadejte uživatelské jméno správce.");
    translations.insert("Set an admin password.", "Zadejte heslo správce.");
    translations.insert("Your web server is not yet properly setup to allow files synchronization because the WebDAV interface seems to be broken.", "Váš webový server není správně nastaven pro umožnění synchronizace, rozhraní WebDAV se zdá být rozbité.");
    translations.insert("Please double check the <a href='%s'>installation guides</a>.", "Zkonzultujte, prosím, <a href='%s'>průvodce instalací</a>.");
    translations.insert("Could not find category \"%s\"", "Nelze nalézt kategorii \"%s\"");
    translations.insert("seconds ago", "před pár sekundami");
    translations.insert("today", "dnes");
    translations.insert("yesterday", "včera");
    translations.insert("last month", "minulý měsíc");
    translations.insert("last year", "minulý rok");
    translations.insert("years ago", "před lety");
    translations.insert("Caused by:", "Příčina:");
    
    translations
}

#[allow(dead_code)]
pub fn get_plural_forms() -> &'static str {
    "nplurals=3; plural=(n==1) ? 0 : (n>=2 && n<=4) ? 1 : 2;"
}

#[allow(dead_code)]
pub fn get_plural_translations() -> HashMap<&'static str, Vec<&'static str>> {
    let mut plural_translations = HashMap::new();
    
    plural_translations.insert("_%n minute ago_::_%n minutes ago_", 
        vec!["před %n minutou", "před %n minutami", "před %n minutami"]);
    plural_translations.insert("_%n hour ago_::_%n hours ago_", 
        vec!["před %n hodinou", "před %n hodinami", "před %n hodinami"]);
    plural_translations.insert("_%n day go_::_%n days ago_", 
        vec!["před %n dnem", "před %n dny", "před %n dny"]);
    plural_translations.insert("_%n month ago_::_%n months ago_", 
        vec!["před %n měsícem", "před %n měsíci", "před %n měsíci"]);
    
    plural_translations
}

i18n!("cs_CZ");