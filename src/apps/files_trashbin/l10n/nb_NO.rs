use std::collections::HashMap;
use rust_i18n::i18n;

lazy_static::lazy_static! {
    static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Couldn't delete %s permanently", "Kunne ikke slette %s fullstendig");
        m.insert("Couldn't restore %s", "Kunne ikke gjenopprette %s");
        m.insert("Error", "Feil");
        m.insert("Nothing in here. Your trash bin is empty!", "Ingenting her. Søppelkassen din er tom!");
        m.insert("Name", "Navn");
        m.insert("Restore", "Gjenopprett");
        m.insert("Deleted", "Slettet");
        m.insert("Delete", "Slett");
        m.insert("Deleted Files", "Slettet filer");
        m
    };
}

#[i18n]
pub fn get_translation(key: &str) -> &'static str {
    TRANSLATIONS.get(key).copied().unwrap_or(key)
}

pub fn get_plural_form(n: usize) -> usize {
    if n != 1 { 1 } else { 0 }
}

pub fn get_plural_forms_count() -> usize {
    2
}