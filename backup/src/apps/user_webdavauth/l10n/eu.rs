use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("WebDAV Authentication", "WebDAV Autentikazioa");
        m.insert("Address: ", "Helbidea:");
        m.insert(
            "The user credentials will be sent to this address. This plugin checks the response and will interpret the HTTP statuscodes 401 and 403 as invalid credentials, and all other responses as valid credentials.",
            "Erabiltzailearen kredentzialak helbide honetara bidaliko dira. Plugin honek erantzuna aztertu eta HTTP 401 eta 403 egoera-kodeak kredentzial ez-egokitzat hartuko ditu, eta beste edozein erantzun, aldiz, kredentzial egokitzat."
        );
        m
    };
    
    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}