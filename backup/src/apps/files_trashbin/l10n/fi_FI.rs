use std::collections::HashMap;
use once_cell::sync::Lazy;

// Finnish (Finland) translations for files_trashbin
pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("Couldn't delete %s permanently", "Kohdetta %s ei voitu poistaa pysyvästi");
    map.insert("Couldn't restore %s", "Kohteen %s palautus epäonnistui");
    map.insert("Error", "Virhe");
    map.insert("restored", "palautettu");
    map.insert("Nothing in here. Your trash bin is empty!", "Tyhjää täynnä! Roskakorissa ei ole mitään.");
    map.insert("Name", "Nimi");
    map.insert("Restore", "Palauta");
    map.insert("Deleted", "Poistettu");
    map.insert("Delete", "Poista");
    map.insert("Deleted Files", "Poistetut tiedostot");
    map
});

pub const PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";