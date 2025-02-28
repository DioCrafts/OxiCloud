use std::collections::HashMap;
use lazy_static::lazy_static;
use i18n_plural::plural_rules::PluralRule;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("_%n folder_::_%n folders_", vec!["", ""]);
        m.insert("_%n file_::_%n files_", vec!["", ""]);
        m.insert("_Uploading %n file_::_Uploading %n files_", vec!["", ""]);
        m
    };
}

pub const PLURAL_FORMS: PluralRule = |n| if n != 1 { 1 } else { 0 };