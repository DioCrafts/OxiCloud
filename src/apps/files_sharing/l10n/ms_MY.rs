use std::collections::HashMap;
use rust_gettext::{Catalog, CatalogBuilder};

pub fn get_translation_catalog() -> Catalog {
    let mut translations = HashMap::new();
    translations.insert("Password".to_string(), "Kata laluan".to_string());
    translations.insert("Download".to_string(), "Muat turun".to_string());
    translations.insert("Upload".to_string(), "Muat naik".to_string());
    translations.insert("Cancel upload".to_string(), "Batal muat naik".to_string());

    CatalogBuilder::new()
        .set_translations(translations)
        .set_plural_forms("nplurals=1; plural=0;".to_string())
        .build()
}