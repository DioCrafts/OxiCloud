use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Password", "Cyfrinair");
        m.insert("%s shared the folder %s with you", "Rhannodd %s blygell %s â chi");
        m.insert("%s shared the file %s with you", "Rhannodd %s ffeil %s â chi");
        m.insert("Download", "Llwytho i lawr");
        m.insert("Upload", "Llwytho i fyny");
        m.insert("Cancel upload", "Diddymu llwytho i fyny");
        m.insert("No preview available for", "Does dim rhagolwg ar gael ar gyfer");
        m
    };
}

pub fn plural_forms(n: i64) -> usize {
    if n == 1 {
        0
    } else if n == 2 {
        1
    } else if n != 8 && n != 11 {
        2
    } else {
        3
    }
}

pub const NPLURALS: usize = 4;