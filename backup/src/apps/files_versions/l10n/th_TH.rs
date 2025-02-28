lazy_static! {
    pub static ref TRANSLATIONS: phf::Map<&'static str, &'static str> = phf::phf_map! {
        "Versions" => "รุ่น",
        "Restore" => "คืนค่า",
    };
}

pub const PLURAL_FORMS: &str = "nplurals=1; plural=0;";