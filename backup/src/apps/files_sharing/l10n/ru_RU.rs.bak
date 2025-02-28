use std::collections::HashMap;
use once_cell::sync::Lazy;

// Define the translations hash map
pub static TRANSLATIONS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    let mut translations = HashMap::new();
    translations.insert("Password", "Пароль");
    translations.insert("Download", "Загрузка");
    translations.insert("Upload", "Загрузка");
    translations.insert("Cancel upload", "Отмена загрузки");
    translations
});

// Define the plural forms rule
pub static PLURAL_FORMS: &str = "nplurals=3; plural=(n%10==1 && n%100!=11 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 || n%100>=20) ? 1 : 2);";

// Helper function to get translations
pub fn get_translation(key: &str) -> &str {
    TRANSLATIONS.get(key).copied().unwrap_or(key)
}