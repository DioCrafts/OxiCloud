use lazy_static::lazy_static;
use std::collections::HashMap;
use rust_i18n::locale::pluralrules::PluralCategory;
use rust_i18n::locale::PluralRules;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Help", "幫助");
        m.insert("Personal", "個人");
        m.insert("Settings", "設定");
        m.insert("Users", "用戶");
        m.insert("Admin", "管理");
        m.insert("Files", "文件");
        m.insert("Text", "文字");
        m.insert("_%n minute ago_::_%n minutes ago_", "");
        m.insert("_%n hour ago_::_%n hours ago_", "");
        m.insert("today", "今日");
        m.insert("yesterday", "昨日");
        m.insert("_%n day go_::_%n days ago_", "");
        m.insert("last month", "前一月");
        m.insert("_%n month ago_::_%n months ago_", "");
        m
    };

    pub static ref PLURAL_RULES: PluralRules = {
        // Chinese has a single plural form
        PluralRules {
            nplurals: 1,
            plural_fn: |_n| PluralCategory::Other,
        }
    };
}

pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

pub fn get_plural_translation(key: &str, count: usize) -> Option<&'static str> {
    // Since Chinese has only one plural form, we ignore count
    get_translation(key)
}