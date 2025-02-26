use std::collections::HashMap;
use lazy_static::lazy_static;
use rust_i18n::locale::plurals::{PluralCategory, PluralRules};

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Password", "මුර පදය");
        m.insert("%s shared the folder %s with you", "%s ඔබව %s ෆෝල්ඩරයට හවුල් කරගත්තේය");
        m.insert("%s shared the file %s with you", "%s ඔබ සමඟ %s ගොනුව බෙදාහදාගත්තේය");
        m.insert("Download", "බාන්න");
        m.insert("Upload", "උඩුගත කරන්න");
        m.insert("Cancel upload", "උඩුගත කිරීම අත් හරින්න");
        m.insert("No preview available for", "පූර්වදර්ශනයක් නොමැත");
        m
    };

    pub static ref PLURAL_RULES: PluralRules = PluralRules {
        nplurals: 2,
        plural_fn: |n| if n != 1 { PluralCategory::Other } else { PluralCategory::One },
    };
}

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

pub fn get_plural_rules() -> &'static PluralRules {
    &PLURAL_RULES
}