use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Translations for Bengali (Bangladesh) language
pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut translations = HashMap::new();
    translations.insert("Access granted", "অধিগমনের  অনুমতি প্রদান করা হলো");
    translations.insert("Error configuring Dropbox storage", "Dropbox সংরক্ষণাগার নির্ধারণ করতে সমস্যা ");
    translations.insert("Grant access", "অধিগমনের  অনুমতি প্রদান কর");
    translations.insert("Please provide a valid Dropbox app key and secret.", "দয়া করে সঠিক এবং বৈধ Dropbox app key and secret প্রদান করুন।");
    translations.insert("Error configuring Google Drive storage", "Google Drive সংরক্ষণাগার নির্ধারণ করতে সমস্যা ");
    translations.insert("External Storage", "বাহ্যিক সংরক্ষণাগার");
    translations.insert("Configuration", "কনফিগারেসন");
    translations.insert("Options", "বিকল্পসমূহ");
    translations.insert("Applicable", "প্রযোজ্য");
    translations.insert("None set", "কোনটিই নির্ধারণ করা হয় নি");
    translations.insert("All Users", "সমস্ত ব্যবহারকারী");
    translations.insert("Groups", "গোষ্ঠীসমূহ");
    translations.insert("Users", "ব্যবহারকারী");
    translations.insert("Delete", "মুছে");
    translations.insert("Enable User External Storage", "ব্যবহারকারীর বাহ্যিক সংরক্ষণাগার সক্রিয় কর");
    translations.insert("Allow users to mount their own external storage", "ব্যবহারকারীদেরকে তাদের নিজস্ব বাহ্যিক সংরক্ষনাগার  সাউন্ট করতে অনুমোদন দাও");
    translations.insert("SSL root certificates", "SSL  রুট সনদপত্র");
    translations.insert("Import Root Certificate", "রুট সনদপত্রটি আমদানি করুন");
    translations
});

/// Plural forms definition for Bengali (Bangladesh)
pub const PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";

/// Gets a translation for the given key
pub fn get_translation(key: &str) -> Option<&'static str> {
    TRANSLATIONS.get(key).copied()
}

/// Translates a string based on count for plural forms
pub fn translate_plural(singular: &str, plural: &str, count: i64) -> Option<&'static str> {
    let key = if count != 1 { plural } else { singular };
    get_translation(key)
}