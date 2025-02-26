use lazy_static::lazy_static;
use std::collections::HashMap;
use rust_i18n::LocalizedString;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Password", "Пароль");
        m.insert("%s shared the folder %s with you", "%s опублікував каталог %s для Вас");
        m.insert("%s shared the file %s with you", "%s опублікував файл %s для Вас");
        m.insert("Download", "Завантажити");
        m.insert("Upload", "Вивантажити");
        m.insert("Cancel upload", "Перервати завантаження");
        m.insert("No preview available for", "Попередній перегляд недоступний для");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=3; plural=(n%10==1 && n%100!=11 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 || n%100>=20) ? 1 : 2);";
}

pub fn translate(key: &str) -> &'static str {
    TRANSLATIONS.get(key).unwrap_or(key)
}

pub fn translate_plural(key: &str, count: i64) -> &'static str {
    // This would need actual implementation based on plural rules
    // Simplified version that just returns the singular form
    translate(key)
}