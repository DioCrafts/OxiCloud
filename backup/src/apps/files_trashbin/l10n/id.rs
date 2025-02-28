use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Couldn't delete %s permanently", "Tidak dapat menghapus permanen %s");
        m.insert("Couldn't restore %s", "Tidak dapat memulihkan %s");
        m.insert("Error", "Galat");
        m.insert("Nothing in here. Your trash bin is empty!", "Tempat sampah anda kosong!");
        m.insert("Name", "Nama");
        m.insert("Restore", "Pulihkan");
        m.insert("Deleted", "Dihapus");
        m.insert("Delete", "Hapus");
        m.insert("Deleted Files", "Berkas yang Dihapus");
        m
    };
    
    pub static ref PLURAL_FORMS: &'static str = "nplurals=1; plural=0;";
}