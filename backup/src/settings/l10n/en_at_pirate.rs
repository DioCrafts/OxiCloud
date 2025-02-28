use std::collections::HashMap;

pub struct PirateLocalization {
    pub translations: HashMap<&'static str, &'static str>,
    pub plural_forms: &'static str,
}

impl Default for PirateLocalization {
    fn default() -> Self {
        let mut translations = HashMap::new();
        translations.insert("Password", "Passcode");

        PirateLocalization {
            translations,
            plural_forms: "nplurals=2; plural=(n != 1);",
        }
    }
}

pub fn get_pirate_localization() -> PirateLocalization {
    PirateLocalization::default()
}