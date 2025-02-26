use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut translations = HashMap::new();
    translations.insert("WebDAV Authentication", "WebDAV overenie");
    translations.insert("Address: ", "Adresa: ");
    translations.insert(
        "The user credentials will be sent to this address. This plugin checks the response and will interpret the HTTP statuscodes 401 and 403 as invalid credentials, and all other responses as valid credentials.",
        "Používateľské prihlasovacie údaje budú odoslané na túto adresu. Tento plugin skontroluje odpoveď servera a interpretuje návratový kód HTTP 401 a 403 ako neplatné prihlasovacie údaje a akýkoľvek iný ako platné prihlasovacie údaje."
    );
    translations
});

pub static PLURAL_FORMS: &str = "nplurals=3; plural=(n==1) ? 0 : (n>=2 && n<=4) ? 1 : 2;";