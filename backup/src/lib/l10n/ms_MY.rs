use rust_fluent::fluent_bundle::FluentValue;
use std::collections::HashMap;

pub struct MsMyLocalization;

impl MsMyLocalization {
    pub fn get_translations() -> HashMap<&'static str, &'static str> {
        let mut translations = HashMap::new();
        
        translations.insert("Help", "Bantuan");
        translations.insert("Personal", "Peribadi");
        translations.insert("Settings", "Tetapan");
        translations.insert("Users", "Pengguna");
        translations.insert("Admin", "Admin");
        translations.insert("web services under your control", "Perkhidmatan web di bawah kawalan anda");
        translations.insert("Authentication error", "Ralat pengesahan");
        translations.insert("Files", "Fail-fail");
        translations.insert("Text", "Teks");
        
        translations
    }

    pub fn get_plural_form() -> &'static str {
        "nplurals=1; plural=0;"
    }

    pub fn get_plural_translations() -> HashMap<&'static str, Vec<&'static str>> {
        let mut plural_translations = HashMap::new();
        
        plural_translations.insert("_%n minute ago_::_%n minutes ago_", vec![""]);
        plural_translations.insert("_%n hour ago_::_%n hours ago_", vec![""]);
        plural_translations.insert("_%n day go_::_%n days ago_", vec![""]);
        plural_translations.insert("_%n month ago_::_%n months ago_", vec![""]);
        
        plural_translations
    }
}

impl Default for MsMyLocalization {
    fn default() -> Self {
        MsMyLocalization
    }
}