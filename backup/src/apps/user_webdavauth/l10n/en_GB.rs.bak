use std::collections::HashMap;
use once_cell::sync::Lazy;

// WebDAV Authentication localization for en_GB
pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("WebDAV Authentication", "WebDAV Authentication");
    m.insert("Address: ", "Address: ");
    m.insert(
        "The user credentials will be sent to this address. This plugin checks the response and will interpret the HTTP statuscodes 401 and 403 as invalid credentials, and all other responses as valid credentials.",
        "The user credentials will be sent to this address. This plugin checks the response and will interpret the HTTP statuscodes 401 and 403 as invalid credentials, and all other responses as valid credentials."
    );
    m
});

pub fn get_plural_form(n: usize) -> usize {
    if n != 1 { 1 } else { 0 }
}

pub const PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";