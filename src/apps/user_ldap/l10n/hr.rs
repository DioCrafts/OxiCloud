use lazy_static::lazy_static;
use std::collections::HashMap;
use rust_i18n::t;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("Error", "Greška");
        map.insert("Save", "Snimi");
        map.insert("Help", "Pomoć");
        map.insert("Password", "Lozinka");
        map.insert("Back", "Natrag");
        map
    };

    pub static ref PLURAL_FORMS: &'static str = 
        "nplurals=3; plural=n%10==1 && n%100!=11 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 || n%100>=20) ? 1 : 2;";
}

pub fn get_plural_form(n: i64) -> usize {
    if n % 10 == 1 && n % 100 != 11 {
        0
    } else if n % 10 >= 2 && n % 10 <= 4 && (n % 100 < 10 || n % 100 >= 20) {
        1
    } else {
        2
    }
}

pub fn translate_group_found(count: i64) -> String {
    match get_plural_form(count) {
        0 => format!("{} group found", count),
        1 => format!("{} groups found", count),
        _ => format!("{} groups found", count),
    }
}

pub fn translate_user_found(count: i64) -> String {
    match get_plural_form(count) {
        0 => format!("{} user found", count),
        1 => format!("{} users found", count),
        _ => format!("{} users found", count),
    }
}