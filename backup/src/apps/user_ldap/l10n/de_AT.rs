use std::collections::HashMap;
use rust_i18n::plural::PluralCategory;

pub struct DeAt;

impl DeAt {
    pub fn translations() -> HashMap<String, Vec<String>> {
        let mut translations = HashMap::new();
        translations.insert(
            "_%s group found_::_%s groups found_".to_string(),
            vec!["".to_string(), "".to_string()]
        );
        translations.insert(
            "_%s user found_::_%s users found_".to_string(),
            vec!["".to_string(), "".to_string()]
        );
        translations
    }

    pub fn plural_form(n: usize) -> PluralCategory {
        if n != 1 {
            PluralCategory::Other
        } else {
            PluralCategory::One
        }
    }
}