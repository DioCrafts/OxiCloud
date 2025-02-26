use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("WebDAV Authentication", "Pengesahan ");
        m.insert("Address: ", "Alamat:");
        m.insert(
            "The user credentials will be sent to this address. This plugin checks the response and will interpret the HTTP statuscodes 401 and 403 as invalid credentials, and all other responses as valid credentials.",
            "Butiran pengguna akan dihantar ke alamat ini. Plugin ini memeriksa maklum balas dan akan mentafsir kod status HTTP 401 dan 403 sebagai butiran tidak sah, dan semua maklum balas lain sebagai butiran yang sah."
        );
        m
    };
    
    pub static ref PLURAL_FORMS: &'static str = "nplurals=1; plural=0;";
}