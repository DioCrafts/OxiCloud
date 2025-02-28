use std::collections::HashMap;
use rust_i18n::Locale;

/// Russian (Russia) translation
pub struct RuRu;

impl Locale for RuRu {
    fn locale_code(&self) -> &'static str {
        "ru_RU"
    }

    fn translations(&self) -> HashMap<&'static str, &'static str> {
        let mut translations = HashMap::new();
        translations.insert("Settings", "Настройки");
        translations.insert("Yes", "Да");
        translations.insert("No", "Нет");
        translations.insert("Cancel", "Отмена");
        translations.insert("Share", "Сделать общим");
        translations.insert("Error", "Ошибка");
        translations.insert("Password", "Пароль");
        translations.insert("Warning", "Предупреждение");
        translations.insert("Username", "Имя пользователя");
        translations.insert("Help", "Помощь");
        translations
    }

    fn plural_forms(&self) -> &'static str {
        "nplurals=3; plural=(n%10==1 && n%100!=11 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 || n%100>=20) ? 1 : 2);"
    }

    fn get_plural_translation(&self, key: &str, count: usize) -> Option<&'static str> {
        match key {
            "_%n minute ago_::_%n minutes ago_" => Some(match self.plural_index(count) {
                0 => "",
                1 => "",
                _ => "",
            }),
            "_%n hour ago_::_%n hours ago_" => Some(match self.plural_index(count) {
                0 => "",
                1 => "",
                _ => "",
            }),
            "_%n day ago_::_%n days ago_" => Some(match self.plural_index(count) {
                0 => "",
                1 => "",
                _ => "",
            }),
            "_%n month ago_::_%n months ago_" => Some(match self.plural_index(count) {
                0 => "",
                1 => "",
                _ => "",
            }),
            "_{count} file conflict_::_{count} file conflicts_" => Some(match self.plural_index(count) {
                0 => "",
                1 => "",
                _ => "",
            }),
            _ => None,
        }
    }

    fn plural_index(&self, n: usize) -> usize {
        let n = n as u64;
        if n % 10 == 1 && n % 100 != 11 {
            0
        } else if n % 10 >= 2 && n % 10 <= 4 && (n % 100 < 10 || n % 100 >= 20) {
            1
        } else {
            2
        }
    }
}