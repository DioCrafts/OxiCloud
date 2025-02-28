use std::collections::HashMap;
use rust_i18n::locale::LocalePlural;

/// Serbian (sr) translations
pub fn get_translations() -> HashMap<&'static str, &'static str> {
    let mut translations = HashMap::new();
    translations.insert("Error", "Грешка");
    translations.insert("Nothing in here. Your trash bin is empty!", "Овде нема ништа. Корпа за отпатке је празна.");
    translations.insert("Name", "Име");
    translations.insert("Restore", "Врати");
    translations.insert("Deleted", "Обрисано");
    translations.insert("Delete", "Обриши");
    translations
}

/// Serbian plural forms
pub fn get_plural_forms() -> LocalePlural {
    LocalePlural::new("nplurals=3; plural=(n%10==1 && n%100!=11 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 || n%100>=20) ? 1 : 2);")
}