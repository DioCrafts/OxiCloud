use once_cell::sync::Lazy;
use std::collections::HashMap;
use unic_langid::LanguageIdentifier;

pub static TRANSLATION_TE: Lazy<(LanguageIdentifier, HashMap<&'static str, &'static str>, &'static str)> = Lazy::new(|| {
    let lang_id: LanguageIdentifier = "te".parse().unwrap();
    
    let mut translations = HashMap::new();
    translations.insert("Password", "సంకేతపదం");
    
    let plural_forms = "nplurals=2; plural=(n != 1);";
    
    (lang_id, translations, plural_forms)
});