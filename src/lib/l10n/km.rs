pub struct Km;

impl Km {
    pub fn translations() -> phf::Map<&'static str, &'static [&'static str]> {
        phf::phf_map! {
            "_%n minute ago_::_%n minutes ago_" => &[""],
            "_%n hour ago_::_%n hours ago_" => &[""],
            "_%n day go_::_%n days ago_" => &[""],
            "_%n month ago_::_%n months ago_" => &[""],
        }
    }

    pub fn plural_forms() -> &'static str {
        "nplurals=1; plural=0;"
    }
}