use lazy_static::lazy_static;
use std::collections::HashMap;
use rust_i18n::t;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Help", "Hjálp");
        m.insert("Personal", "Um mig");
        m.insert("Settings", "Stillingar");
        m.insert("Users", "Notendur");
        m.insert("Admin", "Stjórnun");
        m.insert("web services under your control", "vefþjónusta undir þinni stjórn");
        m.insert("ZIP download is turned off.", "Slökkt á ZIP niðurhali.");
        m.insert("Files need to be downloaded one by one.", "Skrárnar verður að sækja eina og eina");
        m.insert("Back to Files", "Aftur í skrár");
        m.insert("Selected files too large to generate zip file.", "Valdar skrár eru of stórar til að búa til ZIP skrá.");
        m.insert("Application is not enabled", "Forrit ekki virkt");
        m.insert("Authentication error", "Villa við auðkenningu");
        m.insert("Token expired. Please reload page.", "Auðkenning útrunnin. Vinsamlegast skráðu þig aftur inn.");
        m.insert("Files", "Skrár");
        m.insert("Text", "Texti");
        m.insert("Images", "Myndir");
        m.insert("Could not find category \"%s\"", "Fann ekki flokkinn \"%s\"");
        m.insert("seconds ago", "sek.");
        m.insert("today", "í dag");
        m.insert("yesterday", "í gær");
        m.insert("last month", "síðasta mánuði");
        m.insert("last year", "síðasta ári");
        m.insert("years ago", "einhverjum árum");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";

    pub static ref PLURAL_TRANSLATIONS: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("_%n minute ago_::_%n minutes ago_", vec!["", ""]);
        m.insert("_%n hour ago_::_%n hours ago_", vec!["", ""]);
        m.insert("_%n day go_::_%n days ago_", vec!["", ""]);
        m.insert("_%n month ago_::_%n months ago_", vec!["", ""]);
        m
    };
}

pub fn translate(key: &str) -> &'static str {
    TRANSLATIONS.get(key).unwrap_or(&key)
}

pub fn translate_plural(key: &str, count: i64) -> &'static str {
    if let Some(translations) = PLURAL_TRANSLATIONS.get(key) {
        let plural_index = if count != 1 { 1 } else { 0 };
        return translations.get(plural_index).unwrap_or(&"");
    }
    ""
}