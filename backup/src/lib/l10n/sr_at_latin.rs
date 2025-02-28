use std::collections::HashMap;
use rust_i18n::i18n;

pub fn get_translations() -> HashMap<&'static str, &'static str> {
    let mut translations = HashMap::new();
    translations.insert("Help", "Pomoć");
    translations.insert("Personal", "Lično");
    translations.insert("Settings", "Podešavanja");
    translations.insert("Users", "Korisnici");
    translations.insert("Admin", "Adninistracija");
    translations.insert("Authentication error", "Greška pri autentifikaciji");
    translations.insert("Files", "Fajlovi");
    translations.insert("Text", "Tekst");
    translations.insert("seconds ago", "Pre par sekundi");
    translations.insert("today", "Danas");
    translations.insert("yesterday", "juče");
    translations.insert("last month", "prošlog meseca");
    translations.insert("last year", "prošle godine");
    translations.insert("years ago", "pre nekoliko godina");
    translations
}

pub fn get_plural_forms() -> &'static str {
    "nplurals=3; plural=(n%10==1 && n%100!=11 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 || n%100>=20) ? 1 : 2);"
}

pub fn get_plural_translations() -> HashMap<&'static str, Vec<&'static str>> {
    let mut plural_translations = HashMap::new();
    plural_translations.insert("_%n minute ago_::_%n minutes ago_", vec!["", "", ""]);
    plural_translations.insert("_%n hour ago_::_%n hours ago_", vec!["", "", ""]);
    plural_translations.insert("_%n day go_::_%n days ago_", vec!["", "", ""]);
    plural_translations.insert("_%n month ago_::_%n months ago_", vec!["", "", ""]);
    plural_translations
}

// Register the translations with the i18n system
pub fn register_translations() {
    let translations = get_translations();
    let plural_translations = get_plural_translations();
    let plural_forms = get_plural_forms();
    
    i18n!("sr@latin", translations, plural_translations, plural_forms);
}