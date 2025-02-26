use std::collections::HashMap;
use rust_gettext::prelude::*;

pub struct Translations {
    translations: HashMap<&'static str, &'static str>,
    plural_forms: &'static str,
}

impl Translations {
    pub fn new() -> Self {
        let mut translations = HashMap::new();
        translations.insert("Error", "දෝෂයක්");
        translations.insert("Name", "නම");
        translations.insert("Delete", "මකා දමන්න");

        Translations {
            translations,
            plural_forms: "nplurals=2; plural=(n != 1);",
        }
    }
}

impl GetText for Translations {
    fn get_text(&self, msgid: &str) -> &str {
        self.translations.get(msgid).copied().unwrap_or(msgid)
    }

    fn plural_forms(&self) -> &str {
        self.plural_forms
    }
}

pub fn initialize() -> Translations {
    Translations::new()
}