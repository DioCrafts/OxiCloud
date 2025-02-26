use rust_i18n::locale::plurals::{SerbianLike, PluralCategory};
use std::collections::HashMap;

// Serbian translations
pub fn get_translations() -> HashMap<&'static str, &'static str> {
    let mut translations = HashMap::new();
    translations.insert("Groups", "Групе");
    translations.insert("Users", "Корисници");
    translations.insert("Delete", "Обриши");
    translations
}

// Serbian plural forms
pub fn get_plural_form(n: i64) -> PluralCategory {
    SerbianLike::new().get_category(n)
}