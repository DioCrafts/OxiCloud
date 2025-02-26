use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Couldn't delete %s permanently", "Methwyd dileu %s yn barhaol");
        m.insert("Couldn't restore %s", "Methwyd adfer %s");
        m.insert("Error", "Gwall");
        m.insert("Nothing in here. Your trash bin is empty!", "Does dim byd yma. Mae eich bin sbwriel yn wag!");
        m.insert("Name", "Enw");
        m.insert("Restore", "Adfer");
        m.insert("Deleted", "Wedi dileu");
        m.insert("Delete", "Dileu");
        m.insert("Deleted Files", "Ffeiliau Ddilewyd");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=4; plural=(n==1) ? 0 : (n==2) ? 1 : (n != 8 && n != 11) ? 2 : 3;";
}