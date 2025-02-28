use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("_%n folder_::_%n folders_", vec![""]);
        m.insert("_%n file_::_%n files_", vec![""]);
        m.insert("_Uploading %n file_::_Uploading %n files_", vec![""]);
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=1; plural=0;";
}