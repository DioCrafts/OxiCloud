use lazy_static::lazy_static;
use std::collections::HashMap;
use rust_fluent_templates::Loader;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("App \"%s\" can't be installed because it is not compatible with this version of ownCloud.", "Appen \"%s\" kan inte installeras eftersom att den inte är kompatibel med denna version av ownCloud.");
        m.insert("No app name specified", "Inget appnamn angivet");
        m.insert("Help", "Hjälp");
        m.insert("Personal", "Personligt");
        m.insert("Settings", "Inställningar");
        m.insert("Users", "Användare");
        m.insert("Admin", "Admin");
        m.insert("Failed to upgrade \"%s\".", "Misslyckades med att uppgradera \"%s\".");
        m.insert("Unknown filetype", "Okänd filtyp");
        m.insert("Invalid image", "Ogiltig bild");
        m.insert("web services under your control", "webbtjänster under din kontroll");
        m.insert("cannot open \"%s\"", "Kan inte öppna \"%s\"");
        m.insert("ZIP download is turned off.", "Nerladdning av ZIP är avstängd.");
        m.insert("Files need to be downloaded one by one.", "Filer laddas ner en åt gången.");
        m.insert("Back to Files", "Tillbaka till Filer");
        m.insert("Selected files too large to generate zip file.", "Valda filer är för stora för att skapa zip-fil.");
        m.insert("Download the files in smaller chunks, seperately or kindly ask your administrator.", "Ladda ner filerna i mindre bitar, separat eller fråga din administratör.");
        m.insert("No source specified when installing app", "Ingen källa angiven vid installation av app ");
        m.insert("No href specified when installing app from http", "Ingen href angiven vid installation av app från http");
        m.insert("No path specified when installing app from local file", "Ingen sökväg angiven vid installation av app från lokal fil");
        m.insert("Archives of type %s are not supported", "Arkiv av typen %s stöds ej");
        m.insert("Failed to open archive when installing app", "Kunde inte öppna arkivet när appen skulle installeras");
        m.insert("App does not provide an info.xml file", "Appen har ingen info.xml fil");
        m.insert("App can't be installed because of not allowed code in the App", "Appen kan inte installeras eftersom att den innehåller otillåten kod");
        m.insert("App can't be installed because it is not compatible with this version of ownCloud", "Appen kan inte installeras eftersom att den inte är kompatibel med denna version av ownCloud");
        m.insert("App can't be installed because it contains the <shipped>true</shipped> tag which is not allowed for non shipped apps", "Appen kan inte installeras eftersom att den innehåller etiketten <shipped>true</shipped> vilket inte är tillåtet för icke inkluderade appar");
        m.insert("App can't be installed because the version in info.xml/version is not the same as the version reported from the app store", "Appen kan inte installeras eftersom versionen i info.xml inte är samma som rapporteras från app store");
        m.insert("App directory already exists", "Appens mapp finns redan");
        m.insert("Can't create app folder. Please fix permissions. %s", "Kan inte skapa appens mapp. Var god åtgärda rättigheterna. %s");
        m.insert("Application is not enabled", "Applikationen är inte aktiverad");
        m.insert("Authentication error", "Fel vid autentisering");
        m.insert("Token expired. Please reload page.", "Ogiltig token. Ladda om sidan.");
        m.insert("Files", "Filer");
        m.insert("Text", "Text");
        m.insert("Images", "Bilder");
        m.insert("%s enter the database username.", "%s ange databasanvändare.");
        m.insert("%s enter the database name.", "%s ange databasnamn");
        m.insert("%s you may not use dots in the database name", "%s du får inte använda punkter i databasnamnet");
        m.insert("MS SQL username and/or password not valid: %s", "MS SQL-användaren och/eller lösenordet var inte giltigt: %s");
        m.insert("You need to enter either an existing account or the administrator.", "Du måste antingen ange ett befintligt konto eller administratör.");
        m.insert("MySQL username and/or password not valid", "MySQL-användarnamnet och/eller lösenordet är felaktigt");
        m.insert("DB Error: \"%s\"", "DB error: \"%s\"");
        m.insert("Offending command was: \"%s\"", "Det felaktiga kommandot var: \"%s\"");
        m.insert("MySQL user '%s'@'localhost' exists already.", "MySQL-användaren '%s'@'localhost' existerar redan.");
        m.insert("Drop this user from MySQL", "Radera denna användare från MySQL");
        m.insert("MySQL user '%s'@'%%' already exists", "MySQl-användare '%s'@'%%' existerar redan");
        m.insert("Drop this user from MySQL.", "Radera denna användare från MySQL.");
        m.insert("Oracle connection could not be established", "Oracle-anslutning kunde inte etableras");
        m.insert("Oracle username and/or password not valid", "Oracle-användarnamnet och/eller lösenordet är felaktigt");
        m.insert("Offending command was: \"%s\", name: %s, password: %s", "Det felande kommandot var: \"%s\", name: %s, password: %s");
        m.insert("PostgreSQL username and/or password not valid", "PostgreSQL-användarnamnet och/eller lösenordet är felaktigt");
        m.insert("Set an admin username.", "Ange ett användarnamn för administratören.");
        m.insert("Set an admin password.", "Ange ett administratörslösenord.");
        m.insert("Your web server is not yet properly setup to allow files synchronization because the WebDAV interface seems to be broken.", "Din webbserver är inte korrekt konfigurerad för att tillåta filsynkronisering eftersom WebDAV inte verkar fungera.");
        m.insert("Please double check the <a href='%s'>installation guides</a>.", "Var god kontrollera <a href='%s'>installationsguiden</a>.");
        m.insert("Could not find category \"%s\"", "Kunde inte hitta kategorin \"%s\"");
        m.insert("seconds ago", "sekunder sedan");
        m.insert("_%n minute ago_::_%n minutes ago_", "_%n minut sedan_::_%n minuter sedan_");
        m.insert("_%n hour ago_::_%n hours ago_", "_%n timme sedan_::_%n timmar sedan_");
        m.insert("today", "i dag");
        m.insert("yesterday", "i går");
        m.insert("_%n day go_::_%n days ago_", "_%n dag sedan_::_%n dagar sedan_");
        m.insert("last month", "förra månaden");
        m.insert("_%n month ago_::_%n months ago_", "_%n månad sedan_::_%n månader sedan_");
        m.insert("last year", "förra året");
        m.insert("years ago", "år sedan");
        m.insert("Caused by:", "Orsakad av:");
        m
    };
}

pub static PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";

pub struct SwedishTranslator;

impl SwedishTranslator {
    pub fn new() -> Self {
        SwedishTranslator
    }

    pub fn translate(&self, key: &str) -> &str {
        TRANSLATIONS.get(key).unwrap_or(key)
    }

    pub fn translate_plural(&self, singular: &str, plural: &str, count: usize) -> &str {
        if let Some(translation_pattern) = TRANSLATIONS.get(
            &format!("_{}n {} {}_::_{}n {} {}_", 
                    '%', singular, '%', plural)
        ) {
            let parts: Vec<&str> = translation_pattern.split("_::_").collect();
            if count == 1 && parts.len() > 0 {
                return parts[0].replacen("%n", &count.to_string(), 1);
            } else if count != 1 && parts.len() > 1 {
                return parts[1].replacen("%n", &count.to_string(), 1);
            }
        }
        if count == 1 { singular } else { plural }
    }

    pub fn translate_sprintf(&self, key: &str, args: &[&str]) -> String {
        let template = self.translate(key);
        let mut result = String::from(template);
        
        for (i, arg) in args.iter().enumerate() {
            result = result.replacen(&format!("%s"), arg, 1);
        }
        
        result
    }
}