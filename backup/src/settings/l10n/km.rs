use phf::{phf_map, Map};
use once_cell::sync::Lazy;

pub const PLURAL_FORMS: &str = "nplurals=1; plural=0;";

pub static TRANSLATIONS: Lazy<Map<&'static str, &'static str>> = Lazy::new(|| {
    phf_map! {
        "Delete" => "លុប",
    }
});