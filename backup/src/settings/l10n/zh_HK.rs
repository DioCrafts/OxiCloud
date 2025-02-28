use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Error", "錯誤");
        m.insert("Groups", "群組");
        m.insert("Delete", "刪除");
        m.insert("Password", "密碼");
        m.insert("New password", "新密碼");
        m.insert("Email", "電郵");
        m.insert("Encryption", "加密");
        m.insert("Username", "用戶名稱");
        m
    };
    
    pub static ref PLURAL_FORMS: &'static str = "nplurals=1; plural=0;";
}