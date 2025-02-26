use std::collections::HashMap;
use rust_i18n::i18n;

// Definición de las traducciones para af_ZA
pub fn register_translations() -> HashMap<String, Vec<String>> {
    let mut translations = HashMap::new();
    
    translations.insert(String::from("_%n folder_::_%n folders_"), vec![String::from(""), String::from("")]);
    translations.insert(String::from("_%n file_::_%n files_"), vec![String::from(""), String::from("")]);
    
    translations
}

// Definición de la forma plural para af_ZA
pub fn get_plural_form() -> String {
    String::from("nplurals=2; plural=(n != 1);")
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_register_translations() {
        let translations = register_translations();
        assert_eq!(translations.len(), 2);
        assert!(translations.contains_key("_%n folder_::_%n folders_"));
        assert!(translations.contains_key("_%n file_::_%n files_"));
    }
    
    #[test]
    fn test_get_plural_form() {
        assert_eq!(get_plural_form(), "nplurals=2; plural=(n != 1);");
    }
}