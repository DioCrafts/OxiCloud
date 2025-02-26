use once_cell::sync::Lazy;
use std::collections::HashMap;

// Define translations as a static lazy-initialized HashMap
pub static TRANSLATIONS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("WebDAV Authentication", "Ověření WebDAV");
    map.insert("Address: ", "Adresa:");
    map.insert("The user credentials will be sent to this address. This plugin checks the response and will interpret the HTTP statuscodes 401 and 403 as invalid credentials, and all other responses as valid credentials.", 
               "Uživatelské přihlašovací údaje budou odeslány na tuto adresu. Tento plugin zkontroluje odpověď serveru a interpretuje návratový kód HTTP 401 a 403 jako neplatné přihlašovací údaje a jakýkoli jiný jako platné přihlašovací údaje.");
    map
});

// Define plural forms string
pub const PLURAL_FORMS: &str = "nplurals=3; plural=(n==1) ? 0 : (n>=2 && n<=4) ? 1 : 2;";

// Function to get translation
pub fn gettext(text: &str) -> &str {
    TRANSLATIONS.get(text).copied().unwrap_or(text)
}

// Function to get plural form
pub fn ngettext(singular: &str, plural: &str, count: usize) -> &str {
    let form = match count {
        1 => 0,
        2..=4 => 1,
        _ => 2,
    };
    
    if form == 0 {
        gettext(singular)
    } else {
        gettext(plural)
    }
}