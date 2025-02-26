use std::collections::HashMap;
use rust_i18n::i18n;

// Punjabi translations for the application
pub fn load_pa_translations() -> HashMap<String, String> {
    let mut translations = HashMap::new();
    
    translations.insert("Sunday".to_string(), "ਐਤਵਾਰ".to_string());
    translations.insert("Monday".to_string(), "ਸੋਮਵਾਰ".to_string());
    translations.insert("Tuesday".to_string(), "ਮੰਗਲਵਾਰ".to_string());
    translations.insert("Wednesday".to_string(), "ਬੁੱਧਵਾਰ".to_string());
    translations.insert("Thursday".to_string(), "ਵੀਰਵਾਰ".to_string());
    translations.insert("Friday".to_string(), "ਸ਼ੁੱਕਰਵਾਰ".to_string());
    translations.insert("Saturday".to_string(), "ਸ਼ਨਿੱਚਰਵਾਰ".to_string());
    translations.insert("January".to_string(), "ਜਨਵਰੀ".to_string());
    translations.insert("February".to_string(), "ਫਰਵਰੀ".to_string());
    translations.insert("March".to_string(), "ਮਾਰਚ".to_string());
    translations.insert("April".to_string(), "ਅਪਰੈ".to_string());
    translations.insert("May".to_string(), "ਮਈ".to_string());
    translations.insert("June".to_string(), "ਜੂਨ".to_string());
    translations.insert("July".to_string(), "ਜੁਲਾਈ".to_string());
    translations.insert("August".to_string(), "ਅਗਸਤ".to_string());
    translations.insert("September".to_string(), "ਸਤੰਬ".to_string());
    translations.insert("October".to_string(), "ਅਕਤੂਬਰ".to_string());
    translations.insert("November".to_string(), "ਨਵੰਬ".to_string());
    translations.insert("December".to_string(), "ਦਸੰਬਰ".to_string());
    translations.insert("Settings".to_string(), "ਸੈਟਿੰਗ".to_string());
    translations.insert("seconds ago".to_string(), "ਸਕਿੰਟ ਪਹਿਲਾਂ".to_string());
    translations.insert("today".to_string(), "ਅੱਜ".to_string());
    translations.insert("yesterday".to_string(), "ਕੱਲ੍ਹ".to_string());
    translations.insert("last month".to_string(), "ਪਿਛਲੇ ਮਹੀਨੇ".to_string());
    translations.insert("months ago".to_string(), "ਮਹੀਨੇ ਪਹਿਲਾਂ".to_string());
    translations.insert("last year".to_string(), "ਪਿਛਲੇ ਸਾਲ".to_string());
    translations.insert("years ago".to_string(), "ਸਾਲਾਂ ਪਹਿਲਾਂ".to_string());
    translations.insert("Choose".to_string(), "ਚੁਣੋ".to_string());
    translations.insert("Yes".to_string(), "ਹਾਂ".to_string());
    translations.insert("No".to_string(), "ਨਹੀਂ".to_string());
    translations.insert("Ok".to_string(), "ਠੀਕ ਹੈ".to_string());
    translations.insert("Cancel".to_string(), "ਰੱਦ ਕਰੋ".to_string());
    translations.insert("Share".to_string(), "ਸਾਂਝਾ ਕਰੋ".to_string());
    translations.insert("Error".to_string(), "ਗਲ".to_string());
    translations.insert("Password".to_string(), "ਪਾਸਵਰ".to_string());
    translations.insert("Send".to_string(), "ਭੇਜੋ".to_string());
    translations.insert("Warning".to_string(), "ਚੇਤਾਵਨੀ".to_string());
    translations.insert("Delete".to_string(), "ਹਟਾਓ".to_string());
    translations.insert("Username".to_string(), "ਯੂਜ਼ਰ-ਨਾਂ".to_string());
    translations.insert("Security Warning".to_string(), "ਸੁਰੱਖਿਆ ਚੇਤਾਵਨੀ".to_string());
    
    translations
}

// Plural forms rule for Punjabi
pub fn get_plural_form(n: usize) -> usize {
    if n != 1 { 1 } else { 0 }
}

// Define the plural translations
pub fn get_plural_translations() -> HashMap<String, Vec<String>> {
    let mut plural_translations = HashMap::new();
    
    plural_translations.insert("_%n minute ago_::_%n minutes ago_".to_string(), 
                            vec!["".to_string(), "".to_string()]);
    plural_translations.insert("_%n hour ago_::_%n hours ago_".to_string(), 
                            vec!["".to_string(), "".to_string()]);
    plural_translations.insert("_%n day ago_::_%n days ago_".to_string(), 
                            vec!["".to_string(), "".to_string()]);
    plural_translations.insert("_%n month ago_::_%n months ago_".to_string(), 
                            vec!["".to_string(), "".to_string()]);
    plural_translations.insert("_{count} file conflict_::_{count} file conflicts_".to_string(), 
                            vec!["".to_string(), "".to_string()]);
    
    plural_translations
}

// Register the language in the i18n system
pub fn register_pa_language() {
    i18n!("pa", fallback = "en");
}