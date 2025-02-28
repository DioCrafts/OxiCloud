use rust_i18n::t;

// Translations for Afrikaans (South Africa)
pub fn register_translations() {
    t!("_%n folder_::_%n folders_", "");
    t!("_%n folder_::_%n folders_", "", plural: 1);
    
    t!("_%n file_::_%n files_", "");
    t!("_%n file_::_%n files_", "", plural: 1);
    
    t!("_Uploading %n file_::_Uploading %n files_", "");
    t!("_Uploading %n file_::_Uploading %n files_", "", plural: 1);
}

// Plural forms: nplurals=2; plural=(n != 1);
pub fn get_plural_form(n: usize) -> usize {
    if n != 1 { 1 } else { 0 }
}