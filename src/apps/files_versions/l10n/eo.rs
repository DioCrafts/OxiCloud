use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Could not revert: %s", "Ne eblas malfari: %s");
        m.insert("Versions", "Versioj");
        m.insert("Failed to revert {file} to revision {timestamp}.", "Malsukcesis returnigo de {file} al la revizio {timestamp}.");
        m.insert("More versions...", "Pli da versioj...");
        m.insert("No other versions available", "Ne disponeblas aliaj versioj");
        m.insert("Restore", "Restaŭri");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}