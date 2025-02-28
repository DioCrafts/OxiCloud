use rust_i18n::t;

// This file provides translations for the User LDAP app
lazy_static::lazy_static! {
    pub static ref TRANSLATIONS: std::collections::HashMap<&'static str, &'static str> = {
        let mut m = std::collections::HashMap::new();
        m.insert("Error", "ਗਲਤੀ");
        m.insert("Password", "ਪਾਸਵਰ");
        m
    };
}

// Plural forms function for Punjabi language
pub fn get_plural_form(n: usize) -> usize {
    if n != 1 { 1 } else { 0 }
}

// Returns translated string for plural forms
pub fn translate_plural(singular: &str, plural: &str, n: usize) -> String {
    let form = get_plural_form(n);
    match (singular, plural, form) {
        ("_%s group found_", "_%s groups found_", 0) => String::new(),
        ("_%s group found_", "_%s groups found_", 1) => String::new(),
        ("_%s user found_", "_%s users found_", 0) => String::new(),
        ("_%s user found_", "_%s users found_", 1) => String::new(),
        _ => if form == 0 { singular.to_string() } else { plural.to_string() }
    }
}