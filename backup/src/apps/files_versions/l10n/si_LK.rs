use rust_i18n::t;

lazy_static::lazy_static! {
    static ref TRANSLATIONS: std::collections::HashMap<&'static str, &'static str> = {
        let mut m = std::collections::HashMap::new();
        m.insert("Versions", "අනුවාද");
        m
    };
}

// nplurals=2; plural=(n != 1);
pub fn get_plural_form(n: usize) -> usize {
    if n != 1 { 1 } else { 0 }
}

// Register translation function
pub fn register_translations() {
    for (key, value) in TRANSLATIONS.iter() {
        rust_i18n::set_translation("si_LK", key, value);
    }
}