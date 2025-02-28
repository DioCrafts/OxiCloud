use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Couldn't delete %s permanently", "Αδύνατη η μόνιμη διαγραφή του %s");
        m.insert("Couldn't restore %s", "Αδυναμία επαναφοράς %s");
        m.insert("Error", "Σφάλμα");
        m.insert("restored", "έγινε επαναφορά");
        m.insert("Nothing in here. Your trash bin is empty!", "Δεν υπάρχει τίποτα εδώ. Ο κάδος σας είναι άδειος!");
        m.insert("Name", "Όνομα");
        m.insert("Restore", "Επαναφορά");
        m.insert("Deleted", "Διαγράφηκε");
        m.insert("Delete", "Διαγραφή");
        m.insert("Deleted Files", "Διαγραμμένα Αρχεία");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=2; plural=(n != 1);";
}