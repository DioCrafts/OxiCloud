use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("This share is password-protected", "Toto zdieľanie je chránené heslom");
        map.insert("The password is wrong. Try again.", "Heslo je chybné. Skúste to znova.");
        map.insert("Password", "Heslo");
        map.insert("Sorry, this link doesn't seem to work anymore.", "To je nepríjemné, ale tento odkaz už nie je funkčný.");
        map.insert("Reasons might be:", "Možné dôvody:");
        map.insert("the item was removed", "položka bola presunutá");
        map.insert("the link expired", "linke vypršala platnosť");
        map.insert("sharing is disabled", "zdieľanie je zakázané");
        map.insert("For more info, please ask the person who sent this link.", "Pre viac informácií kontaktujte osobu, ktorá vám poslala tento odkaz.");
        map.insert("%s shared the folder %s with you", "%s zdieľa s vami priečinok %s");
        map.insert("%s shared the file %s with you", "%s zdieľa s vami súbor %s");
        map.insert("Download", "Sťahovanie");
        map.insert("Upload", "Nahrať");
        map.insert("Cancel upload", "Zrušiť nahrávanie");
        map.insert("No preview available for", "Žiaden náhľad k dispozícii pre");
        map.insert("Direct link", "Priama linka");
        map
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=3; plural=(n==1) ? 0 : (n>=2 && n<=4) ? 1 : 2;";
}

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

pub fn get_plural_forms() -> &'static str {
    &PLURAL_FORMS
}