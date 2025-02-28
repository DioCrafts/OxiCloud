use std::collections::HashMap;
use rust_gettext::Catalog;

fn get_translations() -> HashMap<&'static str, &'static str> {
    let mut translations = HashMap::new();
    translations.insert("Versions", "Versjoner");
    translations.insert("Restore", "Gjenopprett");
    translations
}

fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}

pub fn build_catalog() -> Catalog {
    let translations = get_translations();
    let plural_forms = get_plural_forms();
    
    let mut catalog = Catalog::new();
    catalog.set_plural_forms(plural_forms);
    
    for (key, value) in translations {
        catalog.add_translation(key, value);
    }
    
    catalog
}