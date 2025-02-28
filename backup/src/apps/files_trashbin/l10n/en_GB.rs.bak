use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Couldn't delete %s permanently", "Couldn't delete %s permanently");
        m.insert("Couldn't restore %s", "Couldn't restore %s");
        m.insert("Error", "Error");
        m.insert("restored", "restored");
        m.insert("Nothing in here. Your trash bin is empty!", "Nothing in here. Your recycle bin is empty!");
        m.insert("Name", "Name");
        m.insert("Restore", "Restore");
        m.insert("Deleted", "Deleted");
        m.insert("Delete", "Delete");
        m.insert("Deleted Files", "Deleted Files");
        m
    };
}

pub const PLURAL_FORMS: &str = "nplurals=2; plural=(n != 1);";