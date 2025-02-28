use std::collections::HashMap;
use unic_langid::LanguageIdentifier;
use rust_fluent::FluentValue;
use std::str::FromStr;

pub struct SwKeLanguagePack {
    translations: HashMap<String, Vec<String>>,
    plural_forms: String,
    language_id: LanguageIdentifier,
}

impl Default for SwKeLanguagePack {
    fn default() -> Self {
        let mut translations = HashMap::new();
        
        translations.insert("_%n minute ago_::_%n minutes ago_".to_string(), vec!["".to_string(), "".to_string()]);
        translations.insert("_%n hour ago_::_%n hours ago_".to_string(), vec!["".to_string(), "".to_string()]);
        translations.insert("_%n day ago_::_%n days ago_".to_string(), vec!["".to_string(), "".to_string()]);
        translations.insert("_%n month ago_::_%n months ago_".to_string(), vec!["".to_string(), "".to_string()]);
        translations.insert("_{count} file conflict_::_{count} file conflicts_".to_string(), vec!["".to_string(), "".to_string()]);
        
        Self {
            translations,
            plural_forms: "nplurals=2; plural=(n != 1);".to_string(),
            language_id: LanguageIdentifier::from_str("sw-KE").unwrap(),
        }
    }
}

impl SwKeLanguagePack {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn get_translation(&self, key: &str, count: usize) -> Option<&str> {
        let forms = self.translations.get(key)?;
        let plural_index = if count != 1 { 1 } else { 0 };
        forms.get(plural_index).map(|s| s.as_str())
    }
    
    pub fn get_language_id(&self) -> &LanguageIdentifier {
        &self.language_id
    }
    
    pub fn get_plural_forms(&self) -> &str {
        &self.plural_forms
    }
}