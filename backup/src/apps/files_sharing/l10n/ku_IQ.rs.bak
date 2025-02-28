use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("Password", "وشەی تێپەربو");
    m.insert("%s shared the folder %s with you", "%s دابه‌شی کردووه‌ بوخچه‌ی %s له‌گه‌ڵ تۆ");
    m.insert("%s shared the file %s with you", "%s دابه‌شی کردووه‌ په‌ڕگه‌یی %s له‌گه‌ڵ تۆ");
    m.insert("Download", "داگرتن");
    m.insert("Upload", "بارکردن");
    m.insert("No preview available for", "هیچ پێشبینیه‌ك ئاماده‌ نیه بۆ");
    m
});

pub static PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

pub fn get_plural_forms() -> &'static str {
    PLURAL_FORMS
}