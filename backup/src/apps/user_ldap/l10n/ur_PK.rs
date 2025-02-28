use phf::phf_map;
use rust_i18n::t;

// Translation strings mapping
pub static TRANSLATIONS: phf::Map<&'static str, &'static str> = phf_map! {
    "Error" => "ایرر",
    "Help" => "مدد",
    "Password" => "پاسورڈ",
};

// Plural forms
pub static PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";

// Plural translations with separate functions
pub fn groups_found(count: usize) -> String {
    if count == 1 {
        format!("{} group found", count)
    } else {
        format!("{} groups found", count)
    }
}

pub fn users_found(count: usize) -> String {
    if count == 1 {
        format!("{} user found", count)
    } else {
        format!("{} users found", count)
    }
}

// Initialization function for the localization
pub fn init() {
    // Initialize localization system if needed
}