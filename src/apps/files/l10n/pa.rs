use std::collections::HashMap;
use rust_gettext::prelude::*;

pub fn get_translations() -> HashMap<&'static str, &'static str> {
    let mut translations = HashMap::new();
    translations.insert("Files", "ਫਾਇਲਾਂ");
    translations.insert("Share", "ਸਾਂਝਾ ਕਰੋ");
    translations.insert("Rename", "ਨਾਂ ਬਦਲੋ");
    translations.insert("undo", "ਵਾਪਸ");
    translations.insert("_%n folder_::_%n folders_", "");
    translations.insert("_%n file_::_%n files_", "");
    translations.insert("_Uploading %n file_::_Uploading %n files_", "");
    translations.insert("Error", "ਗਲਤੀ");
    translations.insert("Upload", "ਅੱਪਲੋਡ");
    translations.insert("Cancel upload", "ਅੱਪਲੋਡ ਰੱਦ ਕਰੋ");
    translations.insert("Download", "ਡਾਊਨਲੋਡ");
    translations.insert("Delete", "ਹਟਾਓ");
    
    translations
}

pub fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}

pub fn init_i18n() -> Catalog {
    let mut cat = Catalog::new();
    for (key, value) in get_translations() {
        cat.add_str(key, value);
    }
    cat.set_plural_forms(get_plural_forms());
    cat
}