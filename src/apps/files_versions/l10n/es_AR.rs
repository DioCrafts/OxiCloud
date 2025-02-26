use lazy_static::lazy_static;
use std::collections::HashMap;
use rust_gettext::Catalog;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Could not revert: %s", "No se pudo revertir: %s ");
        m.insert("Versions", "Versiones");
        m.insert("Failed to revert {file} to revision {timestamp}.", "Falló al revertir {file} a la revisión {timestamp}.");
        m.insert("More versions...", "Más versiones...");
        m.insert("No other versions available", "No hay más versiones disponibles");
        m.insert("Restore", "Recuperar");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}

pub fn get_translation(key: &str) -> &'static str {
    TRANSLATIONS.get(key).copied().unwrap_or(key)
}

pub fn get_plural_forms() -> &'static str {
    &PLURAL_FORMS
}

pub fn init_catalog() -> Catalog {
    let mut catalog = Catalog::new();
    catalog.set_plural_forms_rule(&PLURAL_FORMS);
    
    for (key, value) in TRANSLATIONS.iter() {
        catalog.add_message(key, value);
    }
    
    catalog
}