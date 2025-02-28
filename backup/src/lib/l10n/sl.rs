use std::collections::HashMap;
use rust_i18n::translation_args;

lazy_static::lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("Help", "Pomoč");
        map.insert("Personal", "Osebno");
        map.insert("Settings", "Nastavitve");
        map.insert("Users", "Uporabniki");
        map.insert("Admin", "Skrbništvo");
        map.insert("web services under your control", "spletne storitve pod vašim nadzorom");
        map.insert("ZIP download is turned off.", "Prejemanje datotek v paketu ZIP je onemogočeno.");
        map.insert("Files need to be downloaded one by one.", "Datoteke je mogoče prejeti le posamično.");
        map.insert("Back to Files", "Nazaj na datoteke");
        map.insert("Selected files too large to generate zip file.", "Izbrane datoteke so prevelike za ustvarjanje datoteke arhiva zip.");
        map.insert("Application is not enabled", "Program ni omogočen");
        map.insert("Authentication error", "Napaka pri overjanju");
        map.insert("Token expired. Please reload page.", "Žeton je potekel. Stran je treba ponovno naložiti.");
        map.insert("Files", "Datoteke");
        map.insert("Text", "Besedilo");
        map.insert("Images", "Slike");
        map.insert("%s enter the database username.", "%s - vnos uporabniškega imena podatkovne zbirke.");
        map.insert("%s enter the database name.", "%s - vnos imena podatkovne zbirke.");
        map.insert("%s you may not use dots in the database name", "%s - v imenu podatkovne zbirke ni dovoljeno uporabljati pik.");
        map.insert("MS SQL username and/or password not valid: %s", "Uporabniško ime ali geslo MS SQL ni veljavno: %s");
        map.insert("You need to enter either an existing account or the administrator.", "Prijaviti se je treba v obstoječi ali pa skrbniški račun.");
        map.insert("MySQL username and/or password not valid", "Uporabniško ime ali geslo MySQL ni veljavno");
        map.insert("DB Error: \"%s\"", "Napaka podatkovne zbirke: \"%s\"");
        map.insert("Offending command was: \"%s\"", "Napačni ukaz je: \"%s\"");
        map.insert("MySQL user '%s'@'localhost' exists already.", "Uporabnik MySQL '%s'@'localhost' že obstaja.");
        map.insert("Drop this user from MySQL", "Odstrani uporabnika s podatkovne zbirke MySQL");
        map.insert("MySQL user '%s'@'%%' already exists", "Uporabnik MySQL '%s'@'%%' že obstaja.");
        map.insert("Drop this user from MySQL.", "Odstrani uporabnika s podatkovne zbirke MySQL");
        map.insert("Oracle connection could not be established", "Povezava z bazo Oracle ni uspela.");
        map.insert("Oracle username and/or password not valid", "Uporabniško ime ali geslo Oracle ni veljavno");
        map.insert("Offending command was: \"%s\", name: %s, password: %s", "Napačni ukaz je: \"%s\", ime: %s, geslo: %s");
        map.insert("PostgreSQL username and/or password not valid", "Uporabniško ime ali geslo PostgreSQL ni veljavno");
        map.insert("Set an admin username.", "Nastavi uporabniško ime skrbnika.");
        map.insert("Set an admin password.", "Nastavi geslo skrbnika.");
        map.insert("Your web server is not yet properly setup to allow files synchronization because the WebDAV interface seems to be broken.", "Spletni stražnik še ni ustrezno nastavljen in ne omogoča usklajevanja, saj je nastavitev WebDAV okvarjena.");
        map.insert("Please double check the <a href='%s'>installation guides</a>.", "Preverite <a href='%s'>navodila namestitve</a>.");
        map.insert("Could not find category \"%s\"", "Kategorije \"%s\" ni mogoče najti.");
        map.insert("seconds ago", "pred nekaj sekundami");
        map.insert("today", "danes");
        map.insert("yesterday", "včeraj");
        map.insert("last month", "zadnji mesec");
        map.insert("last year", "lansko leto");
        map.insert("years ago", "let nazaj");
        map
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=4; plural=(n%100==1 ? 0 : n%100==2 ? 1 : n%100==3 || n%100==4 ? 2 : 3);";
    
    pub static ref PLURAL_TRANSLATIONS: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("_%n minute ago_::_%n minutes ago_", vec!["", "", "", ""]);
        map.insert("_%n hour ago_::_%n hours ago_", vec!["", "", "", ""]);
        map.insert("_%n day go_::_%n days ago_", vec!["", "", "", ""]);
        map.insert("_%n month ago_::_%n months ago_", vec!["", "", "", ""]);
        map
    };
}

pub fn get_translation(key: &str) -> &'static str {
    TRANSLATIONS.get(key).copied().unwrap_or(key)
}

pub fn get_plural_translation(key: &str, count: i64) -> &'static str {
    if let Some(forms) = PLURAL_TRANSLATIONS.get(key) {
        let plural_index = calculate_plural_index(count);
        if plural_index < forms.len() {
            return forms[plural_index];
        }
    }
    ""
}

fn calculate_plural_index(n: i64) -> usize {
    let n_mod_100 = n % 100;
    
    if n_mod_100 == 1 {
        0
    } else if n_mod_100 == 2 {
        1
    } else if n_mod_100 == 3 || n_mod_100 == 4 {
        2
    } else {
        3
    }
}