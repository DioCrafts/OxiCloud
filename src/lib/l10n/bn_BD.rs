use once_cell::sync::Lazy;
use std::collections::HashMap;
use fluent::{FluentBundle, FluentResource};
use std::borrow::Cow;
use unic_langid::LanguageIdentifier;

pub struct BnBdTranslations;

impl BnBdTranslations {
    pub fn get_translations() -> HashMap<&'static str, &'static str> {
        let mut translations = HashMap::new();
        
        translations.insert("Help", "সহায়িকা");
        translations.insert("Personal", "ব্যক্তিগত");
        translations.insert("Settings", "নিয়ামকসমূহ");
        translations.insert("Users", "ব্যবহারকারী");
        translations.insert("Admin", "প্রশাসন");
        translations.insert("web services under your control", "ওয়েব সার্ভিস আপনার হাতের মুঠোয়");
        translations.insert("ZIP download is turned off.", "ZIP ডাউনলোড বন্ধ করা আছে।");
        translations.insert("Files need to be downloaded one by one.", "ফাইলগুলো একে একে ডাউনলোড করা আবশ্যক।");
        translations.insert("Back to Files", "ফাইলে ফিরে চল");
        translations.insert("Selected files too large to generate zip file.", "নির্বাচিত ফাইলগুলো এতই বৃহৎ যে জিপ ফাইল তৈরী করা সম্ভব নয়।");
        translations.insert("Application is not enabled", "অ্যাপ্লিকেসনটি সক্রিয় নয়");
        translations.insert("Authentication error", "অনুমোদন ঘটিত সমস্যা");
        translations.insert("Token expired. Please reload page.", "টোকেন মেয়াদোত্তীর্ণ। দয়া করে পৃষ্ঠাটি পূনরায় লোড করুন।");
        translations.insert("Files", "ফাইল");
        translations.insert("Text", "টেক্সট");
        translations.insert("seconds ago", "সেকেন্ড পূর্বে");
        translations.insert("today", "আজ");
        translations.insert("yesterday", "গতকাল");
        translations.insert("last month", "গত মাস");
        translations.insert("last year", "গত বছর");
        translations.insert("years ago", "বছর পূর্বে");
        
        translations
    }

    pub fn get_plural_forms() -> &'static str {
        "nplurals=2; plural=(n != 1);"
    }

    pub fn get_plural_translation(key: &str, count: i32) -> Option<String> {
        match key {
            "_%n minute ago_::_%n minutes ago_" => {
                if count == 1 {
                    Some(format!("{} minute ago", count))
                } else {
                    Some(format!("{} minutes ago", count))
                }
            },
            "_%n hour ago_::_%n hours ago_" => {
                if count == 1 {
                    Some(format!("{} hour ago", count))
                } else {
                    Some(format!("{} hours ago", count))
                }
            },
            "_%n day go_::_%n days ago_" => {
                if count == 1 {
                    Some(format!("{} day ago", count))
                } else {
                    Some(format!("{} days ago", count))
                }
            },
            "_%n month ago_::_%n months ago_" => {
                if count == 1 {
                    Some(format!("{} month ago", count))
                } else {
                    Some(format!("{} months ago", count))
                }
            },
            _ => None,
        }
    }
}

// Setup the lazy static bundle for more idiomatic Rust translation handling
pub static BN_BD_BUNDLE: Lazy<FluentBundle<FluentResource>> = Lazy::new(|| {
    let mut bundle = FluentBundle::new(vec![
        "bn-BD".parse::<LanguageIdentifier>().unwrap()
    ]);
    
    // Add translations to the bundle
    // In a real implementation, these would come from Fluent (.ftl) files
    
    bundle
});