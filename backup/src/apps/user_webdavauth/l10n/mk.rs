use std::collections::HashMap;
use rust_i18n::i18n;

i18n!("mk");

// Create the translation HashMap as a function to allow it to be used in other modules
pub fn get_translations() -> HashMap<&'static str, &'static str> {
    let mut translations = HashMap::new();
    translations.insert("Address: ", "Адреса:");
    translations
}

// Define the plural form rule for Macedonian
pub fn get_plural_form(n: i64) -> usize {
    if n % 10 == 1 && n % 100 != 11 {
        0
    } else {
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translations() {
        let translations = get_translations();
        assert_eq!(translations.get("Address: "), Some(&"Адреса:"));
    }

    #[test]
    fn test_plural_forms() {
        assert_eq!(get_plural_form(1), 0);
        assert_eq!(get_plural_form(11), 1);
        assert_eq!(get_plural_form(21), 0);
        assert_eq!(get_plural_form(111), 1);
    }
}