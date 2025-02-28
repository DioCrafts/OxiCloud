use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static TRANSLATIONS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("WebDAV Authentication", "WebDAV Autentisering");
    m.insert("Address: ", "Adress: ");
    m.insert(
        "The user credentials will be sent to this address. This plugin checks the response and will interpret the HTTP statuscodes 401 and 403 as invalid credentials, and all other responses as valid credentials.",
        "ownCloud kommer skicka användaruppgifterna till denna URL. Denna plugin kontrollerar svaret och tolkar HTTP-statuskoderna 401 och 403 som felaktiga uppgifter, och alla andra svar som giltiga uppgifter."
    );
    m
});

pub static PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";