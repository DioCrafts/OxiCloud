use lazy_static::lazy_static;
use std::collections::HashMap;
use rust_i18n::plurals::PluralRules;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Error", "Ошибка");
        m.insert("Select groups", "Выбрать группы");
        m.insert("_%s group found_::_%s groups found_", "");
        m.insert("_%s user found_::_%s users found_", "");
        m.insert("Save", "Сохранить");
        m.insert("Help", "Помощь");
        m.insert("Password", "Пароль");
        m.insert("Back", "Назад");
        m
    };

    pub static ref PLURAL_RULES: PluralRules = PluralRules::new(
        3,
        Box::new(|n| {
            let n100 = n % 100;
            let n10 = n % 10;
            
            if n10 == 1 && n100 != 11 {
                0
            } else if n10 >= 2 && n10 <= 4 && (n100 < 10 || n100 >= 20) {
                1
            } else {
                2
            }
        })
    );
}

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

pub fn get_plural_form(n: usize) -> usize {
    PLURAL_RULES.get_plural_form(n)
}