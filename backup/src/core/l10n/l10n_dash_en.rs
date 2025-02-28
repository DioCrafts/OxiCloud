// English localization constants

use std::collections::HashMap;
use once_cell::sync::Lazy;

/// English localization constants
pub static LOCALIZATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("jsdate", "MM d, yy");
    map.insert("date", "%B %e, %Y");
    map.insert("datetime", "%B %e, %Y %H:%M");
    map.insert("time", "%H:%M:%S");
    map.insert("firstday", "0");
    map
});

/// Get the firstday value as integer
pub fn get_first_day() -> u8 {
    0
}