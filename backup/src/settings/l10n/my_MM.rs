use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Authentication error", "ခွင့်ပြုချက်မအောင်မြင်");
        m.insert("Invalid request", "တောင်းဆိုချက်မမှန်ကန်ပါ");
        m.insert("Security Warning", "လုံခြုံရေးသတိပေးချက်");
        m.insert("Password", "စကားဝှက်");
        m.insert("New password", "စကားဝှက်အသစ်");
        m.insert("Username", "သုံးစွဲသူအမည်");
        m
    };
    
    pub static ref PLURAL_FORMS: &'static str = "nplurals=1; plural=0;";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translations() {
        assert_eq!(TRANSLATIONS.get("Password"), Some(&"စကားဝှက်"));
    }
}