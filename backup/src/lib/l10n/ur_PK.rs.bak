lazy_static! {
    pub static ref TRANSLATIONS: phf::Map<&'static str, &'static str> = phf::phf_map! {
        "Help" => "مدد",
        "Personal" => "ذاتی",
        "Settings" => "سیٹینگز",
        "Users" => "یوزرز",
        "Admin" => "ایڈمن",
        "web services under your control" => "آپ کے اختیار میں ویب سروسیز",
    };

    pub static ref PLURAL_TRANSLATIONS: phf::Map<&'static str, [&'static str; 2]> = phf::phf_map! {
        "_%n minute ago_::_%n minutes ago_" => ["", ""],
        "_%n hour ago_::_%n hours ago_" => ["", ""],
        "_%n day go_::_%n days ago_" => ["", ""],
        "_%n month ago_::_%n months ago_" => ["", ""],
    };
}

pub fn plural_forms(n: usize) -> usize {
    if n != 1 { 1 } else { 0 }
}