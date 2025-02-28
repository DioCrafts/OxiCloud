use phf::phf_map;
use std::collections::HashMap;

// Translations for ne (Nepali)
lazy_static::lazy_static! {
    static ref TRANSLATIONS: phf::Map<&'static str, Vec<&'static str>> = phf_map! {
        "_%s group found_::_%s groups found_" => vec!["", ""],
        "_%s user found_::_%s users found_" => vec!["", ""],
    };
}

pub fn get_plural_form(n: i64) -> usize {
    if n != 1 { 1 } else { 0 }
}

pub fn get_translation(key: &str) -> Option<&Vec<&'static str>> {
    TRANSLATIONS.get(key)
}

pub fn get_plural_translation(key: &str, count: i64) -> Option<&'static str> {
    get_translation(key).map(|forms| forms[get_plural_form(count)])
}