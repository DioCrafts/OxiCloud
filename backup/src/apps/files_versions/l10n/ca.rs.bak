use lazy_static::lazy_static;
use std::collections::HashMap;
use rust_gettext::TranslationPluralForms;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("Could not revert: %s", "No s'ha pogut revertir: %s");
        map.insert("Versions", "Versions");
        map.insert("Failed to revert {file} to revision {timestamp}.", "Ha fallat en retornar {file} a la revisió {timestamp}");
        map.insert("More versions...", "Més versions...");
        map.insert("No other versions available", "No hi ha altres versions disponibles");
        map.insert("Restore", "Recupera");
        map
    };

    pub static ref PLURAL_FORMS: TranslationPluralForms = TranslationPluralForms {
        nplurals: 2,
        plural_rule: Box::new(|n| (n != 1) as usize),
    };
}