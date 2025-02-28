use rust_i18n::t;
use std::collections::HashMap;
use once_cell::sync::Lazy;

/// Croatian (hr) translations
pub static HR_TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut translations = HashMap::new();
    translations.insert("Help", "Pomoć");
    translations.insert("Personal", "Osobno");
    translations.insert("Settings", "Postavke");
    translations.insert("Users", "Korisnici");
    translations.insert("Admin", "Administrator");
    translations.insert("web services under your control", "web usluge pod vašom kontrolom");
    translations.insert("Authentication error", "Greška kod autorizacije");
    translations.insert("Files", "Datoteke");
    translations.insert("Text", "Tekst");
    translations.insert("seconds ago", "sekundi prije");
    translations.insert("today", "danas");
    translations.insert("yesterday", "jučer");
    translations.insert("last month", "prošli mjesec");
    translations.insert("last year", "prošlu godinu");
    translations.insert("years ago", "godina");
    translations
});

/// Croatian (hr) plural forms for different time units
pub static HR_PLURAL_FORMS: &str = "nplurals=3; plural=n%10==1 && n%100!=11 ? 0 : n%10>=2 && n%10<=4 && (n%100<10 || n%100>=20) ? 1 : 2;";

/// Get Croatian plural form based on n
pub fn get_hr_plural_form(n: usize) -> usize {
    let n = n as i64;
    if n % 10 == 1 && n % 100 != 11 {
        0
    } else if n % 10 >= 2 && n % 10 <= 4 && (n % 100 < 10 || n % 100 >= 20) {
        1
    } else {
        2
    }
}

/// Croatian (hr) plural forms for minutes
pub fn get_hr_minute_plural(n: usize) -> &'static str {
    match get_hr_plural_form(n) {
        0 => "",
        1 => "",
        _ => "",
    }
}

/// Croatian (hr) plural forms for hours
pub fn get_hr_hour_plural(n: usize) -> &'static str {
    match get_hr_plural_form(n) {
        0 => "",
        1 => "",
        _ => "",
    }
}

/// Croatian (hr) plural forms for days
pub fn get_hr_day_plural(n: usize) -> &'static str {
    match get_hr_plural_form(n) {
        0 => "",
        1 => "",
        _ => "",
    }
}

/// Croatian (hr) plural forms for months
pub fn get_hr_month_plural(n: usize) -> &'static str {
    match get_hr_plural_form(n) {
        0 => "",
        1 => "",
        _ => "",
    }
}