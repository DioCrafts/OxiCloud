use lazy_static::lazy_static;
use rust_i18n::domain::TranslationDomain;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("Deletion failed", "Бришењето е неуспешно");
        map.insert("Keep settings?", "Да ги сочувам нагодувањата?");
        map.insert("Cannot add server configuration", "Не можам да ја додадам конфигурацијата на серверот");
        map.insert("Error", "Грешка");
        map.insert("Connection test succeeded", "Тестот за поврзување е успешен");
        map.insert("Connection test failed", "Тестот за поврзување не е успешен");
        map.insert("Confirm Deletion", "Потврдете го бришењето");
        map.insert("Save", "Сними");
        map.insert("Help", "Помош");
        map.insert("Host", "Домаќин");
        map.insert("You can omit the protocol, except you require SSL. Then start with ldaps://", 
                   "Може да го скокнете протколот освен ако не ви треба SSL. Тогаш ставете ldaps://");
        map.insert("Port", "Порта");
        map.insert("Password", "Лозинка");
        map.insert("Back", "Назад");
        map.insert("Continue", "Продолжи");
        map
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n % 10 == 1 && n % 100 != 11) ? 0 : 1;";

    pub static ref PLURAL_TRANSLATIONS: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("_%s group found_::_%s groups found_", vec!["", ""]);
        map.insert("_%s user found_::_%s users found_", vec!["", ""]);
        map
    };
}

pub struct MacedonianTranslationDomain;

impl TranslationDomain for MacedonianTranslationDomain {
    fn get_translation(&self, key: &str) -> Option<&str> {
        TRANSLATIONS.get(key).copied()
    }

    fn get_plural_translation(&self, key: &str, count: usize) -> Option<&str> {
        // Calculate plural form index based on the PLURAL_FORMS rule
        let plural_index = if count % 10 == 1 && count % 100 != 11 { 0 } else { 1 };
        
        PLURAL_TRANSLATIONS.get(key).and_then(|forms| forms.get(plural_index).copied())
    }
}

pub fn init() -> MacedonianTranslationDomain {
    MacedonianTranslationDomain
}