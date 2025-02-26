use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("Help", "Helpo");
    m.insert("Personal", "Persona");
    m.insert("Settings", "Agordo");
    m.insert("Users", "Uzantoj");
    m.insert("Admin", "Administranto");
    m.insert("web services under your control", "TTT-servoj regataj de vi");
    m.insert("ZIP download is turned off.", "ZIP-elŝuto estas malkapabligita.");
    m.insert("Files need to be downloaded one by one.", "Dosieroj devas elŝutiĝi unuope.");
    m.insert("Back to Files", "Reen al la dosieroj");
    m.insert("Selected files too large to generate zip file.", "La elektitaj dosieroj tro grandas por genero de ZIP-dosiero.");
    m.insert("Application is not enabled", "La aplikaĵo ne estas kapabligita");
    m.insert("Authentication error", "Aŭtentiga eraro");
    m.insert("Token expired. Please reload page.", "Ĵetono eksvalidiĝis. Bonvolu reŝargi la paĝon.");
    m.insert("Files", "Dosieroj");
    m.insert("Text", "Teksto");
    m.insert("Images", "Bildoj");
    m.insert("%s enter the database username.", "%s enigu la uzantonomon de la datumbazo.");
    m.insert("%s enter the database name.", "%s enigu la nomon de la datumbazo.");
    m.insert("%s you may not use dots in the database name", "%s vi ne povas uzi punktojn en la nomo de la datumbazo");
    m.insert("MS SQL username and/or password not valid: %s", "La uzantonomo de MS SQL aŭ la pasvorto ne validas: %s");
    m.insert("MySQL username and/or password not valid", "La uzantonomo de MySQL aŭ la pasvorto ne validas");
    m.insert("DB Error: \"%s\"", "Datumbaza eraro: "%s"");
    m.insert("MySQL user '%s'@'localhost' exists already.", "La uzanto de MySQL "%s"@"localhost" jam ekzistas.");
    m.insert("Drop this user from MySQL", "Forigi ĉi tiun uzanton el MySQL");
    m.insert("MySQL user '%s'@'%%' already exists", "La uzanto de MySQL "%s"@"%%" jam ekzistas");
    m.insert("Drop this user from MySQL.", "Forigi ĉi tiun uzanton el MySQL.");
    m.insert("Oracle connection could not be established", "Konekto al Oracle ne povas stariĝi");
    m.insert("Oracle username and/or password not valid", "La uzantonomo de Oracle aŭ la pasvorto ne validas");
    m.insert("PostgreSQL username and/or password not valid", "La uzantonomo de PostgreSQL aŭ la pasvorto ne validas");
    m.insert("Set an admin username.", "Starigi administran uzantonomon.");
    m.insert("Set an admin password.", "Starigi administran pasvorton.");
    m.insert("Your web server is not yet properly setup to allow files synchronization because the WebDAV interface seems to be broken.", "Via TTT-servilo ankoraŭ ne ĝuste agordiĝis por permesi sinkronigi dosierojn ĉar la WebDAV-interfaco ŝajnas rompita.");
    m.insert("Please double check the <a href='%s'>installation guides</a>.", "Bonvolu duoble kontroli la <a href='%s'>gvidilon por instalo</a>.");
    m.insert("Could not find category \"%s\"", "Ne troviĝis kategorio "%s"");
    m.insert("seconds ago", "sekundoj antaŭe");
    m.insert("today", "hodiaŭ");
    m.insert("yesterday", "hieraŭ");
    m.insert("last month", "lastamonate");
    m.insert("last year", "lastajare");
    m.insert("years ago", "jaroj antaŭe");
    m
});

pub static PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";

pub type PluralForm = fn(n: i64) -> usize;

pub struct L10n {
    pub translations: &'static HashMap<&'static str, &'static str>,
    pub plural_forms: &'static str,
    pub plural_function: PluralForm,
}

impl L10n {
    pub fn new() -> Self {
        L10n {
            translations: &TRANSLATIONS,
            plural_forms: PLURAL_FORMS,
            plural_function: |n| if n != 1 { 1 } else { 0 },
        }
    }

    pub fn get_translation(&self, key: &str) -> Option<&'static str> {
        self.translations.get(key).copied()
    }
}

impl Default for L10n {
    fn default() -> Self {
        Self::new()
    }
}