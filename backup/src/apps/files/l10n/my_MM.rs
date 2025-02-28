use rust_i18n::translation_args;
use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static MY_MM: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut translations = HashMap::new();
    translations.insert("Files", "ဖိုင်များ");
    translations.insert("_%n folder_::_%n folders_", "");
    translations.insert("_%n file_::_%n files_", "");
    translations.insert("_Uploading %n file_::_Uploading %n files_", "");
    translations.insert("Download", "ဒေါင်းလုတ်");
    translations
});

pub const PLURAL_FORMS: &str = "nplurals=1; plural=0;";

pub fn translate(key: &str) -> &'static str {
    MY_MM.get(key).copied().unwrap_or(key)
}

pub fn translate_plural(key: &str, count: usize) -> String {
    // Since this language has only one plural form
    let translated = translate(key);
    translation_args!({
        "n" => count
    }).translate(translated)
}