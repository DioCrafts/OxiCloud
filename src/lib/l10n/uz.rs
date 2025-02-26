use std::collections::HashMap;
use phf::phf_map;

pub static TRANSLATIONS: phf::Map<&'static str, &'static [&'static str]> = phf_map! {
    "_%n minute ago_::_%n minutes ago_" => &[""],
    "_%n hour ago_::_%n hours ago_" => &[""],
    "_%n day go_::_%n days ago_" => &[""],
    "_%n month ago_::_%n months ago_" => &[""],
};

pub static PLURAL_FORMS: &str = "nplurals=1; plural=0;";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translations_exist() {
        assert!(TRANSLATIONS.contains_key("_%n minute ago_::_%n minutes ago_"));
        assert!(TRANSLATIONS.contains_key("_%n hour ago_::_%n hours ago_"));
        assert!(TRANSLATIONS.contains_key("_%n day go_::_%n days ago_"));
        assert!(TRANSLATIONS.contains_key("_%n month ago_::_%n months ago_"));
    }

    #[test]
    fn test_plural_forms() {
        assert_eq!(PLURAL_FORMS, "nplurals=1; plural=0;");
    }
}