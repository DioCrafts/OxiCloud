use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static TRANSLATIONS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("WebDAV Authentication", "WebDAV hitelesítés");
    m.insert("Address: ", "Címek:");
    m.insert(
        "The user credentials will be sent to this address. This plugin checks the response and will interpret the HTTP statuscodes 401 and 403 as invalid credentials, and all other responses as valid credentials.",
        "A felhasználói hitelesítő adatai el lesznek küldve erre a címre. Ez a bővítőmodul leellenőrzi a választ és ha a HTTP hibakód nem 401 vagy 403 azaz érvénytelen a hitelesítő adat, akkor minden más válasz érvényes lesz."
    );
    m
});

pub static PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";