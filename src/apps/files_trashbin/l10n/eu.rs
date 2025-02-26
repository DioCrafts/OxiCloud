use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static TRANSLATIONS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("Couldn't delete %s permanently", "Ezin izan da %s betirako ezabatu");
    m.insert("Couldn't restore %s", "Ezin izan da %s berreskuratu");
    m.insert("Error", "Errorea");
    m.insert("restored", "Berrezarrita");
    m.insert("Nothing in here. Your trash bin is empty!", "Ez dago ezer ez. Zure zakarrontzia hutsik dago!");
    m.insert("Name", "Izena");
    m.insert("Restore", "Berrezarri");
    m.insert("Deleted", "Ezabatuta");
    m.insert("Delete", "Ezabatu");
    m.insert("Deleted Files", "Ezabatutako Fitxategiak");
    m
});

pub static PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";