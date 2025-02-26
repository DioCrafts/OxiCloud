use std::collections::HashMap;
use unic_langid::LanguageIdentifier;
use fluent_syntax::ast::Pattern;

/// Locale data for Tamazight (tzm)
pub struct TzmaightLocale;

impl TzmaightLocale {
    pub fn get_translations() -> HashMap<String, Vec<String>> {
        let mut translations = HashMap::new();
        
        translations.insert(
            "_%n minute ago_::_%n minutes ago_".to_string(),
            vec!["".to_string(), "".to_string()]
        );
        translations.insert(
            "_%n hour ago_::_%n hours ago_".to_string(),
            vec!["".to_string(), "".to_string()]
        );
        translations.insert(
            "_%n day go_::_%n days ago_".to_string(),
            vec!["".to_string(), "".to_string()]
        );
        translations.insert(
            "_%n month ago_::_%n months ago_".to_string(),
            vec!["".to_string(), "".to_string()]
        );
        
        translations
    }

    pub fn get_plural_form() -> String {
        "nplurals=2; plural=(n == 0 || n == 1 || (n > 10 && n < 100) ? 0 : 1;".to_string()
    }

    pub fn get_plural_index(n: usize) -> usize {
        if n == 0 || n == 1 || (n > 10 && n < 100) {
            0
        } else {
            1
        }
    }

    pub fn get_language_id() -> LanguageIdentifier {
        "tzm".parse().unwrap()
    }
}