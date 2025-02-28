use lazy_static::lazy_static;
use std::collections::HashMap;
use rust_i18n::plural::PluralRules;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, String> = {
        let mut m = HashMap::new();
        m.insert("Share", "Podijeli".to_string());
        m.insert("_%n folder_::_%n folders_", "".to_string());
        m.insert("_%n file_::_%n files_", "".to_string());
        m.insert("_Uploading %n file_::_Uploading %n files_", "".to_string());
        m.insert("Name", "Ime".to_string());
        m.insert("Size", "Veličina".to_string());
        m.insert("Save", "Spasi".to_string());
        m.insert("Folder", "Fasikla".to_string());
        m
    };

    pub static ref PLURAL_FORMS: PluralRules = PluralRules::new(
        3,
        Box::new(|n| {
            let n100 = n % 100;
            let n10 = n % 10;
            
            if n10 == 1 && n100 != 11 {
                0
            } else if n10 >= 2 && n10 <= 4 && (n100 < 10 || n100 >= 20) {
                1
            } else {
                2
            }
        })
    );
}