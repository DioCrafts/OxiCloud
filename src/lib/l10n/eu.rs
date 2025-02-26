use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Help", "Laguntza");
        m.insert("Personal", "Pertsonala");
        m.insert("Settings", "Ezarpenak");
        m.insert("Users", "Erabiltzaileak");
        m.insert("Admin", "Admin");
        m.insert("Failed to upgrade \"%s\".", "Ezin izan da \"%s\" eguneratu.");
        m.insert("web services under your control", "web zerbitzuak zure kontrolpean");
        m.insert("cannot open \"%s\"", "ezin da \"%s\" ireki");
        m.insert("ZIP download is turned off.", "ZIP deskarga ez dago gaituta.");
        m.insert("Files need to be downloaded one by one.", "Fitxategiak banan-banan deskargatu behar dira.");
        m.insert("Back to Files", "Itzuli fitxategietara");
        m.insert("Selected files too large to generate zip file.", "Hautatuko fitxategiak oso handiak dira zip fitxategia sortzeko.");
        m.insert("Download the files in smaller chunks, seperately or kindly ask your administrator.", "Deskargatu fitzategiak zati txikiagoetan, banan-banan edo eskatu mesedez zure administradoreari");
        m.insert("Application is not enabled", "Aplikazioa ez dago gaituta");
        m.insert("Authentication error", "Autentifikazio errorea");
        m.insert("Token expired. Please reload page.", "Tokena iraungitu da. Mesedez birkargatu orria.");
        m.insert("Files", "Fitxategiak");
        m.insert("Text", "Testua");
        m.insert("Images", "Irudiak");
        m.insert("%s enter the database username.", "%s sartu datu basearen erabiltzaile izena.");
        m.insert("%s enter the database name.", "%s sartu datu basearen izena.");
        m.insert("%s you may not use dots in the database name", "%s ezin duzu punturik erabili datu basearen izenean.");
        m.insert("MS SQL username and/or password not valid: %s", "MS SQL erabiltzaile izena edota pasahitza ez dira egokiak: %s");
        m.insert("You need to enter either an existing account or the administrator.", "Existitzen den kontu bat edo administradorearena jarri behar duzu.");
        m.insert("MySQL username and/or password not valid", "MySQL erabiltzaile edota pasahitza ez dira egokiak.");
        m.insert("DB Error: \"%s\"", "DB errorea: \"%s\"");
        m.insert("Offending command was: \"%s\"", "Errorea komando honek sortu du: \"%s\"");
        m.insert("MySQL user '%s'@'localhost' exists already.", "MySQL '%s'@'localhost' erabiltzailea dagoeneko existitzen da.");
        m.insert("Drop this user from MySQL", "Ezabatu erabiltzaile hau MySQLtik");
        m.insert("MySQL user '%s'@'%%' already exists", "MySQL '%s'@'%%' erabiltzailea dagoeneko existitzen da");
        m.insert("Drop this user from MySQL.", "Ezabatu erabiltzaile hau MySQLtik.");
        m.insert("Oracle connection could not be established", "Ezin da Oracle konexioa sortu");
        m.insert("Oracle username and/or password not valid", "Oracle erabiltzaile edota pasahitza ez dira egokiak.");
        m.insert("Offending command was: \"%s\", name: %s, password: %s", "Errorea komando honek sortu du: \"%s\", izena: %s, pasahitza: %s");
        m.insert("PostgreSQL username and/or password not valid", "PostgreSQL erabiltzaile edota pasahitza ez dira egokiak.");
        m.insert("Set an admin username.", "Ezarri administraziorako erabiltzaile izena.");
        m.insert("Set an admin password.", "Ezarri administraziorako pasahitza.");
        m.insert("Your web server is not yet properly setup to allow files synchronization because the WebDAV interface seems to be broken.", "Zure web zerbitzaria ez dago oraindik ongi konfiguratuta fitxategien sinkronizazioa egiteko, WebDAV interfazea ongi ez dagoela dirudi.");
        m.insert("Please double check the <a href='%s'>installation guides</a>.", "Mesedez begiratu <a href='%s'>instalazio gidak</a>.");
        m.insert("Could not find category \"%s\"", "Ezin da \"%s\" kategoria aurkitu");
        m.insert("seconds ago", "segundu");
        m.insert("today", "gaur");
        m.insert("yesterday", "atzo");
        m.insert("last month", "joan den hilabetean");
        m.insert("last year", "joan den urtean");
        m.insert("years ago", "urte");
        m.insert("Caused by:", "Honek eraginda:");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";

    pub static ref PLURAL_TRANSLATIONS: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("_%n minute ago_::_%n minutes ago_", vec!["orain dela minutu %n", "orain dela %n minutu"]);
        m.insert("_%n hour ago_::_%n hours ago_", vec!["orain dela ordu %n", "orain dela %n ordu"]);
        m.insert("_%n day go_::_%n days ago_", vec!["orain dela egun %n", "orain dela %n egun"]);
        m.insert("_%n month ago_::_%n months ago_", vec!["orain dela hilabete %n", "orain dela %n hilabete"]);
        m
    };
}

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

pub fn get_plural_translation(key: &str, count: i64) -> Option<&'static str> {
    PLURAL_TRANSLATIONS.get(key).and_then(|forms| {
        let index = if count != 1 { 1 } else { 0 };
        forms.get(index).copied()
    })
}