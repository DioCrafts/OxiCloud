use std::collections::HashMap;
use rust_fluent::types::FluentNumber;

pub fn get_ug_translations() -> HashMap<&'static str, &'static str> {
    let mut translations = HashMap::new();
    
    translations.insert("Help", "ياردەم");
    translations.insert("Personal", "شەخسىي");
    translations.insert("Settings", "تەڭشەكلەر");
    translations.insert("Users", "ئىشلەتكۈچىلەر");
    translations.insert("Authentication error", "سالاھىيەت دەلىللەش خاتالىقى");
    translations.insert("Files", "ھۆججەتلەر");
    translations.insert("Text", "قىسقا ئۇچۇر");
    translations.insert("Images", "سۈرەتلەر");
    translations.insert("Your web server is not yet properly setup to allow files synchronization because the WebDAV interface seems to be broken.", "سىزنىڭ تور مۇلازىمېتىرىڭىز ھۆججەت قەدەمداشلاشقا يول قويىدىغان قىلىپ توغرا تەڭشەلمەپتۇ، چۈنكى WebDAV نىڭ ئېغىزى بۇزۇلغاندەك تۇرىدۇ.");
    translations.insert("today", "بۈگۈن");
    translations.insert("yesterday", "تۈنۈگۈن");
    
    translations
}

pub fn get_plural_forms() -> &'static str {
    "nplurals=1; plural=0;"
}

pub fn format_minutes_ago(n: FluentNumber) -> String {
    // Only one plural form in Uyghur
    String::new()
}

pub fn format_hours_ago(n: FluentNumber) -> String {
    // Only one plural form in Uyghur
    String::new()
}

pub fn format_days_ago(n: FluentNumber) -> String {
    // Only one plural form in Uyghur
    String::new()
}

pub fn format_months_ago(n: FluentNumber) -> String {
    // Only one plural form in Uyghur
    String::new()
}