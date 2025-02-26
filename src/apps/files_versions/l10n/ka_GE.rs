use once_cell::sync::Lazy;
use std::collections::HashMap;
use gettext_plural_forms::PluralForms;

pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut translations = HashMap::new();
    translations.insert("Could not revert: %s", "ვერ მოხერხდა უკან დაბრუნება: %s");
    translations.insert("Versions", "ვერსიები");
    translations.insert("Restore", "აღდგენა");
    translations
});

pub static PLURAL_FORMS: Lazy<PluralForms> = Lazy::new(|| {
    PluralForms::new("nplurals=1; plural=0;").expect("Invalid plural form expression")
});