use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert(
            "WebDAV Authentication",
            "Uwierzytelnienie WebDAV"
        );
        m.insert(
            "Address: ",
            "Adres:"
        );
        m.insert(
            "The user credentials will be sent to this address. This plugin checks the response and will interpret the HTTP statuscodes 401 and 403 as invalid credentials, and all other responses as valid credentials.",
            "Dane uwierzytelniające użytkownika zostaną wysłane na ten adres. Wtyczka sprawdza odpowiedź i będzie interpretował status  HTTP 401 i 403 jako nieprawidłowe dane uwierzytelniające i wszystkie inne odpowiedzi jako prawidłowe uwierzytelnienie."
        );
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=3; plural=(n==1 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 || n%100>=20) ? 1 : 2);";
}

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

pub fn get_plural_forms() -> &'static str {
    &PLURAL_FORMS
}