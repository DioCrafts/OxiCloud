use std::collections::HashMap;
use rust_i18n::i18n;

/// Telugu translation mappings
pub fn get_te_translations() -> HashMap<&'static str, &'static str> {
    let mut translations = HashMap::new();
    
    translations.insert("Error", "పొరపాటు");
    translations.insert("Delete", "తొలగించు");
    translations.insert("More", "మరిన్ని");
    translations.insert("Password", "సంకేతపదం");
    translations.insert("New password", "కొత్త సంకేతపదం");
    translations.insert("Email", "ఈమెయిలు");
    translations.insert("Your email address", "మీ ఈమెయిలు చిరునామా");
    translations.insert("Language", "భాష");
    translations.insert("Username", "వాడుకరి పేరు");
    
    translations
}

/// Telugu plural forms rule
pub fn get_te_plural_forms() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translations_not_empty() {
        let translations = get_te_translations();
        assert!(!translations.is_empty());
    }

    #[test]
    fn test_plural_forms_defined() {
        let plural_forms = get_te_plural_forms();
        assert!(!plural_forms.is_empty());
    }
}