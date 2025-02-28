use std::collections::HashMap;
use rust_i18n::LocalizationArgs;

pub fn get_translations() -> HashMap<&'static str, String> {
    let mut translations: HashMap<&'static str, String> = HashMap::new();
    
    translations.insert("App \"%s\" can't be installed because it is not compatible with this version of ownCloud.", String::from("App'en \"%s\" kan ikke blive installeret, da den ikke er kompatibel med denne version af ownCloud."));
    translations.insert("No app name specified", String::from("Intet app-navn angivet"));
    translations.insert("Help", String::from("Hjælp"));
    translations.insert("Personal", String::from("Personligt"));
    translations.insert("Settings", String::from("Indstillinger"));
    translations.insert("Users", String::from("Brugere"));
    translations.insert("Admin", String::from("Admin"));
    translations.insert("Failed to upgrade \"%s\".", String::from("Upgradering af \"%s\" fejlede"));
    translations.insert("Unknown filetype", String::from("Ukendt filtype"));
    translations.insert("Invalid image", String::from("Ugyldigt billede"));
    translations.insert("web services under your control", String::from("Webtjenester under din kontrol"));
    translations.insert("cannot open \"%s\"", String::from("Kan ikke åbne \"%s\""));
    translations.insert("ZIP download is turned off.", String::from("ZIP-download er slået fra."));
    translations.insert("Files need to be downloaded one by one.", String::from("Filer skal downloades en for en."));
    translations.insert("Back to Files", String::from("Tilbage til Filer"));
    translations.insert("Selected files too large to generate zip file.", String::from("De markerede filer er for store til at generere en ZIP-fil."));
    translations.insert("Download the files in smaller chunks, seperately or kindly ask your administrator.", String::from("Download filerne i små bider, seperat, eller kontakt venligst din administrator."));
    translations.insert("No source specified when installing app", String::from("Ingen kilde angivet under installation af app"));
    translations.insert("No href specified when installing app from http", String::from("Ingen href angivet under installation af app via http"));
    translations.insert("No path specified when installing app from local file", String::from("Ingen sti angivet under installation af app fra lokal fil"));
    translations.insert("Archives of type %s are not supported", String::from("Arkiver af type %s understøttes ikke"));
    translations.insert("Failed to open archive when installing app", String::from("Kunne ikke åbne arkiv under installation af appen"));
    translations.insert("App does not provide an info.xml file", String::from("Der følger ingen info.xml-fil med appen"));
    translations.insert("App can't be installed because of not allowed code in the App", String::from("Appen kan ikke installeres, da den indeholder ikke-tilladt kode"));
    translations.insert("App can't be installed because it is not compatible with this version of ownCloud", String::from("Appen kan ikke installeres, da den ikke er kompatibel med denne version af ownCloud."));
    translations.insert("App can't be installed because it contains the <shipped>true</shipped> tag which is not allowed for non shipped apps", String::from("Appen kan ikke installeres, da den indeholder taget\n<shipped>\n\ntrue\n</shipped>\n\nhvilket ikke er tilladt for ikke-medfølgende apps"));
    translations.insert("App can't be installed because the version in info.xml/version is not the same as the version reported from the app store", String::from("App kan ikke installeres, da versionen i info.xml/version ikke er den samme som versionen rapporteret fra app-storen"));
    translations.insert("App directory already exists", String::from("App-mappe findes allerede"));
    translations.insert("Can't create app folder. Please fix permissions. %s", String::from("Kan ikke oprette app-mappe. Ret tilladelser. %s"));
    translations.insert("Application is not enabled", String::from("Programmet er ikke aktiveret"));
    translations.insert("Authentication error", String::from("Adgangsfejl"));
    translations.insert("Token expired. Please reload page.", String::from("Adgang er udløbet. Genindlæs siden."));
    translations.insert("Files", String::from("Filer"));
    translations.insert("Text", String::from("SMS"));
    translations.insert("Images", String::from("Billeder"));
    translations.insert("%s enter the database username.", String::from("%s indtast database brugernavnet."));
    translations.insert("%s enter the database name.", String::from("%s indtast database navnet."));
    translations.insert("%s you may not use dots in the database name", String::from("%s du må ikke bruge punktummer i databasenavnet."));
    translations.insert("MS SQL username and/or password not valid: %s", String::from("MS SQL brugernavn og/eller adgangskode ikke er gyldigt: %s"));
    translations.insert("You need to enter either an existing account or the administrator.", String::from("Du bliver nødt til at indtaste en eksisterende bruger eller en administrator."));
    translations.insert("MySQL username and/or password not valid", String::from("MySQL brugernavn og/eller kodeord er ikke gyldigt."));
    translations.insert("DB Error: \"%s\"", String::from("Databasefejl: \"%s\""));
    translations.insert("Offending command was: \"%s\"", String::from("Fejlende kommando var: \"%s\""));
    translations.insert("MySQL user '%s'@'localhost' exists already.", String::from("MySQL brugeren '%s'@'localhost' eksisterer allerede."));
    translations.insert("Drop this user from MySQL", String::from("Slet denne bruger fra MySQL"));
    translations.insert("MySQL user '%s'@'%%' already exists", String::from("MySQL brugeren '%s'@'%%' eksisterer allerede."));
    translations.insert("Drop this user from MySQL.", String::from("Slet denne bruger fra MySQL"));
    translations.insert("Oracle connection could not be established", String::from("Oracle forbindelsen kunne ikke etableres"));
    translations.insert("Oracle username and/or password not valid", String::from("Oracle brugernavn og/eller kodeord er ikke gyldigt."));
    translations.insert("Offending command was: \"%s\", name: %s, password: %s", String::from("Fejlende kommando var: \"%s\", navn: %s, password: %s"));
    translations.insert("PostgreSQL username and/or password not valid", String::from("PostgreSQL brugernavn og/eller kodeord er ikke gyldigt."));
    translations.insert("Set an admin username.", String::from("Angiv et admin brugernavn."));
    translations.insert("Set an admin password.", String::from("Angiv et admin kodeord."));
    translations.insert("Your web server is not yet properly setup to allow files synchronization because the WebDAV interface seems to be broken.", String::from("Din webserver er endnu ikke sat op til at tillade fil synkronisering fordi WebDAV grænsefladen virker ødelagt."));
    translations.insert("Please double check the <a href='%s'>installation guides</a>.", String::from("Dobbelttjek venligst <a href='%s'>installations vejledningerne</a>."));
    translations.insert("Could not find category \"%s\"", String::from("Kunne ikke finde kategorien \"%s\""));
    translations.insert("seconds ago", String::from("sekunder siden"));
    translations.insert("today", String::from("i dag"));
    translations.insert("yesterday", String::from("i går"));
    translations.insert("last month", String::from("sidste måned"));
    translations.insert("last year", String::from("sidste år"));
    translations.insert("years ago", String::from("år siden"));
    translations.insert("Caused by:", String::from("Forårsaget af:"));
    
    translations
}

