use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LdapLocalization {
    translations: HashMap<String, Translation>,
    plural_forms: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Translation {
    Simple(String),
    Plural(Vec<String>),
}

pub fn get_zh_hk_translations() -> LdapLocalization {
    let mut translations = HashMap::new();
    
    translations.insert("Success".to_string(), Translation::Simple("成功".to_string()));
    translations.insert("Error".to_string(), Translation::Simple("錯誤".to_string()));
    translations.insert("_%s group found_::_%s groups found_".to_string(), Translation::Plural(vec!["".to_string()]));
    translations.insert("_%s user found_::_%s users found_".to_string(), Translation::Plural(vec!["".to_string()]));
    translations.insert("Save".to_string(), Translation::Simple("儲存".to_string()));
    translations.insert("Help".to_string(), Translation::Simple("幫助".to_string()));
    translations.insert("Port".to_string(), Translation::Simple("連接埠".to_string()));
    translations.insert("Password".to_string(), Translation::Simple("密碼".to_string()));
    
    LdapLocalization {
        translations,
        plural_forms: "nplurals=1; plural=0;".to_string(),
    }
}