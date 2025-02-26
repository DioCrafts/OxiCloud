use std::collections::HashMap;
use fluent_templates::fluent_bundle::FluentValue;
use unic_langid::LanguageIdentifier;

/// Localizations for Tamil (Sri Lanka)
pub fn ta_lk_translations() -> HashMap<String, String> {
    let mut translations = HashMap::new();
    
    translations.insert("Help".to_string(), "உதவி".to_string());
    translations.insert("Personal".to_string(), "தனிப்பட்ட".to_string());
    translations.insert("Settings".to_string(), "அமைப்புகள்".to_string());
    translations.insert("Users".to_string(), "பயனாளர்".to_string());
    translations.insert("Admin".to_string(), "நிர்வாகம்".to_string());
    translations.insert("web services under your control".to_string(), "வலைய சேவைகள் உங்களுடைய கட்டுப்பாட்டின் கீழ் உள்ளது".to_string());
    translations.insert("ZIP download is turned off.".to_string(), "வீசொலிப் பூட்டு பதிவிறக்கம் நிறுத்தப்பட்டுள்ளது.".to_string());
    translations.insert("Files need to be downloaded one by one.".to_string(), "கோப்புகள்ஒன்றன் பின் ஒன்றாக பதிவிறக்கப்படவேண்டும்.".to_string());
    translations.insert("Back to Files".to_string(), "கோப்புகளுக்கு செல்க".to_string());
    translations.insert("Selected files too large to generate zip file.".to_string(), "வீ சொலிக் கோப்புகளை உருவாக்குவதற்கு தெரிவுசெய்யப்பட்ட கோப்புகள் மிகப்பெரியவை".to_string());
    translations.insert("Application is not enabled".to_string(), "செயலி இயலுமைப்படுத்தப்படவில்லை".to_string());
    translations.insert("Authentication error".to_string(), "அத்தாட்சிப்படுத்தலில் வழு".to_string());
    translations.insert("Token expired. Please reload page.".to_string(), "அடையாளவில்லை காலாவதியாகிவிட்டது. தயவுசெய்து பக்கத்தை மீள் ஏற்றுக.".to_string());
    translations.insert("Files".to_string(), "கோப்புகள்".to_string());
    translations.insert("Text".to_string(), "உரை".to_string());
    translations.insert("Images".to_string(), "படங்கள்".to_string());
    translations.insert("Could not find category \"%s\"".to_string(), "பிரிவு \"%s\" ஐ கண்டுப்பிடிக்க முடியவில்லை".to_string());
    translations.insert("seconds ago".to_string(), "செக்கன்களுக்கு முன்".to_string());
    translations.insert("_%n minute ago_::_%n minutes ago_".to_string(), "".to_string());
    translations.insert("_%n hour ago_::_%n hours ago_".to_string(), "".to_string());
    translations.insert("today".to_string(), "இன்று".to_string());
    translations.insert("yesterday".to_string(), "நேற்று".to_string());
    translations.insert("_%n day go_::_%n days ago_".to_string(), "".to_string());
    translations.insert("last month".to_string(), "கடந்த மாதம்".to_string());
    translations.insert("_%n month ago_::_%n months ago_".to_string(), "".to_string());
    translations.insert("last year".to_string(), "கடந்த வருடம்".to_string());
    translations.insert("years ago".to_string(), "வருடங்களுக்கு முன்".to_string());

    translations
}

/// Returns the plural forms rule for Tamil (Sri Lanka)
pub fn ta_lk_plural_forms() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}

pub fn get_language_id() -> LanguageIdentifier {
    "ta-LK".parse().expect("Failed to parse language identifier")
}

pub fn get_plural_rule(n: usize) -> usize {
    if n != 1 { 1 } else { 0 }
}

pub fn format_plural(message_id: &str, n: usize) -> String {
    let translations = ta_lk_translations();
    let plural_key = match message_id {
        "_%n minute ago_::_%n minutes ago_" => {
            if n == 1 { "_%n minute ago_" } else { "_%n minutes ago_" }
        },
        "_%n hour ago_::_%n hours ago_" => {
            if n == 1 { "_%n hour ago_" } else { "_%n hours ago_" }
        },
        "_%n day go_::_%n days ago_" => {
            if n == 1 { "_%n day go_" } else { "_%n days ago_" }
        },
        "_%n month ago_::_%n months ago_" => {
            if n == 1 { "_%n month ago_" } else { "_%n months ago_" }
        },
        _ => message_id,
    };
    
    // Get translation or fall back to the original message
    translations.get(plural_key).cloned().unwrap_or_else(|| plural_key.to_string())
}