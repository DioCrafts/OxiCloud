use std::collections::HashMap;

fn get_translations() -> HashMap<String, Vec<String>> {
    let mut translations = HashMap::new();
    translations.insert("_%n folder_::_%n folders_".to_string(), vec!["".to_string(), "".to_string()]);
    translations.insert("_%n file_::_%n files_".to_string(), vec!["".to_string(), "".to_string()]);
    translations
}

fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}