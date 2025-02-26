use lazy_static::lazy_static;
use std::collections::HashMap;
use unic_langid::LanguageIdentifier;
use fluent_templates::fluent_bundle::FluentValue;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("_%n folder_::_%n folders_", "");
        m.insert("_%n file_::_%n files_", "");
        m.insert("_Uploading %n file_::_Uploading %n files_", "");
        m.insert("Delete", "លុប");
        m
    };
    
    pub static ref PLURAL_FORMS: &'static str = "nplurals=1; plural=0;";
    
    pub static ref KM_LANG_ID: LanguageIdentifier = "km".parse().unwrap();
}

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

pub fn format_with_args(message_id: &str, args: Option<HashMap<&str, FluentValue>>) -> String {
    if let Some(translation) = get_translation(message_id) {
        if translation.is_empty() {
            return message_id.to_string();
        }
        
        // Basic placeholder replacement
        if let Some(args_map) = args {
            let mut result = translation.to_string();
            for (key, value) in args_map {
                let placeholder = format!("{{{}}}", key);
                match value {
                    FluentValue::String(s) => {
                        result = result.replace(&placeholder, s);
                    },
                    FluentValue::Number(n) => {
                        result = result.replace(&placeholder, &n.to_string());
                    },
                    _ => {}
                }
            }
            return result;
        }
        
        return translation.to_string();
    }
    
    message_id.to_string()
}