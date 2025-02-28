use std::collections::HashMap;
use rust_i18n::Localization;

// Russian language localization definition
pub struct Russian;

impl Localization for Russian {
    fn translations(&self) -> HashMap<String, Vec<String>> {
        let mut translations = HashMap::new();
        translations.insert(
            "_%n file__%n files_".to_string(),
            vec!["%n файл".to_string(), "%n файла".to_string(), "%n файлов".to_string()],
        );
        translations
    }

    fn plural_form(&self) -> String {
        "nplurals=3; plural=(n%10==1 && n%100!=11 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 || n%100>=20) ? 1 : 2);".to_string()
    }

    fn language_code(&self) -> String {
        "ru".to_string()
    }
}