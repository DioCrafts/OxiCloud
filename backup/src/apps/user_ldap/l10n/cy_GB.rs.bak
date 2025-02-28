use std::collections::HashMap;
use rust_i18n::i18n;

pub fn get_translations() -> HashMap<String, String> {
    let mut translations = HashMap::new();
    translations.insert("Deletion failed".to_string(), "Methwyd dileu".to_string());
    translations.insert("Error".to_string(), "Gwall".to_string());
    translations.insert("Save".to_string(), "Cadw".to_string());
    translations.insert("Help".to_string(), "Cymorth".to_string());
    translations.insert("Password".to_string(), "Cyfrinair".to_string());
    translations
}

pub fn get_plural_forms() -> &'static str {
    "nplurals=4; plural=(n==1) ? 0 : (n==2) ? 1 : (n != 8 && n != 11) ? 2 : 3;"
}

pub fn get_plural_translations() -> HashMap<String, Vec<String>> {
    let mut plural_translations = HashMap::new();
    plural_translations.insert(
        "_%s group found_::_%s groups found_".to_string(),
        vec!["".to_string(), "".to_string(), "".to_string(), "".to_string()],
    );
    plural_translations.insert(
        "_%s user found_::_%s users found_".to_string(),
        vec!["".to_string(), "".to_string(), "".to_string(), "".to_string()],
    );
    plural_translations
}

i18n! {
    fallback: "en",
    locales: {
        "cy_GB": {
            path: "./locales/cy_GB",
            format: "json"
        }
    }
}