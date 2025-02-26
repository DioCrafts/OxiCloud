use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Translations for Northern Low German language
pub static TRANSLATIONS: Lazy<HashMap<&'static str, Vec<&'static str>>> = Lazy::new(|| {
    let mut translations = HashMap::new();
    
    translations.insert("_%n minute ago_::_%n minutes ago_", vec!["", ""]);
    translations.insert("_%n hour ago_::_%n hours ago_", vec!["", ""]);
    translations.insert("_%n day go_::_%n days ago_", vec!["", ""]);
    translations.insert("_%n month ago_::_%n months ago_", vec!["", ""]);
    
    translations
});

pub static PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";

pub fn get_plural_index(n: usize) -> usize {
    if n != 1 { 1 } else { 0 }
}