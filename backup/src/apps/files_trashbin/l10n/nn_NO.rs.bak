use std::collections::HashMap;
use rust_i18n::i18n;

i18n!("nn_NO");

pub fn get_translations() -> HashMap<&'static str, &'static str> {
    let mut translations = HashMap::new();
    translations.insert("Couldn't delete %s permanently", "Klarte ikkje sletta %s for godt");
    translations.insert("Couldn't restore %s", "Klarte ikkje gjenoppretta %s");
    translations.insert("Error", "Feil");
    translations.insert("restored", "gjenoppretta");
    translations.insert("Nothing in here. Your trash bin is empty!", "Ingenting her. Papirkorga di er tom!");
    translations.insert("Name", "Namn");
    translations.insert("Restore", "Gjenopprett");
    translations.insert("Deleted", "Sletta");
    translations.insert("Delete", "Slett");
    translations.insert("Deleted Files", "Sletta filer");
    translations
}

pub fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}