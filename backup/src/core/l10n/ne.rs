pub fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}

pub fn get_translations() -> phf::Map<&'static str, [&'static str; 2]> {
    phf::phf_map! {
        "_%n minute ago_::_%n minutes ago_" => ["", ""],
        "_%n hour ago_::_%n hours ago_" => ["", ""],
        "_%n day ago_::_%n days ago_" => ["", ""],
        "_%n month ago_::_%n months ago_" => ["", ""],
        "_{count} file conflict_::_{count} file conflicts_" => ["", ""],
    }
}