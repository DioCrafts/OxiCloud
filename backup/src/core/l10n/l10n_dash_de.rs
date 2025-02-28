use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref LOCALIZATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("jsdate", "dd.mm.yy");
        m.insert("date", "%d.%m.%Y");
        m.insert("datetime", "%d.%m.%Y %H:%M:%S");
        m.insert("time", "%H:%M:%S");
        m.insert("firstday", "0");
        m
    };
}