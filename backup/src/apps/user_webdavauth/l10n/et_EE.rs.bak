use lazy_static::lazy_static;
use std::collections::HashMap;
use rust_i18n::t;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("WebDAV Authentication", "WebDAV autentimine");
        m.insert("Address: ", "Aadress:");
        m.insert("The user credentials will be sent to this address. This plugin checks the response and will interpret the HTTP statuscodes 401 and 403 as invalid credentials, and all other responses as valid credentials.", "ownCloud saadab kasutajatunnused sellel aadressil. See vidin kontrollib vastust ning tuvastab ning tõlgendab HTTP olekukoodid 401 ja 403 valedeks andmeteks ning kõik teised vastused korrektseteks andmeteks.");
        m
    };
}

pub fn plural_forms(n: usize) -> usize {
    if n != 1 { 1 } else { 0 }
}