use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("Password", "პაროლი");
    m.insert("%s shared the folder %s with you", "%s–მა გაგიზიარათ ფოლდერი %s");
    m.insert("%s shared the file %s with you", "%s–მა გაგიზიარათ ფაილი %s");
    m.insert("Download", "ჩამოტვირთვა");
    m.insert("Upload", "ატვირთვა");
    m.insert("Cancel upload", "ატვირთვის გაუქმება");
    m.insert("No preview available for", "წინასწარი დათვალიერება შეუძლებელია");
    m
});

pub static PLURAL_FORMS: &str = "nplurals=1; plural=0;";