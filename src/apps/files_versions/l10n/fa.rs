use once_cell::sync::Lazy;
use phf::phf_map;
use std::collections::HashMap;

pub static TRANSLATIONS: Lazy<phf::Map<&'static str, &'static str>> = Lazy::new(|| {
    phf_map! {
        "Could not revert: %s" => "بازگردانی امکان ناپذیر است: %s",
        "Versions" => "نسخه ها",
        "Restore" => "بازیابی",
    }
});

pub static PLURAL_FORMS: &str = "nplurals=1; plural=0;";

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

pub fn format_translation(key: &str, params: &[&str]) -> String {
    if let Some(template) = get_translation(key) {
        let mut result = template.to_string();
        for (i, param) in params.iter().enumerate() {
            result = result.replace(&format!("%{}", i + 1), param);
            result = result.replace("%s", param);
        }
        result
    } else {
        key.to_string()
    }
}