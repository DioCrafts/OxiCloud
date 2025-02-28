use rust_i18n::locale::PluralForms;
use std::collections::HashMap;

pub fn init_translations() -> (HashMap<String, Vec<String>>, PluralForms) {
    let mut translations = HashMap::new();
    
    translations.insert("_%n folder_::_%n folders_".to_string(), vec!["".to_string(), "".to_string()]);
    translations.insert("_%n file_::_%n files_".to_string(), vec!["".to_string(), "".to_string()]);
    
    let plural_forms = PluralForms::new("nplurals=2; plural=(n != 1);");
    
    (translations, plural_forms)
}