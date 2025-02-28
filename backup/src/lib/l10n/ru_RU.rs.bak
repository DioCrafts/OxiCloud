use std::collections::HashMap;
use rust_i18n::translation_set::PluralForms;

pub struct RuRu;

impl RuRu {
    pub fn new() -> Self {
        RuRu
    }

    pub fn translations(&self) -> HashMap<String, String> {
        let mut translations = HashMap::new();
        translations.insert("Help".to_string(), "Помощь".to_string());
        translations.insert("Settings".to_string(), "Настройки".to_string());
        translations.insert("Files".to_string(), "Файлы".to_string());
        translations
    }

    pub fn plural_translations(&self) -> HashMap<String, Vec<String>> {
        let mut plural_translations = HashMap::new();
        plural_translations.insert(
            "_%n minute ago_::_%n minutes ago_".to_string(),
            vec!["".to_string(), "".to_string(), "".to_string()]
        );
        plural_translations.insert(
            "_%n hour ago_::_%n hours ago_".to_string(),
            vec!["".to_string(), "".to_string(), "".to_string()]
        );
        plural_translations.insert(
            "_%n day go_::_%n days ago_".to_string(),
            vec!["".to_string(), "".to_string(), "".to_string()]
        );
        plural_translations.insert(
            "_%n month ago_::_%n months ago_".to_string(),
            vec!["".to_string(), "".to_string(), "".to_string()]
        );
        plural_translations
    }

    pub fn plural_forms(&self) -> PluralForms {
        PluralForms {
            nplurals: 3,
            plural_function: Box::new(|n| {
                if n % 10 == 1 && n % 100 != 11 {
                    0
                } else if n % 10 >= 2 && n % 10 <= 4 && (n % 100 < 10 || n % 100 >= 20) {
                    1
                } else {
                    2
                }
            }),
        }
    }
}