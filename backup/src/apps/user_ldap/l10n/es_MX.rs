use std::collections::HashMap;
use rust_gettext::plural::PluralForms;

struct L10n {
    translations: HashMap<String, Vec<String>>,
    plural_forms: PluralForms,
}

impl L10n {
    pub fn new() -> Self {
        let mut translations = HashMap::new();
        
        translations.insert("_%s group found_::_%s groups found_".to_string(), 
                           vec!["".to_string(), "".to_string()]);
        translations.insert("_%s user found_::_%s users found_".to_string(), 
                           vec!["".to_string(), "".to_string()]);
        
        L10n {
            translations,
            plural_forms: PluralForms::from("nplurals=2; plural=(n != 1);").unwrap(),
        }
    }
    
    pub fn get_translation(&self, key: &str, count: usize) -> Option<&String> {
        let forms = self.translations.get(key)?;
        let form_index = self.plural_forms.get_plural_form(count as u64) as usize;
        forms.get(form_index)
    }
}

pub fn create_es_mx_l10n() -> L10n {
    L10n::new()
}