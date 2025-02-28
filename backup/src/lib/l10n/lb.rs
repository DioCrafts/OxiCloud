use phf::phf_map;
use std::collections::HashMap;

pub fn get_plural_forms() -> &'static str {
    "nplurals=2; plural=(n != 1);"
}

pub static TRANSLATIONS: phf::Map<&'static str, &'static str> = phf_map! {
    "Help" => "Hëllef",
    "Personal" => "Perséinlech",
    "Settings" => "Astellungen",
    "Users" => "Benotzer",
    "Admin" => "Admin",
    "Unknown filetype" => "Onbekannten Fichier Typ",
    "Invalid image" => "Ongülteg d'Bild",
    "web services under your control" => "Web-Servicer ënnert denger Kontroll",
    "Authentication error" => "Authentifikatioun's Fehler",
    "Files" => "Dateien",
    "Text" => "SMS",
    "seconds ago" => "Sekonnen hir",
    "today" => "haut",
    "yesterday" => "gëschter",
    "last month" => "Läschte Mount",
    "last year" => "Läscht Joer",
    "years ago" => "Joren hier",
};

pub fn get_plural_translations() -> HashMap<&'static str, Vec<&'static str>> {
    let mut map = HashMap::new();
    
    map.insert("_%n minute ago_::_%n minutes ago_", vec!["", "%n Minutten hir"]);
    map.insert("_%n hour ago_::_%n hours ago_", vec!["", ""]);
    map.insert("_%n day go_::_%n days ago_", vec!["", ""]);
    map.insert("_%n month ago_::_%n months ago_", vec!["", ""]);
    
    map
}