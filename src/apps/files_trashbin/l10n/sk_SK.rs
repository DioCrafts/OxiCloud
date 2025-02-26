use once_cell::sync::Lazy;
use std::collections::HashMap;
use rust_i18n::translation_function;

// Slovak translations (sk_SK)
pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("Couldn't delete %s permanently", "Nemožno zmazať %s navždy");
    m.insert("Couldn't restore %s", "Nemožno obnoviť %s");
    m.insert("Error", "Chyba");
    m.insert("restored", "obnovené");
    m.insert("Nothing in here. Your trash bin is empty!", "Žiadny obsah. Kôš je prázdny!");
    m.insert("Name", "Názov");
    m.insert("Restore", "Obnoviť");
    m.insert("Deleted", "Zmazané");
    m.insert("Delete", "Zmazať");
    m.insert("Deleted Files", "Zmazané súbory");
    m
});

// Define plural forms for Slovak language
pub const PLURAL_FORMS: &str = "nplurals=3; plural=(n==1) ? 0 : (n>=2 && n<=4) ? 1 : 2;";

// Translation function implementation
translation_function!(sk_SK, PLURAL_FORMS);