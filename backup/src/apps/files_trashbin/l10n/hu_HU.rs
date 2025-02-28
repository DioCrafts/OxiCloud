use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Couldn't delete %s permanently", "Nem sikerült %s végleges törlése");
        m.insert("Couldn't restore %s", "Nem sikerült %s visszaállítása");
        m.insert("Error", "Hiba");
        m.insert("restored", "visszaállítva");
        m.insert("Nothing in here. Your trash bin is empty!", "Itt nincs semmi. Az Ön szemetes mappája üres!");
        m.insert("Name", "Név");
        m.insert("Restore", "Visszaállítás");
        m.insert("Deleted", "Törölve");
        m.insert("Delete", "Törlés");
        m.insert("Deleted Files", "Törölt fájlok");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}