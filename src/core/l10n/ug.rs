use phf::phf_map;
use rust_gettext::Catalog;

pub static UG_TRANSLATIONS: phf::Map<&'static str, &'static str> = phf_map! {
    "Sunday" => "يەكشەنبە",
    "Monday" => "دۈشەنبە",
    "Tuesday" => "سەيشەنبە",
    "Wednesday" => "چارشەنبە",
    "Thursday" => "پەيشەنبە",
    "Friday" => "جۈمە",
    "Saturday" => "شەنبە",
    "January" => "قەھرىتان",
    "February" => "ھۇت",
    "March" => "نەۋرۇز",
    "April" => "ئۇمۇت",
    "May" => "باھار",
    "June" => "سەپەر",
    "July" => "چىللە",
    "August" => "تومۇز",
    "September" => "مىزان",
    "October" => "ئوغۇز",
    "November" => "ئوغلاق",
    "December" => "كۆنەك",
    "Settings" => "تەڭشەكلەر",
    "_%n minute ago_::_%n minutes ago_" => "",
    "_%n hour ago_::_%n hours ago_" => "",
    "today" => "بۈگۈن",
    "yesterday" => "تۈنۈگۈن",
    "_%n day ago_::_%n days ago_" => "",
    "_%n month ago_::_%n months ago_" => "",
    "Yes" => "ھەئە",
    "No" => "ياق",
    "Ok" => "جەزملە",
    "_{count} file conflict_::_{count} file conflicts_" => "",
    "Cancel" => "ۋاز كەچ",
    "Share" => "ھەمبەھىر",
    "Error" => "خاتالىق",
    "Password" => "ئىم",
    "Send" => "يوللا",
    "group" => "گۇرۇپپا",
    "Unshare" => "ھەمبەھىرلىمە",
    "delete" => "ئۆچۈر",
    "share" => "ھەمبەھىر",
    "Warning" => "ئاگاھلاندۇرۇش",
    "Delete" => "ئۆچۈر",
    "Add" => "قوش",
    "Username" => "ئىشلەتكۈچى ئاتى",
    "New password" => "يېڭى ئىم",
    "Personal" => "شەخسىي",
    "Users" => "ئىشلەتكۈچىلەر",
    "Apps" => "ئەپلەر",
    "Help" => "ياردەم",
    "Security Warning" => "بىخەتەرلىك ئاگاھلاندۇرۇش",
    "Advanced" => "ئالىي",
    "Finish setup" => "تەڭشەك تامام",
    "Log out" => "تىزىمدىن چىق",
};

pub fn get_catalog() -> Catalog {
    let mut catalog = Catalog::new("ug".to_string());
    catalog.set_plural_forms("nplurals=1; plural=0;".to_string());
    
    for (key, value) in UG_TRANSLATIONS.entries() {
        if key.contains("::") && value.is_empty() {
            // Plural forms are represented as key with :: separator
            let parts: Vec<&str> = key.split("::").collect();
            if parts.len() == 2 {
                let singular = parts[0].trim_matches(|c| c == '_');
                let plural = parts[1].trim_matches(|c| c == '_');
                catalog.add_plural(singular.to_string(), plural.to_string(), vec!["".to_string()]);
            }
        } else {
            catalog.add(key.to_string(), value.to_string());
        }
    }
    
    catalog
}

pub fn translate(msgid: &str) -> String {
    UG_TRANSLATIONS.get(msgid).map(|s| s.to_string()).unwrap_or_else(|| msgid.to_string())
}