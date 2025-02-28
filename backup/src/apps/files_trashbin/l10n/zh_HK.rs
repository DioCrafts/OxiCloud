use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Error", "錯誤");
        m.insert("Name", "名稱");
        m.insert("Delete", "刪除");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=1; plural=0;";
}