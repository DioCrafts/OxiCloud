use std::collections::HashMap;
use rust_i18n::i18n;

/// Russian (Russia) localization
pub struct RuRU;

impl RuRU {
    pub fn translations() -> HashMap<&'static str, &'static str> {
        let mut translations = HashMap::new();
        translations.insert("Error", "Ошибка");
        translations.insert("More", "Подробнее");
        translations.insert("Password", "Пароль");
        translations.insert("Username", "Имя пользователя");
        translations
    }

    pub fn plural_forms() -> &'static str {
        "nplurals=3; plural=(n%10==1 && n%100!=11 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 || n%100>=20) ? 1 : 2);"
    }
}