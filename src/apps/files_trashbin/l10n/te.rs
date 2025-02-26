use std::collections::HashMap;
use rust_i18n::t;

// Telugu language translations
pub fn register_te_translations() -> (HashMap<&'static str, &'static str>, &'static str) {
    let mut translations = HashMap::new();
    
    translations.insert("Error", "పొరపాటు");
    translations.insert("Name", "పేరు");
    translations.insert("Delete", "తొలగించు");
    
    let plural_forms = "nplurals=2; plural=(n != 1);";
    
    (translations, plural_forms)
}