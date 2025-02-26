use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Password", "Geslo");
        m.insert(
            "%s shared the folder %s with you",
            "Oseba %s je določila mapo %s za souporabo",
        );
        m.insert(
            "%s shared the file %s with you",
            "Oseba %s je določila datoteko %s za souporabo",
        );
        m.insert("Download", "Prejmi");
        m.insert("Upload", "Pošlji");
        m.insert("Cancel upload", "Prekliči pošiljanje");
        m.insert("No preview available for", "Predogled ni na voljo za");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=4; plural=(n%100==1 ? 0 : n%100==2 ? 1 : n%100==3 || n%100==4 ? 2 : 3);";
}