use lazy_static::lazy_static;
use std::collections::HashMap;
use rust_i18n::locale::PluralForm;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Could not revert: %s", "Nelze vrátit: %s");
        m.insert("Versions", "Verze");
        m.insert("Failed to revert {file} to revision {timestamp}.", "Selhalo vrácení souboru {file} na verzi {timestamp}.");
        m.insert("More versions...", "Více verzí...");
        m.insert("No other versions available", "Žádné další verze nejsou dostupné");
        m.insert("Restore", "Obnovit");
        m
    };

    pub static ref PLURAL_FORMS: PluralForm = PluralForm {
        nplurals: 3,
        plural_fn: |n| {
            if n == 1 {
                0
            } else if n >= 2 && n <= 4 {
                1
            } else {
                2
            }
        },
    };
}