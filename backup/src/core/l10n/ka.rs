use lazy_static::lazy_static;
use std::collections::HashMap;
use unic_langid::LanguageIdentifier;

lazy_static! {
    pub static ref KA_TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("seconds ago", "წამის წინ");
        m.insert("_%n minute ago_::_%n minutes ago_", "");
        m.insert("_%n hour ago_::_%n hours ago_", "");
        m.insert("today", "დღეს");
        m.insert("yesterday", "გუშინ");
        m.insert("_%n day ago_::_%n days ago_", "");
        m.insert("_%n month ago_::_%n months ago_", "");
        m.insert("_{count} file conflict_::_{count} file conflicts_", "");
        m.insert("Password", "პაროლი");
        m.insert("Personal", "პერსონა");
        m.insert("Users", "მომხმარებლები");
        m.insert("Admin", "ადმინისტრატორი");
        m.insert("Help", "შველა");
        m
    };

    pub static ref KA_PLURAL_FORMS: &'static str = "nplurals=1; plural=0;";

    pub static ref KA_LANGUAGE_ID: LanguageIdentifier = 
        "ka".parse().expect("Failed to parse language identifier");
}

pub fn get_translation(key: &str) -> Option<&'static str> {
    KA_TRANSLATIONS.get(key).copied()
}

pub fn get_plural_forms() -> &'static str {
    &KA_PLURAL_FORMS
}