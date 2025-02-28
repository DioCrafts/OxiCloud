use rust_gettext::prelude::*;
use std::collections::HashMap;

pub fn get_plural_forms() -> &'static str {
    "nplurals=1; plural=0;"
}

pub fn get_translations() -> HashMap<&'static str, Vec<&'static str>> {
    let mut translations = HashMap::new();
    
    translations.insert("_%n folder_::_%n folders_", vec![""]);
    translations.insert("_%n file_::_%n files_", vec![""]);
    translations.insert("_Uploading %n file_::_Uploading %n files_", vec![""]);
    
    translations
}

pub fn initialize_i18n() -> Catalog {
    let mut catalog = Catalog::new();
    catalog.set_plural_forms(get_plural_forms());
    
    for (key, values) in get_translations() {
        let parts: Vec<&str> = key.split("::").collect();
        if parts.len() == 2 {
            catalog.add_plural(parts[0], parts[1], &values);
        }
    }
    
    catalog
}