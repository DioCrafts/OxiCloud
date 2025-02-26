use std::collections::HashMap;
use rust_i18n::i18n;

pub fn get_translations() -> HashMap<&'static str, &'static str> {
    let mut translations = HashMap::new();
    translations.insert("Password", "কূটশব্দ");
    translations.insert("%s shared the folder %s with you", "%s আপনার সাথে %s ফোল্ডারটি ভাগাভাগি করেছেন");
    translations.insert("%s shared the file %s with you", "%s আপনার সাথে %s ফাইলটি ভাগাভাগি করেছেন");
    translations.insert("Download", "ডাউনলোড");
    translations.insert("Upload", "আপলোড");
    translations.insert("Cancel upload", "আপলোড বাতিল কর");
    translations.insert("No preview available for", "এর জন্য কোন প্রাকবীক্ষণ সুলভ নয়");
    translations
}

pub fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}

i18n!("bn_BD");