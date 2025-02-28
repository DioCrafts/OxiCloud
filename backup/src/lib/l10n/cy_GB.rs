use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Help", "Cymorth");
        m.insert("Personal", "Personol");
        m.insert("Settings", "Gosodiadau");
        m.insert("Users", "Defnyddwyr");
        m.insert("Admin", "Gweinyddu");
        m.insert("web services under your control", "gwasanaethau gwe a reolir gennych");
        m.insert("ZIP download is turned off.", "Mae llwytho ZIP wedi ei ddiffodd.");
        m.insert("Files need to be downloaded one by one.", "Mae angen llwytho ffeiliau i lawr fesul un.");
        m.insert("Back to Files", "Nôl i Ffeiliau");
        m.insert("Selected files too large to generate zip file.", "Mae'r ffeiliau ddewiswyd yn rhy fawr i gynhyrchu ffeil zip.");
        m.insert("Application is not enabled", "Nid yw'r pecyn wedi'i alluogi");
        m.insert("Authentication error", "Gwall dilysu");
        m.insert("Token expired. Please reload page.", "Tocyn wedi dod i ben. Ail-lwythwch y dudalen.");
        m.insert("Files", "Ffeiliau");
        m.insert("Text", "Testun");
        m.insert("Images", "Delweddau");
        m.insert("%s enter the database username.", "%s rhowch enw defnyddiwr y gronfa ddata.");
        m.insert("%s enter the database name.", "%s rhowch enw'r gronfa ddata.");
        m.insert("%s you may not use dots in the database name", "%s does dim hawl defnyddio dot yn enw'r gronfa ddata");
        m.insert("MS SQL username and/or password not valid: %s", "Enw a/neu gyfrinair MS SQL annilys: %s");
        m.insert("You need to enter either an existing account or the administrator.", "Rhaid i chi naill ai gyflwyno cyfrif presennol neu'r gweinyddwr.");
        m.insert("MySQL username and/or password not valid", "Enw a/neu gyfrinair MySQL annilys");
        m.insert("DB Error: \"%s\"", "Gwall DB: \"%s\"");
        m.insert("Offending command was: \"%s\"", "Y gorchymyn wnaeth beri tramgwydd oedd: \"%s\"");
        m.insert("MySQL user '%s'@'localhost' exists already.", "Defnyddiwr MySQL '%s'@'localhost' yn bodoli eisoes.");
        m.insert("Drop this user from MySQL", "Gollwng y defnyddiwr hwn o MySQL");
        m.insert("MySQL user '%s'@'%%' already exists", "Defnyddiwr MySQL '%s'@'%%' eisoes yn bodoli");
        m.insert("Drop this user from MySQL.", "Gollwng y defnyddiwr hwn o MySQL.");
        m.insert("Oracle username and/or password not valid", "Enw a/neu gyfrinair Oracle annilys");
        m.insert("Offending command was: \"%s\", name: %s, password: %s", "Y gorchymyn wnaeth beri tramgwydd oedd: \"%s\", enw: %s, cyfrinair: %s");
        m.insert("PostgreSQL username and/or password not valid", "Enw a/neu gyfrinair PostgreSQL annilys");
        m.insert("Set an admin username.", "Creu enw defnyddiwr i'r gweinyddwr.");
        m.insert("Set an admin password.", "Gosod cyfrinair y gweinyddwr.");
        m.insert("Your web server is not yet properly setup to allow files synchronization because the WebDAV interface seems to be broken.", "Nid yw eich gweinydd wedi'i gyflunio eto i ganiatáu cydweddu ffeiliau oherwydd bod y rhyngwyneb WebDAV wedi torri.");
        m.insert("Please double check the <a href='%s'>installation guides</a>.", "Gwiriwch y <a href='%s'>canllawiau gosod</a> eto.");
        m.insert("Could not find category \"%s\"", "Methu canfod categori \"%s\"");
        m.insert("seconds ago", "eiliad yn ôl");
        m.insert("today", "heddiw");
        m.insert("yesterday", "ddoe");
        m.insert("last month", "mis diwethaf");
        m.insert("last year", "y llynedd");
        m.insert("years ago", "blwyddyn yn ôl");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=4; plural=(n==1) ? 0 : (n==2) ? 1 : (n != 8 && n != 11) ? 2 : 3;";

    pub static ref PLURAL_TRANSLATIONS: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("_%n minute ago_::_%n minutes ago_", vec!["", "", "", ""]);
        m.insert("_%n hour ago_::_%n hours ago_", vec!["", "", "", ""]);
        m.insert("_%n day go_::_%n days ago_", vec!["", "", "", ""]);
        m.insert("_%n month ago_::_%n months ago_", vec!["", "", "", ""]);
        m
    };
}

pub struct CyGB;

impl CyGB {
    pub fn get_translation(&self, key: &str) -> Option<&'static str> {
        TRANSLATIONS.get(key).copied()
    }

    pub fn get_plural_translation(&self, key: &str, count: usize) -> Option<&'static str> {
        PLURAL_TRANSLATIONS.get(key).and_then(|forms| {
            let idx = if count == 1 {
                0
            } else if count == 2 {
                1
            } else if count != 8 && count != 11 {
                2
            } else {
                3
            };
            forms.get(idx).copied()
        })
    }
}