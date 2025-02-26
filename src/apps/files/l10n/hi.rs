use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Translations for Hindi (hi)
pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("Share", "साझा करें");
    m.insert("Error", "त्रुटि");
    m.insert("Upload", "अपलोड ");
    m.insert("Save", "सहेजें");
    m
});

pub static PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";

/// Returns the plural form for the given count
pub fn get_plural_form(n: usize) -> usize {
    if n != 1 { 1 } else { 0 }
}

/// Plural translations for Hindi (hi)
pub static PLURAL_TRANSLATIONS: Lazy<HashMap<&'static str, Vec<&'static str>>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("_%n folder_::_%n folders_", vec!["", ""]);
    m.insert("_%n file_::_%n files_", vec!["", ""]);
    m.insert("_Uploading %n file_::_Uploading %n files_", vec!["", ""]);
    m
});