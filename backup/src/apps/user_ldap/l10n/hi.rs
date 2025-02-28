use lazy_static::lazy_static;
use std::collections::HashMap;
use rust_gettext::ngettext;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Error", "त्रुटि");
        m.insert("Save", "सहेजें");
        m.insert("Help", "सहयोग");
        m.insert("Password", "पासवर्ड");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}

pub fn translate_plural(singular: &str, plural: &str, count: u32) -> String {
    match singular {
        "%s group found" => ngettext(count, "%s group found", "%s groups found", PLURAL_FORMS.as_ref()),
        "%s user found" => ngettext(count, "%s user found", "%s users found", PLURAL_FORMS.as_ref()),
        _ => if count == 1 { singular.to_string() } else { plural.to_string() }
    }
}

pub fn translate(key: &str) -> &str {
    TRANSLATIONS.get(key).unwrap_or(&key)
}