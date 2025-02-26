use std::collections::HashMap;
use rust_gettext::plural::PluralForms;

pub fn get_translations() -> HashMap<&'static str, &'static str> {
    let mut translations = HashMap::new();
    translations.insert("Saving...", "جاري الحفظ...");
    translations.insert("Encryption", "التشفير");
    translations
}

pub fn get_plural_forms() -> PluralForms {
    PluralForms::from_rule_string("nplurals=6; plural=n==0 ? 0 : n==1 ? 1 : n==2 ? 2 : n%100>=3 && n%100<=10 ? 3 : n%100>=11 && n%100<=99 ? 4 : 5")
        .expect("Invalid plural form expression")
}