use std::collections::HashMap;
use once_cell::sync::Lazy;

/// Static translations for Bengali (Bangladesh)
pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("Saving...", "সংরক্ষণ করা হচ্ছে..");
    map.insert("Encryption", "সংকেতায়ন");
    map
});

/// Plural forms rule for Bengali (Bangladesh)
pub const PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translations_exist() {
        assert!(TRANSLATIONS.contains_key("Saving..."));
        assert!(TRANSLATIONS.contains_key("Encryption"));
    }

    #[test]
    fn test_translation_values() {
        assert_eq!(TRANSLATIONS.get("Saving..."), Some(&"সংরক্ষণ করা হচ্ছে.."));
        assert_eq!(TRANSLATIONS.get("Encryption"), Some(&"সংকেতায়ন"));
    }
}