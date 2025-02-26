lazy_static! {
    pub static ref TRANSLATIONS: phf::Map<&'static str, &'static str> = phf::phf_map! {
        "WebDAV Authentication" => "WebDAV идентификация",
    };
    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}