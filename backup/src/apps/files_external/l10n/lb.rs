use std::collections::HashMap;
use rust_gettext::Catalog;

pub fn get_translation_catalog() -> Catalog {
    let mut translations = HashMap::new();
    translations.insert("Folder name".to_string(), "Dossiers Numm:".to_string());
    translations.insert("Groups".to_string(), "Gruppen".to_string());
    translations.insert("Users".to_string(), "Benotzer".to_string());
    translations.insert("Delete".to_string(), "Läschen".to_string());

    Catalog {
        translations,
        plural_form: "nplurals=2; plural=(n != 1);".to_string(),
        ..Default::default()
    }
}