use rust_i18n::t;
use std::collections::HashMap;
use lazy_static::lazy_static;
use once_cell::sync::Lazy;

pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("Help", "Ndihmë");
    m.insert("Personal", "Personale");
    m.insert("Settings", "Parametra");
    m.insert("Users", "Përdoruesit");
    m.insert("Admin", "Admin");
    m.insert("web services under your control", "shërbime web nën kontrollin tënd");
    m.insert("ZIP download is turned off.", "Shkarimi i skedarëve ZIP është i çaktivizuar.");
    m.insert("Files need to be downloaded one by one.", "Skedarët duhet të shkarkohen një nga një.");
    m.insert("Back to Files", "Kthehu tek skedarët");
    m.insert("Selected files too large to generate zip file.", "Skedarët e selektuar janë shumë të mëdhenj për të krijuar një skedar ZIP.");
    m.insert("Application is not enabled", "Programi nuk është i aktivizuar.");
    m.insert("Authentication error", "Veprim i gabuar gjatë vërtetimit të identitetit");
    m.insert("Token expired. Please reload page.", "Përmbajtja ka skaduar. Ju lutemi ringarkoni faqen.");
    m.insert("Files", "Skedarët");
    m.insert("Text", "Tekst");
    m.insert("Images", "Foto");
    m.insert("%s enter the database username.", "% shkruani përdoruesin e database-it.");
    m.insert("%s enter the database name.", "%s shkruani emrin e database-it.");
    m.insert("%s you may not use dots in the database name", "%s nuk mund të përdorni pikat tek emri i database-it");
    m.insert("MS SQL username and/or password not valid: %s", "Përdoruesi dhe/apo kodi i MS SQL i pavlefshëm: %s");
    m.insert("You need to enter either an existing account or the administrator.", "Duhet të përdorni një llogari ekzistuese ose llogarinë e administratorit.");
    m.insert("MySQL username and/or password not valid", "Përdoruesi dhe/apo kodi i MySQL-it i pavlefshëm.");
    m.insert("DB Error: \"%s\"", "Veprim i gabuar i DB-it: \"%s\"");
    m.insert("Offending command was: \"%s\"", "Komanda e gabuar ishte: \"%s\"");
    m.insert("MySQL user '%s'@'localhost' exists already.", "Përdoruesi MySQL '%s'@'localhost' ekziston.");
    m.insert("Drop this user from MySQL", "Eliminoni këtë përdorues nga MySQL");
    m.insert("MySQL user '%s'@'%%' already exists", "Përdoruesi MySQL '%s'@'%%' ekziston");
    m.insert("Drop this user from MySQL.", "Eliminoni këtë përdorues nga MySQL.");
    m.insert("Oracle username and/or password not valid", "Përdoruesi dhe/apo kodi i Oracle-it i pavlefshëm");
    m.insert("Offending command was: \"%s\", name: %s, password: %s", "Komanda e gabuar ishte: \"%s\", përdoruesi: %s, kodi: %s");
    m.insert("PostgreSQL username and/or password not valid", "Përdoruesi dhe/apo kodi i PostgreSQL i pavlefshëm");
    m.insert("Set an admin username.", "Cakto emrin e administratorit.");
    m.insert("Set an admin password.", "Cakto kodin e administratorit.");
    m.insert("Your web server is not yet properly setup to allow files synchronization because the WebDAV interface seems to be broken.", "Serveri web i juaji nuk është konfiguruar akoma për të lejuar sinkronizimin e skedarëve sepse ndërfaqja WebDAV mund të jetë e dëmtuar.");
    m.insert("Please double check the <a href='%s'>installation guides</a>.", "Ju lutemi kontrolloni mirë <a href='%s'>shoqëruesin e instalimit</a>.");
    m.insert("Could not find category \"%s\"", "Kategoria \"%s\" nuk u gjet");
    m.insert("seconds ago", "sekonda më parë");
    m.insert("_%n minute ago_::_%n minutes ago_", "%n minuta më parë");
    m.insert("_%n hour ago_::_%n hours ago_", "%n orë më parë");
    m.insert("today", "sot");
    m.insert("yesterday", "dje");
    m.insert("_%n day go_::_%n days ago_", "%n ditë më parë");
    m.insert("last month", "muajin e shkuar");
    m.insert("_%n month ago_::_%n months ago_", "%n muaj më parë");
    m.insert("last year", "vitin e shkuar");
    m.insert("years ago", "vite më parë");
    m
});

pub const PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";

lazy_static! {
    static ref PLURAL_FORMS_FN: Box<dyn Fn(usize) -> usize> = {
        Box::new(|n| if n != 1 { 1 } else { 0 })
    };
}

pub fn get_plural_index(n: usize) -> usize {
    PLURAL_FORMS_FN(n)
}

pub fn get_translation(key: &str) -> &'static str {
    TRANSLATIONS.get(key).copied().unwrap_or(key)
}

pub fn get_plural_translation(key: &str, n: usize) -> &'static str {
    let index = get_plural_index(n);
    let plural_key = if let Some(base_key) = key.split("_::_").nth(0) {
        if index > 0 && key.contains("_::_") {
            if let Some(plural_form) = key.split("_::_").nth(index) {
                plural_form
            } else {
                key
            }
        } else {
            base_key
        }
    } else {
        key
    };
    
    get_translation(plural_key)
}