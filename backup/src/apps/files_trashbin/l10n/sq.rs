use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static TRANSLATIONS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("Couldn't delete %s permanently", "Nuk munda ta eliminoj përfundimisht %s");
    m.insert("Couldn't restore %s", "Nuk munda ta rivendos %s");
    m.insert("Error", "Veprim i gabuar");
    m.insert("restored", "rivendosur");
    m.insert("Nothing in here. Your trash bin is empty!", "Këtu nuk ka asgjë. Koshi juaj është bosh!");
    m.insert("Name", "Emri");
    m.insert("Restore", "Rivendos");
    m.insert("Deleted", "Eliminuar");
    m.insert("Delete", "Elimino");
    m.insert("Deleted Files", "Skedarë të eliminuar");
    m
});

pub const PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";