pub fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}

pub fn get_plural_translations() -> HashMap<&'static str, Vec<String>> {
    let mut plural_translations: HashMap<&'static str, Vec<String>> = HashMap::new();
    
    plural_translations.insert("_%n minute ago_::_%n minutes ago_", vec![
        String::from("%n minut siden"),
        String::from("%n minutter siden"),
    ]);
    
    plural_translations.insert("_%n hour ago_::_%n hours ago_", vec![
        String::from("%n time siden"),
        String::from("%n timer siden"),
    ]);
    
    plural_translations.insert("_%n day go_::_%n days ago_", vec![
        String::from("%n dag siden"),
        String::from("%n dage siden"),
    ]);
    
    plural_translations.insert("_%n month ago_::_%n months ago_", vec![
        String::from("%n måned siden"),
        String::from("%n måneder siden"),
    ]);
    
    plural_translations
}

pub struct DanishLocalization;

impl DanishLocalization {
    pub fn new() -> Self {
        DanishLocalization
    }
    
    pub fn translate(&self, key: &str) -> String {
        let translations = get_translations();
        translations.get(key).cloned().unwrap_or_else(|| key.to_string())
    }
    
    pub fn translate_plural(&self, key: &str, count: i64) -> String {
        let plural_translations = get_plural_translations();
        if let Some(forms) = plural_translations.get(key) {
            let plural_form = if count != 1 { 1 } else { 0 };
            if let Some(translation) = forms.get(plural_form) {
                return translation.replace("%n", &count.to_string());
            }
        }
        key.to_string()
    }
}