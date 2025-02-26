use rust_i18n::t;

lazy_static! {
    static ref TRANSLATIONS: phf::Map<&'static str, (&'static str, &'static str)> = phf::phf_map! {
        "_%n minute ago_::_%n minutes ago_" => ("", ""),
        "_%n hour ago_::_%n hours ago_" => ("", ""),
        "_%n day go_::_%n days ago_" => ("", ""),
        "_%n month ago_::_%n months ago_" => ("", ""),
    };
}

pub const PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";

pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    rust_i18n::set_locale("hy");
    Ok(())
}