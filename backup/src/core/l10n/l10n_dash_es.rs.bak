use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static LOCALIZATIONS: Lazy<HashMap<&'static str, LocalizationValue>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("jsdate", LocalizationValue::Str("d 'de' MM 'de' yy"));
    m.insert("date", LocalizationValue::Str("%e de %B de %Y"));
    m.insert("datetime", LocalizationValue::Str("%e de %B de %Y %H:%M"));
    m.insert("time", LocalizationValue::Str("%H:%M:%S"));
    m.insert("firstday", LocalizationValue::Int(1));
    m
});

#[derive(Debug, Clone)]
pub enum LocalizationValue {
    Str(&'static str),
    Int(i32),
}