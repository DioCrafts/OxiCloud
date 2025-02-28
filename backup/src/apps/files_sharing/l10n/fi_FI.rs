use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("This share is password-protected", "Tämä jako on suojattu salasanalla");
        m.insert("The password is wrong. Try again.", "Väärä salasana. Yritä uudelleen.");
        m.insert("Password", "Salasana");
        m.insert("Sorry, this link doesn't seem to work anymore.", "Valitettavasti linkki ei vaikuta enää toimivan.");
        m.insert("Reasons might be:", "Mahdollisia syitä:");
        m.insert("the item was removed", "kohde poistettiin");
        m.insert("the link expired", "linkki vanheni");
        m.insert("sharing is disabled", "jakaminen on poistettu käytöstä");
        m.insert("For more info, please ask the person who sent this link.", "Kysy lisätietoja henkilöltä, jolta sait linkin.");
        m.insert("%s shared the folder %s with you", "%s jakoi kansion %s kanssasi");
        m.insert("%s shared the file %s with you", "%s jakoi tiedoston %s kanssasi");
        m.insert("Download", "Lataa");
        m.insert("Upload", "Lähetä");
        m.insert("Cancel upload", "Peru lähetys");
        m.insert("No preview available for", "Ei esikatselua kohteelle");
        m.insert("Direct link", "Suora linkki");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}