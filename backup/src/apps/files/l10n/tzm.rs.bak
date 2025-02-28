#[derive(Debug, Clone)]
pub struct TzmLocalization {
    pub translations: std::collections::HashMap<String, Vec<String>>,
    pub plural_forms: String,
}

impl TzmLocalization {
    pub fn new() -> Self {
        let mut translations = std::collections::HashMap::new();
        
        translations.insert("_%n folder_::_%n folders_".to_string(), vec!["".to_string(), "".to_string()]);
        translations.insert("_%n file_::_%n files_".to_string(), vec!["".to_string(), "".to_string()]);
        translations.insert("_Uploading %n file_::_Uploading %n files_".to_string(), vec!["".to_string(), "".to_string()]);
        
        Self {
            translations,
            plural_forms: "nplurals=2; plural=(n == 0 || n == 1 || (n > 10 && n < 100) ? 0 : 1;".to_string(),
        }
    }
    
    pub fn get_plural_form(&self, n: i64) -> usize {
        if n == 0 || n == 1 || (n > 10 && n < 100) {
            0
        } else {
            1
        }
    }
}

impl Default for TzmLocalization {
    fn default() -> Self {
        Self::new()
    }
}